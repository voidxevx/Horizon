use crate::{rendering::{camera::Camera, mesh_data::{shader::ShaderProgram, texture::Texture, vertex_array::VertexArray}}, tools::math::transforms::orthographic_matrix};
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

    pub fn draw_requests(&self, camera: Camera) {
        unsafe
        {
            for req in &self.render_requests {
                req.texture.activate(gl::TEXTURE0);
                req.shader.apply();
                match camera {
                    Camera::Orthographic(ortho) => {
                        req.shader.set_mat_uniform("viewMatrix", *ortho.get_view_matrix());
                    }
                    _ => ()
                }

                req.va.bind();
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }
        }
    }
}