use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("i = {} in thread", i);
            thread::sleep(Duration::from_millis(2))
        }
    });

    for i in 1..5 {
        println!("number in main = {}", i);
    }
    handle.join().unwrap();

    let vec1 = vec![1, 2, 3, 4, 5];
    let handle1 = thread::spawn(move || {
        for v in vec1.iter() {
            println!("{}", v);
        }
    });
    handle1.join().unwrap();
}
