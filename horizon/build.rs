
fn main() {
    println!("cargo:rustc-link-search=native=./src/Nebula/bin/Neblang/Release_x64/");
    println!("cargo:rustc-link-lib=static=Neblang");
}