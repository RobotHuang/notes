// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions

// call C's function
extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe fn function_unsafe() {
    println!("this function is unsafe");
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    dereference_raw_pointer();
    unsafe {
        function_unsafe();
        println!("abs = {}", abs(-4));
    }
}

fn dereference_raw_pointer() {
    let mut num = 2;
    // create raw pointer in safe code
    let r1 = &mut num as *mut i32;
    unsafe {
        *r1 += 1;
        println!("{}", *r1);
    }
    add_to_count(5);
}
