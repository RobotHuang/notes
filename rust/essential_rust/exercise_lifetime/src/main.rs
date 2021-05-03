fn get_str<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
} 

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let s: &'static str = "I have a static lifetime.";
    println!("{}", get_str("a", "b"));
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{}", i.part);
    println!("{}", s);
}
