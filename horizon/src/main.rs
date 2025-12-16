use crate::{
    rendering::{
        render_target::RENDER_TARGET_ORTHOGRAPHIC
    }
};

#[allow(unused)]
use crate::{
    rendering::application::*,
    rendering::mesh_data::{buffer},
};

mod tools {
    pub mod debug_widgets{
        pub mod widget;
    }
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
        pub mod shader_data_type;
        pub mod vertex_layout;
    }
}

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

    // initialize graphics api and generate window handle
    let handle = unsafe { window_init("Horizon") };
    
    // start main game loop
    unsafe { window_event_loop(handle, vec![RENDER_TARGET_ORTHOGRAPHIC]); }
}
