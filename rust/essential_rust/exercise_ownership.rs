fn main() {
    let s1 = String::from("hello world");
    // let s2 = s1;
    // value borrowed here after move
    // println!("{}", s1);
    let s2 = s1.clone();
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    let s3 = takes_and_gives_back(s2);
    println!("s3 = {}", s3);

    println!("The length of s3 = {}", calcaulate_length(&s3));
    println!("s3 = {}", s3);
    let mut s4 = String::from("Hello");
    change(&mut s4);
    println!("s4 = {}", s4);
}

// take s's ownership and give back
fn takes_and_gives_back(s: String) -> String {
    s
}

fn calcaulate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("!");
}