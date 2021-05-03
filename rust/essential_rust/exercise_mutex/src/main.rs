use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

// Mutex是一个智能指针
// Mutex和Arc相当于线程安全的RefCell和Rc
fn main() {
    let mutex = Mutex::new(0);
    let counter = Arc::new(mutex);
    let mut handles = vec![];

    for _i in 1..5 {
        let cnt = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = cnt.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.lock().unwrap());
}
