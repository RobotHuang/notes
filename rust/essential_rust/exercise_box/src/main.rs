#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

fn main() {
    let b = Box::new(5); // b存储于栈上，5在堆上
    println!("{}", b);
    assert_eq!(5, *b);

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);
}



