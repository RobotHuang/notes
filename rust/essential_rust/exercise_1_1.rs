fn main() {
    let a = 1;
    let b: u32 = 1;

    //mutable
    let mut c = 1;
    c = 2;
    println!("c = {}", c);
    let c: f32 = 4.1;
    println!("c = {}", c);

    const MAX_POINT: u32 = 10000;

    // array
    let arr1: [u32; 5] = [1,2,3,4,5];
    println!("arr1 = {:?}", arr1);

    // tuple
    let tup1: (i32, f32, char) = (3, 3.1415, 'a');
    let (x, y, z) = tup1;
    println!("tup1 = {:?}", tup1);
    println!("x = {}", x);

    println!("sum = {}", sum(1, 2));

    let d = {
        let e = 1;
        e + 2
    };
    println!("d = {}", d);
}

fn sum(a:i32, b:i32) -> i32 {
    let result = a + b;
    return result;
}

fn sum1(a:i32, b:i32) -> i32 {
    let result = a + b;
    // 注意结尾没有分号，表示需要返回
    result
}