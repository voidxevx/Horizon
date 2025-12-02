use crate::{rendering::mesh_data::{buffer::Buffer, shader::*, vertex_array::VertexArray}, tools::math::vector::Vec3};
use gl::types::*;

#[allow(unused)]
use crate::{
    rendering::application::*,
    rendering::mesh_data::{buffer},
};

mod tools {
    pub mod math{
        pub mod vector;
    }
}

mod rendering {
    pub mod application;
    pub mod mesh_data {
        pub mod buffer;
        pub mod shader;
        pub mod vertex_array;
    }
}

struct Vertex(Vec3);

fn main() {
    let app: App = App::create(
        WindowProps::new(String::from("Horizon"), [0.0, 0.0, 0.0, 1.0])
    );

    let _test_shader: ShaderProgram = generate_shader("./content/shaders/default.shader")
        .expect("Error creating shader.");

    let triangle_mesh: [Vertex; 3] = [
        Vertex(Vec3::new([1.0, 1.0, 1.0])),
        Vertex(Vec3::new([0.0, 1.0, 1.0])),
        Vertex(Vec3::new([1.0, 0.0, 1.0]))
    ];

    let triangle_indecis: [GLuint; 3] = [
        0, 1, 2
    ];

    let test_buffer: Buffer = Buffer::new(gl::ARRAY_BUFFER);
    let index_buffer: Buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
    test_buffer.buffer_data(&triangle_mesh, gl::STATIC_DRAW);
    index_buffer.buffer_data(&triangle_indecis, gl::STATIC_DRAW);

    let vertex_array: VertexArray = VertexArray::new(test_buffer, index_buffer);

    vertex_array.bind();

    app.main_loop();
}
