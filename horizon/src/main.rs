use crate::{
    rendering::{
        camera::{
            Camera, 
            OrthographicCamera
        },

        mesh_data::{
            buffer::Buffer, 
            shader::*, 
            texture::Texture, 
            vertex_array::VertexArray
        },
        
        renderer::Renderer
    },

    tools::{
        math::vector::{
            Vec2, 
            Vec3
        }, 
        nebula::{
            debug::{
                self, NEB_ERROR_INFO, enable_debug_mode, print_error, set_error_pipe
            }, 
            neb
        }
    }
};

use crate::{
    rendering::application::*,
    rendering::mesh_data::{buffer},
};

mod tools {
    pub mod math{
        pub mod vector;
        pub mod matrix;
        pub mod transforms;
    }
    pub mod nebula{
        pub mod neb;
        pub mod debug;
    }
}

mod rendering {
    pub mod application;
    pub mod renderer;
    pub mod camera;
    pub mod mesh_data {
        pub mod buffer;
        pub mod shader;
        pub mod vertex_array;
        pub mod texture;
    }
}

use std::ffi::CString;

#[repr(C, packed)]
struct Vertex(Vec3, Vec2);


fn main() {

    // initialize graphics api and generate window handle
    let handle = unsafe {
        window_init("Horizon")
    };
        
    // TEMP
    let mesh: [Vertex; 4] = [
        Vertex(Vec3::new([0.0,   0.0,   0.0]), Vec2::new([0.0, 1.0])),
        Vertex(Vec3::new([100.0, 0.0,   0.0]), Vec2::new([1.0, 1.0])),
        Vertex(Vec3::new([100.0, 100.0, 0.0]), Vec2::new([1.0, 0.0])),
        Vertex(Vec3::new([0.0,   100.0, 0.0]), Vec2::new([0.0, 0.0])),
    ];

    let indeces: [i32; 6] = [
        0, 1, 2,
        2, 3, 0,
    ];


    let shader = generate_shader("./content/shaders/default.shader").unwrap();

    let vertex_array = VertexArray::new();
    vertex_array.bind();

    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
    vertex_buffer.buffer_data(&mesh, gl::STATIC_DRAW);

    let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
    index_buffer.buffer_data(&indeces, gl::STATIC_DRAW);

    let loc_attrib = shader.get_attrib_location("loc").expect("attribute not found");
    set_attribute!(vertex_array, loc_attrib, Vertex::0);
    let tex_attrib = shader.get_attrib_location("vertTexCoords").expect("attribute not found");
    set_attribute!(vertex_array, tex_attrib, Vertex::1);

    let texture = Texture::new();
    texture.set_wrapping(gl::REPEAT);
    texture.load("./content/textures/tetosphere.png").expect("unable to load texture");
    shader.set_int_uniform("texture0", 0).expect("unable to set uniform");

    let camera: Camera = Camera::Orthographic(OrthographicCamera::new(720.0, 640.0));
    // let camera: Camera = Camera::Perspective(PerspectiveCamera::new(720.0, 640.0));

    let mut render_target = Renderer::new();
    render_target.add_request(vertex_array, shader, texture);


    // start main game loop
    unsafe {
        enable_debug_mode();
        neb::neb_init();

        set_error_pipe_message!("Failed to link module");
        neb::link_file("./content/scripts/test.neb");

        set_error_pipe_message!("Runtime error");
        window_event_loop(handle, render_target, &camera);
    }
}
