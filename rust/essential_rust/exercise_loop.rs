fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("result = {}", result);

    let mut i = 0;
    while i < 10 {
        i += 1;
    }
    println!("i = {}", i);

    let arr1:[u32;5] = [1,2,3,4,5];
    for i in arr1.iter() {
        println!("i = {}", i);
    }
    for i in &arr1 {
        println!("i = {}", i);
    }
    println!("----reverse----");
    for number in (0..5).rev() {
        println!("number = {}", number);
    }
}