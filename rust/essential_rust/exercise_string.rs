fn main() {
    let mut s0 = String::new();
    s0.push_str("hello");
    println!("s0 = {}", s0);
    let s1 = "init".to_string();
    println!("s1= {}", s1);
    let s2 = format!("{}-{}", s0, s1);
    println!("s2 = {}", s2);

    for c in s2.chars() {
        println!("c = {}", c);
    }

    for b in s2.bytes() {
        println!("b = {}", b);
    }
}