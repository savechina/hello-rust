use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut stdout = io::stdout();

    // Read lines from stdin
    for line in handle.lines() {
        let input = line?;
        if input == "done" {
            break;
        }
        // Process: convert to uppercase
        writeln!(stdout, "Received: {}", input.to_uppercase())?;
        stdout.flush()?;
    }

    Ok(())
}
