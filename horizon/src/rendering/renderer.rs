use crate::{rendering::{camera::Camera, mesh_data::{shader::ShaderProgram, texture::Texture, vertex_array::VertexArray}}, tools::math::matrix::{Mat4, Matrix}};
use std::ptr;

struct RenderRequest {
    va: VertexArray,
    shader: ShaderProgram,
    texture: Texture,
}

#[allow(unused)]
pub struct Renderer {
    render_requests: Vec<RenderRequest>,
}


#[allow(unused)]
impl Renderer {
    pub fn new() -> Self {
        Self {
            render_requests: Vec::new(),
        }
    }

    pub fn add_request(&mut self, va: VertexArray, shader: ShaderProgram, texture: Texture) {
        self.render_requests.push(RenderRequest { 
            va: va, 
            shader:  shader,
            texture: texture,
        });
    }

    pub fn draw_requests(&self, camera: &Camera) {
        unsafe
        {
            let mut projection_matrix: Matrix = Matrix::SquareLength4(Mat4::new([
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ]));

            match camera {
                Camera::Orthographic(ortho) => {
                    projection_matrix = ortho.get_projection_matrix();
                },
                _ => ()
            }

            for req in &self.render_requests {
                req.texture.activate(gl::TEXTURE0);
                req.va.bind();
                req.shader.apply();
                req.shader.set_mat_uniform("projectionMatrix", projection_matrix);

                match camera {
                    Camera::Orthographic(ortho) => {
                        req.shader.set_mat_uniform("viewMatrix", *ortho.get_view_matrix());
                    }
                    _ => ()
                }

                // Draw Call
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }
        }
    }
}