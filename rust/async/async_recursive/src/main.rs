use futures::future::{BoxFuture, FutureExt};

fn re() -> BoxFuture<'static, ()> {
    // Box::pin(async move {
    //     re().await;
    //     re().await;
    // })
    
    async move {
        re().await;
        re().await;
    }.boxed()

}

fn main() {
    re();
    println!("Hello, world!");
}
