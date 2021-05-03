use futures;
use tokio::runtime::Runtime;

async fn func1() {
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("func1");
}

async fn func2() {
    println!("func2");
}

async fn async_main() {
    let f1 = func1();
    let f2 = func2();

    futures::join!(f1, f2);
}

fn main() {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async_main());
    println!("Hello, world!");
}
