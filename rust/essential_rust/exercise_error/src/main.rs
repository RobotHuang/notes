use std::fs::File;
use std::io::prelude::*;
use std::io;

fn main() {
    // panic!("crash here");
    // Method 1
    // let file = File::open("hello.txt");
    // let r = match file {
    //     Ok(file) => file,
    //     Err(error) => panic!("{:?}", error),
    // };
    
    // // Method 2
    // let f1 = File::open("hello.txt").unwrap();

    // // Method 3
    // let f2 = File::open("hello.txt").expect("crash here");

    let s = read_from_file("hello.txt".to_string()).unwrap();
    println!("{}", s);
}

fn read_from_file(file_name: String) -> Result<String, io::Error> {
    // let f = File::open(file_name);
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(error) => Err(error),
    // }
    
    let mut f = File::open(file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    // let mut s = String::new();
    // File::open(file_name)?.read_to_string(&mut s)?;
    // Ok(s)
}