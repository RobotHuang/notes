pub trait GetInfor {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> i32;
}

#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
}

impl GetInfor for Student {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_age(&self) -> i32 {
        return self.age;
    }
}

fn print_name(get_info: &impl GetInfor) {
    println!("{}", get_info.get_name());
}

fn print_age<T>(item: &T) where T: GetInfor {
    println!("{}", item.get_age());
}

pub trait SchoolName {
    fn get_school_name(&self) -> String {
        return String::from("school");
    }
}

impl SchoolName for Student {}

fn main() {
    let s = Student {
        name: "abc".to_string(),
        age: 21,
    };
    println!("{}", s.get_age());
    print_name(&s);
    print_age(&s);
    s.get_school_name();
}
