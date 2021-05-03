fn main() {
    let mut v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("v1 = {:?}", v1);

    let mut v2 = Vec::new();
    v2.push(1);
    println!("v2 = {:?}", v2);
    println!("v2[0] = {}", v2[0]);
    let one = v2[0];
    v2[0] = 2;
    println!("one = {}", one);
    println!("v2 = {:?}", v2);

    // recommend
    if let Some(v) = v1.get(1) {
        println!("v1[1] = {}", v);
    }

    // v1 moves into into_iter(self)
    // for i in v1 {
    //     println!("i = {}", i);
    // }
    for i in &v1 {
        println!("i = {}", i);
    }
    for i in v1.iter() {
        println!("i = {}", i);
    }

    // 可变的遍历
    for i in &mut v1 {
        *i += 1;
        println!("{}", i);
    }
    println!("v1 = {:?}", v1);

    // 不可变引用 -> 可变引用 -> 使用不可变引用 错误
    // let mut v3 = vec![1,3,4,5];
    // let first = &v3[0];
    // v3.push(6);
    // println!("{}", first);
    
    // 第一次可变引用 -> 第二次可变引用 -> 使用第一个可变引用 错误
    // let mut v3 = vec![1,3,4,5];
    // let mut first = &mut v3[0];
    // v3.push(6);
    // println!("{}", first);
}