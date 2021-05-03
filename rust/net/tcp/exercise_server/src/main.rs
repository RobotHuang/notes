use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&mut buf)?;
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080");
    let mut handles = vec![];
    for stream in listener?.incoming() {
        let handle = thread::spawn(|| {
            let stream = stream.expect("failed");
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}
