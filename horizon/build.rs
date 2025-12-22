fn main() {
    cxx_build::bridge("src/novascript/mod.rs")
        .file("src/novascript/state/novastate.cpp")
        .std("c++20")
        .compile("nova-bridge");

    println!("cargo:rerun-if-changed=src/novascript/state/novastate.cpp");
    println!("cargo:rerun-if-changed=build/novascript/state/novastate.h");
}