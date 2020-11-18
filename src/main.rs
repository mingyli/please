use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

fn proxy_output(child_output: &mut dyn Read, mut output: &mut dyn Write) -> io::Result<()> {
    for byte in child_output.bytes() {
        let byte = byte?;
        output.write_all(&[byte])?;
        if byte as char == '?' {
            write!(&mut output, "🥺👉👈")?;
        }
        output.flush()?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut child = Command::new(&args[1])
        .args(&args[2..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut child_stdout = child.stdout.take().unwrap();
    let mut child_stderr = child.stderr.take().unwrap();

    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let proxy_stdout = std::thread::spawn(move || proxy_output(&mut child_stdout, &mut stdout));
    let proxy_stderr = std::thread::spawn(move || proxy_output(&mut child_stderr, &mut stderr));
    proxy_stdout.join().unwrap()?;
    proxy_stderr.join().unwrap()?;
    Ok(())
}
