use clap::{Args, Parser, Subcommand};
#[cfg(target_family = "unix")] // This code only compiles on Unix-like systems
use nix::sys::signal::{kill, Signal};
#[cfg(target_family = "unix")]
use nix::unistd::Pid;
use nix::Error;
use std::fs;
use std::io::{self, Write};
use std::process;
use std::str::FromStr;
use std::thread;
use std::time::Duration; // Import for flush
#[derive(Parser)]
#[command(author="ren", version, about="about advance utils", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the Application
    Start,
    /// Stop the Application
    Stop,
}

fn main() -> io::Result<()> {
    env_logger::init(); // 初始化日志
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => {
            return start();
        }
        Commands::Stop => {
            return stop();
        }
    }
}

fn start() -> io::Result<()> {
    // Get the current process's ID
    let current_pid = process::id();

    // Print the PID to standard output
    // This makes it easy for another process to capture it.
    println!("PID: {}", current_pid);

    // Ensure the PID is immediately flushed to stdout
    // This is crucial if the parent process is trying to read it quickly.
    io::stdout().flush()?;

    // Define the file where the PID is stored
    let pid_file = "app.pid";

    match fs::exists(pid_file).unwrap_or(false) {
        exists if exists => {
            eprintln!(
                "PID file '{}' already exists. Application might be running.",
                pid_file
            );
            eprintln!("Exiting without starting a new instance.");
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "PID file '{}' already exists. Application might be running.",
                    pid_file
                ),
            ));
        }
        _ => (),
    }
    // Write the PID to a file for later reference

    let pid_string = match fs::write(pid_file, current_pid.to_string()) {
        Ok(_) => {
            println!(
                "Successfully wrote PID {} to file '{}'",
                current_pid, pid_file
            );
            current_pid.to_string()
        }
        Err(e) => {
            eprintln!("Error write PID file '{}': {}", pid_file, e);
            // eprintln!("Make sure the main application is running and has written its PID.");
            return Err(e); // Exit if we can't read the PID
        }
    };

    println!(
        "Main application running with PID {}. Will run for 60 seconds unless stopped...",
        pid_string
    );

    // Simulate a long-running process
    thread::sleep(Duration::from_secs(60));

    println!("Main application finished naturally.");

    // 删除 PID 文件
    fs::remove_file(pid_file)?;
    Ok(())
}

fn stop() -> io::Result<()> {
    println!("--- Stopper Application ---");

    // Define the file where the PID is stored
    let pid_file = "app.pid";

    println!("--- Process Graceful Shutdown with PID File Cleanup ---");

    // Define the file where the PID is stored
    let pid_file = "app.pid"; // Make sure this matches where your main_app outputs its PID

    // 1. Read the PID from the file
    println!("Attempting to read PID from '{}'...", pid_file);
    let pid_string = match fs::read_to_string(pid_file) {
        Ok(s) => {
            println!("Successfully read PID string: '{}'", s.trim());
            s.trim().to_string()
        }
        Err(e) => {
            eprintln!("Error reading PID file '{}': {}", pid_file, e);
            eprintln!(
                "Ensure the main application is running and has written its PID to this file."
            );
            return Err(e); // Exit if we can't read the PID
        }
    };

    let target_pid: u32 = match pid_string.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!(
                "Invalid PID '{}' found in file. It must be a valid number.",
                pid_string
            );
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid PID format",
            ));
        }
    };

    // 2. Attempt to gracefully shut down the process and clean up the PID file
    // Give it a 5-second grace period.
    match graceful_shutdown_process_and_cleanup(target_pid, 5, pid_file) {
        Ok(_) => println!("Process shutdown and PID file cleanup operation completed."),
        Err(e) => eprintln!(
            "Process shutdown and PID file cleanup operation failed: {}",
            e
        ),
    }

    println!("\n--- Stopper Finished ---");
    Ok(())
}

/// Attempts to gracefully shut down a process with the given PID.
/// If graceful shutdown fails, it tries a forceful kill.
/// After attempting to stop the process, it deletes the specified PID file.
fn graceful_shutdown_process_and_cleanup(
    pid: u32,
    timeout_seconds: u64,
    pid_file_path: &str, // Added parameter for the PID file path
) -> io::Result<()> {
    #[cfg(not(target_family = "unix"))]
    {
        // On non-Unix systems (like Windows), signal mechanisms don't exist.
        eprintln!(
            "Graceful shutdown via signals is only supported on Unix-like systems. For Windows, consider using `taskkill`."
        );
        // On Windows, you might use std::process::Command to run taskkill.
        // Command::new("taskkill").args(&["/PID", &pid.to_string(), "/F"]).status()?;
        // After that, you'd delete the file: fs::remove_file(pid_file_path)?;
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Unsupported OS for signal-based process termination.",
        ));
    }

    #[cfg(target_family = "unix")]
    {
        let nix_pid = Pid::from_raw(pid as i32);
        let mut process_stopped = false;

        println!("Attempting graceful shutdown for PID {}...", pid);

        // 1. Send SIGTERM signal
        match kill(nix_pid, Signal::SIGTERM) {
            Ok(_) => {
                println!(
                    "Sent SIGTERM to PID {}. Waiting for {} seconds...",
                    pid, timeout_seconds
                );
                // 2. Wait for the process to exit
                thread::sleep(Duration::from_secs(timeout_seconds));

                // 3. Check if the process is still running
                // Send SIGCONT (continue) signal to check if process exists
                match kill(nix_pid, Signal::SIGCONT) {
                    // Sending SIGCONT checks if the process exists and you have permission
                    Ok(_) => {
                        // If successful, the process is still running
                        println!("PID {} is still running. Sending SIGKILL...", pid);
                        // 4. Send SIGKILL signal (forceful termination)
                        match kill(nix_pid, Signal::SIGKILL) {
                            Ok(_) => {
                                println!(
                                    "Sent SIGKILL to PID {}. Process forcibly terminated.",
                                    pid
                                );
                                process_stopped = true;
                            }
                            Err(e) => {
                                eprintln!("Failed to send SIGKILL to PID {}: {}", pid, e);
                                return Err(io::Error::new(
                                    io::ErrorKind::Other,
                                    format!("Failed to SIGKILL process: {}", e),
                                ));
                            }
                        }
                    }
                    Err(e) if e == nix::errno::Errno::ESRCH => {
                        // ESRCH (No such process) means it exited gracefully
                        println!("PID {} exited gracefully. Shutdown successful.", pid);
                        process_stopped = true;
                    }
                    Err(e) => {
                        eprintln!("Failed to check status of PID {}: {}", pid, e);
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!("Failed to check process status: {}", e),
                        ));
                    }
                }
            }
            Err(e) if e == nix::errno::Errno::ESRCH => {
                println!(
                    "PID {} was not running or already exited. No action needed.",
                    pid
                );
                process_stopped = true; // Consider it stopped if it wasn't running
            }
            Err(e) => {
                eprintln!("Failed to send SIGTERM to PID {}: {}", pid, e);
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to SIGTERM process: {}", e),
                ));
            }
        }

        // --- NEW STEP: Delete the PID file ---
        if process_stopped {
            // Only delete if we successfully stopped it or found it not running
            println!("Attempting to delete PID file '{}'...", pid_file_path);
            match fs::remove_file(pid_file_path) {
                Ok(_) => {
                    println!("Successfully deleted PID file: '{}'", pid_file_path);
                }
                Err(e) => {
                    eprintln!(
                        "Failed to delete PID file '{}': {}. You may need to remove it manually.",
                        pid_file_path, e
                    );
                    // Decide if this is a critical error or just a warning.
                    // For now, we'll continue, but print the error.
                }
            }
        } else {
            println!(
                "Process PID {} could not be stopped. PID file '{}' was NOT deleted.",
                pid, pid_file_path
            );
        }

        Ok(())
    }
}
