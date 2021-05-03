use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("abc".to_string(), 10);

    if let Some(v) = scores.get("abc") {
        println!("v = {}", v);
    }

    let keys = vec![String::from("red"), String::from("yellow")];
    let values = vec![1, 2];
    let scores1: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    for (key, value) in &scores1 {
        println!("key = {}, value = {}", key, value);
    }
    scores1.entry("blue".to_string()).or_insert(3);
}