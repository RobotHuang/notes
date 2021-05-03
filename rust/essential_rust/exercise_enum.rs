#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let ip_v4 = IPAddr::V4(String::from("192.168.1.0"));
    println!("ipv4 = {:?}", ip_v4);
    let message = Message::Move{x: 1, y: 2};
    println!("message = {:?}", message);

    let y: Option<i32> = Some(5);
    println!("y = {:?}", y);
    if let Some(v) = y {
        println!("v = {}", v);
    }

    if let Some(x) = plus_one(y) {
        println!("x = {}", x);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}