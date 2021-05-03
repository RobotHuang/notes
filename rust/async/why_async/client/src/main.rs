use std::io::{self, prelude::*, BufReader};
use std::net::{TcpStream};
use std::str;

use futures::join;
use futures::executor;

async fn async_user_server(server: &str, port: u16, content: &str) {
    user_server(server, port, content).unwrap();
}

async fn use_all_servers() {
    let f1 = async_user_server("127.0.0.1", 8080, "server 1");
    let f2 = async_user_server("127.0.0.1", 8080, "server 2");
    join!(f1, f2);
}

fn user_server(server: &str, port: u16, content: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect((server, port))?;
    stream.write(content.as_bytes())?;

    let mut reader = BufReader::new(&stream);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_until(b'\n', &mut buffer)?;

    println!("{}", str::from_utf8(&buffer).unwrap());
    Ok(())
}

fn main() -> io::Result<()> {
    let f = use_all_servers();
    executor::block_on(f);
    Ok(())
}
