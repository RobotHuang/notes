use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move ||{
        for _i in 1..5 {
            let hello = String::from("Hello World");
            tx.send(hello).unwrap();
            // println!("{}", hello); // hello moves into tx
        }
    });

    thread::spawn(move || {
        let vec1 = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        for v in vec1 {
            tx1.send(v).unwrap();
        }
    });

    for _i in 1..5 {
        let receive = rx.recv().unwrap();
        println!("receive = {}", receive);
    }
}
