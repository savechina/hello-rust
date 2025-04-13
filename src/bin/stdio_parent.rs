use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let messages = vec!["hello", "world", "rust", "done"];

    // Spawn the child process
    let mut child = Command::new("./stdio_child")
        .stdin(Stdio::piped()) // Pipe for parent's writes
        .stdout(Stdio::piped()) // Pipe for parent's reads
        .spawn()?;

    // Get handles to child's stdin and stdout
    let mut child_stdin = child.stdin.take().expect("Failed to open child stdin");
    let child_stdout = child.stdout.take().expect("Failed to open child stdout");
    let mut reader = BufReader::new(child_stdout);

    // Communicate with the child
    for msg in messages {
        // Write to child's stdin
        writeln!(child_stdin, "{}", msg)?;
        child_stdin.flush()?;

        // Read from child's stdout
        let mut response = String::new();
        reader.read_line(&mut response)?;
        print!("Parent got: {}", response);
        io::stdout().flush()?;
    }

    // Drop stdin to close the pipe
    drop(child_stdin);

    // Wait for the child to finish
    child.wait()?;

    Ok(())
}
