
#[link(name = "rust_test", kind = "static")]
unsafe extern "C" {
    unsafe fn cpp_print();
    unsafe fn square_add(a: i32, b: i32) -> i32;
}

fn main() {
    println!("Hello, rust");

    unsafe {
        cpp_print();
        let ret: i32 = square_add(2, 5);
        println!("{}", ret);
    }
}
