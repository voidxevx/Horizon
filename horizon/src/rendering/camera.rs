use crate::tools::math::{matrix::Matrix, transforms::orthographic_matrix};

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
            )
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

    pub fn get_view_matrix(&self) -> &Matrix {
        &self.view_matrix
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct PerspectiveCamera {

}