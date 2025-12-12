use crate::{rendering::{camera::Camera, material::{MaterialInstance}, mesh_data::{shader::{ShaderUniform}, vertex_array::VertexArray}}, tools::math::matrix::{Mat4, Matrix}};
use std::ptr;

struct RenderRequest {
    va: VertexArray,
    material: MaterialInstance,
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

    pub fn add_request(&mut self, va: VertexArray, material: MaterialInstance) {
        self.render_requests.push(RenderRequest { 
            va: va, 
            material: material
        });
    }

    pub fn draw_requests(&mut self, camera: &Camera) {
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

            for req in &mut self.render_requests {
                req.va.bind();
                req.material.apply();
                req.material.set_uniform(&String::from("viewMatrix"), ShaderUniform::MatrixUniform(projection_matrix).clone());

                match camera {
                    Camera::Orthographic(ortho) => {
                        req.material.set_uniform(&String::from("viewMatrix"), ShaderUniform::MatrixUniform(*ortho.get_view_matrix()).clone());
                    }
                    _ => ()
                }

                // Draw Call
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }
        }
    }
}