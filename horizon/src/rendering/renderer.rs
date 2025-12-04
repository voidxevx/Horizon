use crate::rendering::mesh_data::{shader::ShaderProgram, texture::Texture, vertex_array::VertexArray};
use std::ptr;

struct RenderRequest {
    va: VertexArray,
    shader: ShaderProgram,
    texture: Texture,
}

#[allow(unused)]
pub struct Renderer {
    render_requests: Vec<RenderRequest>
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

    pub fn draw_requests(&self) {
        unsafe
        {
            for req in &self.render_requests {
                req.texture.activate(gl::TEXTURE0);
                req.shader.apply();
                req.va.bind();
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }
        }
    }
}