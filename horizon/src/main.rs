use std::path::Path;

use cxx::{CxxString, let_cxx_string};

use crate::{
    novascript::nova::{State, new_novastate}, 
    rendering::{
        application::*, 
        mesh_data::buffer, 
        render_target::RENDER_TARGET_ORTHOGRAPHIC
    }, 
    tools::file_manager::nova_file_loader::nova_load_files,
};

mod tools {
    pub mod math{
        pub mod vector;
        pub mod matrix;
        pub mod transforms;
    }
    pub mod file_manager{
        pub mod nova_file_loader;
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
        nova_load_files(state.clone(), Path::new(".\\content\\scripts"))
            .expect("Error while loading files.");
        let_cxx_string!(root_mod = "root");
        state.pin_mut_unchecked().loadModule(&root_mod);

        // initialize graphics api and generate window handle
        let handle = window_init("Horizon");
        // start main game loop
        window_event_loop(handle, RENDER_TARGET_ORTHOGRAPHIC);
    }


}
