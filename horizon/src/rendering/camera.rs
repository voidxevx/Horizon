use crate::tools::math::{matrix::Matrix, transforms::{orthographic_matrix, perspective_matrix, translation_matrix}, vector::{Vec2, Vec3, Vector}};

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Camera {
    Orthographic(OrthographicCamera),
    Perspective(PerspectiveCamera),
    None,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct OrthographicCamera {
    view_matrix: Matrix,
    camera_position: Vector,
    camera_rotation: f32,
}

#[allow(unused)]
impl OrthographicCamera {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Self {
            view_matrix: orthographic_matrix(
                window_width,
                0.0,
                window_height,
                0.0,
                0.0,
                1.0
            ),
            camera_position: Vector::Length3(Vec3::new([0.0, 0.0, 0.0])),
            camera_rotation: 0.0,
        }
    }

    pub fn update_view_matrix(&mut self, width: f32, height: f32) {
        self.view_matrix = orthographic_matrix(
            0.0,
            width,
            0.0,
            height,
            0.0,
            1.0
        );
    }

    pub fn get_projection_matrix(&self) -> Matrix {
        translation_matrix(self.camera_position)
            .expect("bad camera position")
    }

    pub fn get_view_matrix(&self) -> &Matrix {
        &self.view_matrix
    }

    pub fn change_camera_position(&mut self, dx: f32, dy: f32) {
        self.camera_position = self.camera_position + Vector::Length3(Vec3::new([dx, dy, 0.0]));
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct PerspectiveCamera {
    view_matrix: Matrix,
    camera_position: Vector,
    camera_rotation: Vector,
}

#[allow(unused)]
impl PerspectiveCamera {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Self {
            view_matrix: perspective_matrix(
                0.0,
                window_width,
                0.0,
                window_height,
                0.0,
                1.0,

            ),
            camera_position: Vector::Length3(Vec3::new([0.0, 0.0, 0.0])),
            camera_rotation: Vector::Length3(Vec3::new([0.0, 0.0, 0.0]))
        }
    }

    pub fn update_view_matrix(&mut self, width: f32, height: f32) {
        self.view_matrix = perspective_matrix(
            0.0,
            width,
            0.0,
            height,
            0.0,
            1.0,
        )
    }

    pub fn get_view_matrix(&self) -> &Matrix {
        &self.view_matrix
    }
}