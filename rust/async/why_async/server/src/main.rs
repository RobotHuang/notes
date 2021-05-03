use std::net::{TcpListener, TcpStream};
use std::io::{self, prelude::*};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream, time: u64) -> io::Result<()> {
    let mut buf = [0; 125];
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&mut buf)?;
        thread::sleep(Duration::from_secs(time));
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    
    let mut handles = vec![];
    for stream in listener.incoming() {
        let handle = thread::spawn(|| {
            let stream = stream.expect("failed");
            handle_client(stream, 3).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}
