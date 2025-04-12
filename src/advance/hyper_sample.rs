use std::convert::Infallible;
use std::fmt::format;
use std::net::SocketAddr;

use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::{Body, Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

/// This function handles the `/hello` endpoint.
/// It will respond with "Hello, World!".
/// It uses `hyper::Body` to handle the response body.
/// The `hello` function is a simple handler that responds with a static message.
/// It uses `hyper::Body` to handle the response body.
async fn hello(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

/// This function handles the `/echo` endpoint.
/// It will echo back the body of the request.
/// It uses `hyper::Body` to handle the request body.
async fn echo(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    println!("Received request: {:?}", req);
    // Extract the body of the request
    let body = req.body();
    println!("Received body: {:?}", body);

    // Await the whole body to be collected into a single `Bytes`...
    let whole_body = req.collect().await.unwrap().to_bytes();
    println!("Received body: {:?}", String::from_utf8_lossy(&whole_body));

    Ok(Response::new(Full::new(Bytes::from(format!(
        "Echo:{}",
        String::from_utf8_lossy(&whole_body),
    )))))
}

//// This function is the main entry point for our server.
/// It will be called for each incoming connection.
/// It will route requests to the appropriate handler based on the path.
async fn index(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    match req.uri().path() {
        "/hello" => hello(req).await,
        "/echo" => echo(req).await,
        _ => {
            let response = Response::new(Full::new(Bytes::from("404 Not Found")));
            Ok(response)
        }
    }
}

// We create some utility functions to make Empty and Full bodies
// fit our broadened Response body type.
fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

#[tokio::main]
async fn hyper_hello_sample() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(index))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[ignore]
    #[test]
    fn test_hyper_sample() {
        hyper_hello_sample();
    }
}
