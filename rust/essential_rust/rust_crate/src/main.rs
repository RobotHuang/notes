// pub mod factory {
//     pub mod produce_refrigerator {
//         pub fn produce_re() {
//             println!("produce_re");
//         }
//     }

//     mod produce_washing_machine {
//         fn produce_washing_machine() {
//             println!("produce_washing_machine");
//         }
//     }
    
//     // 父模块调用 super
//     mod B {
//         pub fn call_super() {
//             super::produce_refrigerator::produce_re();
//         }
//     }
// }
mod factory;
use mylib::front_of_house;

// extern crate crypto;
use crypto::digest::Digest;
use crypto::sha3::Sha3;

fn main() {
    factory::produce_refrigerator::produce_re();
    factory::hello_world();
    front_of_house::hosting::add_to_waitlist();

    let mut hasher = Sha3::sha3_256();
    hasher.input_str("hello world");
    let result = hasher.result_str();
    println!("result = {}", result);
}
