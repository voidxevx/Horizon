
fn main() {
    println!("cargo:rustc-link-search=native=./src/rust_test/bin/");
    println!("cargo:rustc-link-lib=static=rust_test");
}