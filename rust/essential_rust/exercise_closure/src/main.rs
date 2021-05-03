fn main() {
    let func1 = ||{
        println!("Hello World");
    };
    func1();
    let func2 = |num| {
        println!("{}", num);
    };
    func2(2);
    // func2(3.4); // 已经类型推断过为 i32
    let func3 = |s: String| {
        println!("{}", s);
    };
    func3("Hello World".to_string());

    let mut cacher = Cacher::new(|i|{i+1});
    println!("{}", cacher.value(2));

    let x = 10;
    let equal_to_x = |z| x == z;
    let y = 10;
    assert!(equal_to_x(y));

    let vec1 = vec![1, 2,3 ,4, 5];
    let equal_to_vec1 = move |z| {z == vec1};
    // println!("{:?}", vec1);
    let vec2 = vec![1, 2, 3, 4, 5];
    assert!(equal_to_vec1(vec2));
}

// 闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。这三种捕获值的方式被编码为如下三个 Fn trait：
// FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
// FnMut 获取可变的借用值所以可以改变其环境
// Fn 从其环境获取不可变的借用值
struct Cacher<T: Fn(i32) -> i32> {
    calculation: T,
    value: Option<i32>,
}

impl<T> Cacher<T> where T: Fn(i32) -> i32 {
    fn new(c: T) -> Cacher<T> {
        Cacher {
            calculation: c,
            value: None,
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}