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

pub mod produce_refrigerator {
    pub fn produce_re() {
        println!("produce_re");
    }
}

pub fn hello_world() {
    println!("hello world")
}