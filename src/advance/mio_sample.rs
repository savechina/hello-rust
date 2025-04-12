use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::str;

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

// Some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn mio_simple() -> Result<(), Box<dyn Error>> {
    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the server socket.
    let addr = "127.0.0.1:13265".parse()?;
    let mut server = TcpListener::bind(addr)?;

    // Start listening for incoming connections.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    // Setup the client socket.
    let mut client = TcpStream::connect(addr)?;
    // Register the socket.
    poll.registry()
        .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    // Start an event loop.
    loop {
        // Poll Mio for events, blocking until we get an event.
        poll.poll(&mut events, None)?;

        // Process each event.
        for event in events.iter() {
            // We can use the token we previously provided to `register` to
            // determine for which socket the event is.
            match event.token() {
                SERVER => {
                    // If this is an event for the server, it means a connection
                    // is ready to be accepted.
                    //
                    // Accept the connection and drop it immediately. This will
                    // close the socket and notify the client of the EOF.
                    let connection = server.accept();

                    if (connection.is_ok()) {
                        println!("Accepted connection");
                    } else {
                        println!("Failed to accept connection");
                    }

                    // We can use the `?` operator to propagate errors.
                    if event.is_readable() {
                        // We can (likely) read from the socket without blocking.
                        //
                        // This is where we would normally read from the socket.
                        // For this example, we will just drop the connection.
                        let mut buf = String::new();
                        if let Ok((mut socket, _)) = connection {
                            // Read from the socket.
                            socket.read_to_string(&mut buf);
                            println!("Read from connection: {}", buf);

                            // Drop the socket to close the connection.
                            drop(socket);
                            println!("Dropped connection");
                        }
                        // println!("Read from connection: {}", buf);
                    }
                }

                CLIENT => {
                    if event.is_writable() {
                        // We can (likely) write to the socket without blocking.
                        //

                        client.write(b"hello world")?;
                    }

                    if event.is_readable() {
                        // We can (likely) read from the socket without blocking.
                    }

                    // Since the server just shuts down the connection, let's
                    // just exit from our event loop.

                    // return Ok(());
                }
                // We don't expect any events with tokens other than those we provided.
                _ => unreachable!(),
            }
        }
    }
}

// Server code
fn run_server(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = address.parse()?;
    let mut listener = TcpListener::bind(addr)?;
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let mut connections = HashMap::new();
    let mut next_client_id = 2;

    // Register the server socket with the poll
    poll.registry()
        .register(&mut listener, SERVER, Interest::READABLE)?;

    println!("Server listening on {}", address);

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    // Accept a new connection
                    match listener.accept() {
                        Ok((mut stream, addr)) => {
                            println!("Accepted connection from {}", addr);
                            let token = Token(next_client_id);
                            connections.insert(token, stream);
                            poll.registry().register(
                                connections.get_mut(&token).unwrap(),
                                token,
                                Interest::READABLE | Interest::WRITABLE,
                            )?;
                            next_client_id += 1;
                        }
                        Err(e) => eprintln!("Error accepting connection: {}", e),
                    }
                }
                token => {
                    if let Some(stream) = connections.get_mut(&token) {
                        let mut buffer = [0; 1024];
                        if event.is_readable() {
                            match stream.read(&mut buffer) {
                                Ok(0) => {
                                    println!("Client {} disconnected", token.0);
                                    connections.remove(&token);
                                }
                                Ok(n) => {
                                    let received_data = &buffer[0..n];
                                    if let Ok(s) = str::from_utf8(received_data) {
                                        println!("Received from client {}: {}", token.0, s.trim());
                                        // Echo back the data
                                        if event.is_writable() {
                                            match stream.write_all(received_data) {
                                                Ok(_) => {
                                                    println!("Echoed back to client {}", token.0)
                                                }
                                                Err(e) => eprintln!(
                                                    "Error writing to client {}: {}",
                                                    token.0, e
                                                ),
                                            }
                                        }
                                    } else {
                                        eprintln!("Received non-UTF8 data from client {}", token.0);
                                    }
                                }
                                Err(e) => eprintln!("Error reading from client {}: {}", token.0, e),
                            }
                        } else if event.is_writable() {
                            // In a real application, you might have data buffered to write here
                            stream.write_all("Hello from server!\n".as_bytes())?;
                            println!("Sent message to client {}", token.0);
                        }
                    }
                }
            }
        }
    }
}

// Client code
fn run_client(address: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = address.parse()?;
    let mut stream = TcpStream::connect(addr)?;
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1);

    // Register the client socket with the poll
    poll.registry()
        .register(&mut stream, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    println!("Connecting to {}", address);

    // Wait for the connection to be established and writable
    poll.poll(&mut events, None)?;

    if let Some(event) = events.iter().next() {
        if event.is_writable() {
            println!("Connected. Sending message: {}", message);
            stream.write_all(message.as_bytes())?;
        } else {
            eprintln!("Failed to connect or socket not writable");
            return Ok(());
        }
    }

    // Wait for the server's response
    events.clear();

    for _i in 0..2 {
        poll.poll(&mut events, Some(std::time::Duration::from_secs(30)))?;

        if let Some(event) = events.iter().next() {
            if event.is_readable() {
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(n) => {
                        let response = &buffer[0..n];
                        println!(
                            "Received response from server: {}",
                            str::from_utf8(response)?.trim()
                        );
                    }
                    Err(e) => eprintln!("Error reading response: {}", e),
                }
            } else {
                println!("No response from server within the timeout.");
            }
        }
    }

    Ok(())
}

fn mio_sample_main(is_server: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Example of running the server with a default address
    let address = "127.0.0.1:8080";
    if is_server {
        run_server(address)?;
    } else {
        // Example of running the client with a default message
        let message = "Hello, server!";
        run_client(address, message)?;
    }

    Ok(())
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_mio_simple() {
        mio_simple();
    }
    #[ignore = "mio server test"]
    #[test]
    fn test_mio_sample_server() {
        mio_sample_main(true);
    }

    #[ignore = "mio client test"]
    #[test]
    fn test_mio_sample_client() {
        mio_sample_main(false);
    }
}
