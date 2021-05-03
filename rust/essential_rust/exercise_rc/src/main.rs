use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil
}

fn main() {
    let rc = List::Cons(1, Rc::new(List::Cons(2, Rc::new(List::Nil))));
    let rc1 = Rc::new(rc);
    println!("{:?}", rc1);
    println!("rc1 = {}", Rc::strong_count(&rc1));
    let rc2 = Rc::new(Rc::clone(&rc1));
    println!("rc1 = {}", Rc::strong_count(&rc1));
    {
        let rc3 = Rc::new(Rc::clone(&rc1));
        println!("rc1 = {}", Rc::strong_count(&rc1));
    }
    println!("rc1 = {}", Rc::strong_count(&rc1));
}
