fn main() {
    cxx_build::bridge("src/novascript/mod.rs")
        .file("src/novascript/state/novastate.cpp")
        .file("src/novascript/generation/token.cpp")
        .file("src/novascript/generation/nameJudger.cpp")
        .std("c++20")
        .compile("novascript");

    println!("cargo:rerun-if-changed=src/novascript/state/novastate.cpp");
    println!("cargo:rerun-if-changed=src/novascript/generation/nameJudger.cpp");
    println!("cargo:rerun-if-changed=src/novascript/state/novastate.h");
}