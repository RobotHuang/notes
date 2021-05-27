use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Meters(u32);
#[derive(Debug, PartialEq)]
struct Millimeters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> <Self as std::ops::Add<Meters>>::Output {
        Millimeters(self.0 + other.0)
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("pilot");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("wizard");
    }
}

impl Human {
    fn fly(&self) {
        println!("human");
    }
}

trait Animal {
    fn baby_name();
}

struct Dog;

impl Animal for Dog {
    fn baby_name() {
        println!("puppy");
    }
}

impl Dog {
    fn baby_name() {
        println!("spot");
    }
}

use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "[{}]", self.0.join(","))
    }
}
fn main() {
    let p1 = Point { x: 1, y: 2 };

    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    assert_eq!(p3, Point { x: 4, y: 6 });
    assert_eq!(Millimeters(2) + Meters(1), Millimeters(3));
    let h = Human;
    Pilot::fly(&h);
    // 完全限定语法
    <Dog as Animal>::baby_name();
    p3.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
