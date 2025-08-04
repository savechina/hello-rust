use std::io::{self, Write};
use std::process;
use std::thread;
use std::time::Duration; // Import for flush

fn process_getpid_sample() -> io::Result<()> {
    // Get the current process's ID
    let current_pid = process::id();

    // Print the PID to standard output
    // This makes it easy for another process to capture it.
    println!("PID: {}", current_pid);

    // Ensure the PID is immediately flushed to stdout
    // This is crucial if the parent process is trying to read it quickly.
    io::stdout().flush()?;

    println!(
        "Main application running with PID {}. Will run for 60 seconds unless stopped...",
        current_pid
    );

    // Simulate a long-running process
    thread::sleep(Duration::from_secs(60));

    println!("Main application finished naturally.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_process_getpid_sample() {
        process_getpid_sample().unwrap();
    }
}
