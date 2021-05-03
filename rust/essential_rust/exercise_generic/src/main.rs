#[derive(Debug)]
struct Point<T: Copy, U: Copy> {
    x: T,
    y: U,
}

fn new_point<V: Copy, W: Copy>(x: V, y: W) -> Point<V, W> {
    return Point {
        x: x,
        y: y,
    };
}

impl <T: Copy, U: Copy> Point<T, U> {
    fn create_point<V:Copy, W:Copy>(&self, p2: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: p2.y,
        }
    }
}

fn largest_number<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let p = new_point(10, 11);
    println!("{:?}", p);
    let p1 = Point {
        x: 10,
        y: 20.0,
    };
    let p2 = p.create_point(p1);
    println!("{:?}", p2);
    println!("{:?}", p);

    let vec1 = vec![1, 2, 3, 4];
    println!("{}", largest_number(&vec1));
}
