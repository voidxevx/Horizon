
use std::pin::pin;

use cxx::{CxxString, let_cxx_string};

use crate::{
    novascript::nova::{State, new_novastate}, 
    rendering::{
        application::*, 
        mesh_data::buffer, 
        render_target::RENDER_TARGET_ORTHOGRAPHIC
    ,}
};

mod tools {
    pub mod math{
        pub mod vector;
        pub mod matrix;
        pub mod transforms;
    }
}

mod rendering {
    pub mod application;
    pub mod renderer;
    pub mod camera;
    pub mod render_target;
    pub mod material;
    pub mod mesh_data {
        pub mod buffer;
        pub mod shader;
        pub mod vertex_array;
        pub mod texture;
        pub mod shader_types;
        pub mod vertex_layout;
    }

}

pub mod novascript;

const TITLE: &str = 
r#"
██░ ██  ▒█████   ██▀███   ██▓▒███████▒ ▒█████   ███▄    █ 
▓██░ ██▒▒██▒  ██▒▓██ ▒ ██▒▓██▒▒ ▒ ▒ ▄▀░▒██▒  ██▒ ██ ▀█   █ 
▒██▀▀██░▒██░  ██▒▓██ ░▄█ ▒▒██▒░ ▒ ▄▀▒░ ▒██░  ██▒▓██  ▀█ ██▒
░▓█ ░██ ▒██   ██░▒██▀▀█▄  ░██░  ▄▀▒   ░▒██   ██░▓██▒  ▐▌██▒
░▓█▒░██▓░ ████▓▒░░██▓ ▒██▒░██░▒███████▒░ ████▓▒░▒██░   ▓██░
 ▒ ░░▒░▒░ ▒░▒░▒░ ░ ▒▓ ░▒▓░░▓  ░▒▒ ▓░▒░▒░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
 ▒ ░▒░ ░  ░ ▒ ▒░   ░▒ ░ ▒░ ▒ ░░░▒ ▒ ░ ▒  ░ ▒ ▒░ ░ ░░   ░ ▒░
 ░  ░░ ░░ ░ ░ ▒    ░░   ░  ▒ ░░ ░ ░ ░ ░░ ░ ░ ▒     ░   ░ ░ 
 ░  ░  ░    ░ ░     ░      ░    ░ ░        ░ ░           ░ 
                              ░                            
"#;

fn main() {
    println!("\x1b[31m{}\x1b[0m", TITLE);

    unsafe
    {
        let mut state = new_novastate();
        let_cxx_string!(testmod = "test");
        let_cxx_string!(testpath = "./content/scripts/test.ns");
        println!("[RS] linking module");
        state.pin_mut_unchecked().linkModule(&testmod, &testpath);
        println!("loading module");
        state.pin_mut_unchecked().loadModule(&testmod);
        println!("module loaded successfully");
    }

    // initialize graphics api and generate window handle
    let handle = unsafe { window_init("Horizon") };

    // start main game loop
    unsafe { window_event_loop(handle, RENDER_TARGET_ORTHOGRAPHIC); }
}
