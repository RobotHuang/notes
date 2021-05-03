use futures::executor;
use std::thread;
use std::time::Duration;

async fn hello() {
    println!("hello");
    thread::sleep(Duration::from_secs(1));
}
// =>
// fn hello() -> impl Future<Output=()> {
//     async {
//         println!("hello");
//     }
// }

fn my_function() {
    println!("my function!");
}

fn main() {
    let f = hello();
    executor::block_on(f);

    my_function();
}
