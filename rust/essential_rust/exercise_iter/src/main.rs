struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> { 
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new();
    for c in counter {
        println!("{}", c);
    }

    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("{}", total);
    println!("{:?}", v1);

    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("{:?}", v2);

    let v3: Vec<_> = v1.iter().filter(|&&x| {x % 2 == 0 }).collect();
    println!("{:?}", v3);
}
