#[derive(Debug)]
struct User {
    name: String,
    age: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        name: String::from("abc"),
        age: 18,
        active: false,
    };
    println!("user1 = {:#?}", user1);

    let name = String::from("cde");
    let age = 29;
    let active = true;
    let user2 = User {
        name,
        age,
        active,
    };
    println!("user2 = {:#?}", user2);

    let user3 = User {
        ..user2
    };
    println!("user3 = {:#?}", user3);

    let user4 = User{
        name: String::from("gd"),
        ..user2
    };
    println!("user4 = {:#?}", user4);

    #[derive(Debug)]
    struct Point(i32, i32);
    let a = Point(1, 2);
    println!("a = {:?}", a);
    let Point(x, y) = a;
    println!("x = {}", x);
}