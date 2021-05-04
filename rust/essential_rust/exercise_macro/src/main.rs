// declarative macro
macro_rules! myVec {
    ($($x: expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// procedural macro
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let my_vec = myVec![1 + 2, 2, 3, 4, 5];
    println!("{:?}", my_vec);
    Pancakes::hello_macro();
}
