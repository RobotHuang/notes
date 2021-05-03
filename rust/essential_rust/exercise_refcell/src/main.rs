use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(1));
    let a = Rc::new(Cons(value.clone(), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), a.clone());
    let c = Cons(Rc::new(RefCell::new(4)), a.clone());
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    *value.borrow_mut() += 10;
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

}