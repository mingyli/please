use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;

fn proxy_output(sender: mpsc::Sender<u8>, output: &mut dyn Read) -> io::Result<()> {
    for byte in output.bytes() {
        sender.send(byte?).unwrap();
    }
    Ok(())
}

fn forward_output(receiver: mpsc::Receiver<u8>, mut out: &mut dyn Write) -> io::Result<()> {
    for byte in receiver {
        out.write_all(&[byte])?;
        if byte as char == '?' {
            write!(&mut out, "🥺👉👈")?;
        }
        out.flush()?;
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

    let (stdout_sender, stdout_receiver) = mpsc::channel();
    let (stderr_sender, stderr_receiver) = mpsc::channel();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    let proxy_stdout = std::thread::spawn(move || proxy_output(stdout_sender, &mut stdout));
    let proxy_stderr = std::thread::spawn(move || proxy_output(stderr_sender, &mut stderr));
    let forward_stdout =
        std::thread::spawn(move || forward_output(stdout_receiver, &mut io::stdout()));
    let forward_stderr =
        std::thread::spawn(move || forward_output(stderr_receiver, &mut io::stderr()));
    proxy_stdout.join().unwrap()?;
    proxy_stderr.join().unwrap()?;
    forward_stdout.join().unwrap()?;
    forward_stderr.join().unwrap()?;
    Ok(())
}
