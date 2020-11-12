use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut child = Command::new(&args[1])
        .args(&args[2..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let (stdout_sender, stdout_receiver) = mpsc::channel();
    let (stderr_sender, stderr_receiver) = mpsc::channel();
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    std::thread::spawn(move || -> io::Result<()> {
        for byte in stdout.bytes() {
            stdout_sender.send(byte? ).unwrap();
        }
        Ok(())
    });
    std::thread::spawn(move || -> io::Result<()> {
        for byte in stderr.bytes() {
            stderr_sender.send(byte? ).unwrap();
        }
        Ok(())
    });

    fn forward_output(
        receiver: mpsc::Receiver<u8>,
        mut out: &mut dyn Write,
    ) -> io::Result<()> {
        for byte in receiver {
            out.write_all(&[byte])?;
            if byte as char == '?' {
                write!(&mut out, "🥺👉👈")?;
            }
            out.flush()?;
        }
        Ok(())
    }

    std::thread::spawn(move || forward_output(stdout_receiver, &mut io::stdout()));
    forward_output(stderr_receiver, &mut io::stderr())?;

    Ok(())
}
