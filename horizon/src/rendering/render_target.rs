use std::sync::Arc;

use crate::rendering::{camera::{Camera, OrthographicCamera, PerspectiveCamera}, material::{self, Material, MaterialInstance, instance_material}, mesh_data::{shader::ShaderUniform, vertex_array::VertexArray}, renderer::Renderer};

pub const RENDER_TARGET_ORTHOGRAPHIC: i32 = 0b1 as i32;
pub const RENDER_TARGET_PERSPECTIVE: i32 = 0b10 as i32;

pub struct MeshBuilder {
    va: Arc<VertexArray>,
    mat_inst: MaterialInstance,
}

impl MeshBuilder {
    pub fn new(va: Arc<VertexArray>, mat: Arc<Material>) -> Self {
        let inst = instance_material(mat);
        Self {
            va: va,
            mat_inst: inst
        }
    }

    pub fn uniform(mut self, name: &str, val: ShaderUniform) -> Self {
        self.mat_inst.set_uniform(&String::from(name), val);
        self
    }

    pub fn attach(self, target: &mut RenderTarget) {
        target.add_mesh(self);
    }
}

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

    pub fn capture(&mut self) {
        self.renderer.draw_requests(&self.camera);
    }

    pub fn add_mesh(&mut self, mesh: MeshBuilder) {
        self.renderer.add_request(mesh.va, mesh.mat_inst);
    }

}