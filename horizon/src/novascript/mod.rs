use std::ffi::c_char;
use cxx::CxxString;
use cxx::SharedPtr;

#[cxx::bridge(namespace = "nova")]
pub mod nova {




    extern "Rust" {

    }

    unsafe extern "C++" {
        include!("horizon/src/novascript/state/novastate.h");

        pub type State;

        pub fn linkModule(self: Pin<&mut State>, moduleName: &CxxString, filePath: &CxxString);
        pub fn loadModule(self: Pin<&mut State>, moduleName: &CxxString);

        pub fn new_novastate() -> SharedPtr<State>;
    }

    
}