use futures::{self, executor};

async fn move_block() {
    let my_string = "Hello".to_string();

    let f = async move {
        println!("{}", my_string);
    };

    // 必须要f.await，否则不会执行完线程就结束
    f.await
}


fn main() {
    executor::block_on(move_block());
}
