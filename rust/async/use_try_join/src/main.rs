use futures;
use tokio::runtime::Runtime;
use std::io;

async fn func1() -> io::Result<()> {
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("func1");
    Ok(())
}

async fn func2() -> io::Result<()> {
    println!("func2");
    Ok(())
}

async fn async_main() {
    let f1 = func1();
    let f2 = func2();

    if let Err(_) = futures::try_join!(f1, f2) {
        println!("Error");
    }
}

fn main() {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async_main());
    println!("Hello, world!");
}
