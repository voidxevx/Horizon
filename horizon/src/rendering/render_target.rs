use crate::rendering::{camera::{Camera, OrthographicCamera, PerspectiveCamera}, mesh_data::{shader::ShaderProgram, texture::Texture, vertex_array::VertexArray}, renderer::Renderer};

pub const RENDER_TARGET_ORTHOGRAPHIC: i32 = 0b1 as i32;
pub const RENDER_TARGET_PERSPECTIVE: i32 = 0b10 as i32;

#[allow(unused)]
pub struct RenderTarget {
    camera: Camera,
    renderer: Renderer,
}

#[allow(unused)]
impl RenderTarget {
    pub fn new(camera_type: i32) -> Self {
        let mut camera: Camera = Camera::None;

        match camera_type {
            RENDER_TARGET_ORTHOGRAPHIC => {
                camera = Camera::Orthographic(OrthographicCamera::new(720.0, 640.0));
            },
            RENDER_TARGET_PERSPECTIVE => {
                camera = Camera::Perspective(PerspectiveCamera::new(720.0, 640.0));
            },
            _ => ()
        }

        let renderer: Renderer = Renderer::new();

        Self { 
            camera,
            renderer,
        }
    }

    pub fn resize_capture(&mut self, screen_width: f32, screen_height: f32) {
        match &mut self.camera {
            Camera::Orthographic(ortho) => {
                ortho.update_view_matrix(screen_width, screen_height);
            },
            Camera::Perspective(persp) => {
                persp.update_view_matrix(screen_width, screen_height);
            },
            _ => ()
        }
    }

    pub fn capture(&self) {
        self.renderer.draw_requests(&self.camera);
    }

    pub fn add_draw_request(&mut self, va: VertexArray, shader: ShaderProgram, texture: Texture) {
        self.renderer.add_request(va, shader, texture);
    }

}