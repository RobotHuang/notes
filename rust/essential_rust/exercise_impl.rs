struct Dog {
    name: String,
    weight: f32,
    height: f32,
}

impl Dog {
    fn get_name(&self) -> &String {
        &self.name
    }
}

fn main() {
    let dog = Dog {
        name: String::from("dog"),
        weight: 100.0,
        height: 100.0,
    };
    let name = dog.get_name();
    println!("name = {}", name);
}