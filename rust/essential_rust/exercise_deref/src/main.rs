use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(v: T) -> MyBox<T> {
        MyBox(v)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.0
    }
}

fn hello(s: &str) {
    println!("{}", s);
}

fn main() {
    let my_box = MyBox::new(3);
    assert_eq!(3, *my_box);

    // Rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：
    // 当 T: Deref<Target=U> 时从 &T 到 &U。
    // 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    // 当 T: Deref<Target=U> 时从 &mut T 到 &U。
    let my_str = MyBox::new(String::from("Hello World"));
    hello(&my_str);
}
