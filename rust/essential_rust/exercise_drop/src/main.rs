struct Dog {
    name: String,
    count: i32,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("{}", self.name)
    }
}

fn main() {
    let a = Dog {name: String::from("a"), count: 12};

    {
        drop(a); // 提前释放
        let b = Dog {name: String::from("b"), count: 13};
    }
}