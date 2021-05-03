use futures::{select, future::FutureExt, pin_mut};
use tokio;

async fn func1() -> std::io::Result<()> {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("func1");
    Ok(())
}

async fn func2() -> std::io::Result<()> {
    println!("func2");
    Ok(())
}

async fn async_main() {
    let f1 = func1().fuse();
    let f2 = func2().fuse();

    pin_mut!(f1, f2);

    select! {
        _ = f1 => println!("func1 finished"),
        _ = f2 => println!("func2 finished"),
    };

}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async_main());
    println!("Hello, world!");
}
