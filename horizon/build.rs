fn main() {
    cxx_build::bridge("src/novascript/mod.rs")
        .file("src/novascript/state/novastate.cpp")
        .file("src/novascript/generation/token.cpp")
        .std("c++20")
        .compile("nova-bridge");

    println!("cargo:rerun-if-changed=src/novascript/state/novastate.cpp");
    println!("cargo:rerun-if-changed=src/novascript/state/novastate.h");
}