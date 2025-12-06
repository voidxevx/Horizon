use crate::tools::math::matrix::{Mat2, Mat3, Mat4, Matrix};
use crate::tools::math::vector::Vector;

#[allow(unused)]
pub fn translation_matrix(vec: Vector) -> Option<Matrix> {
    match vec {
        Vector::Length2(vec2) => 
        {
            Some(Matrix::SquareLength3(Mat3::new([
                1.0, 0.0, vec2.data[0].clone(),
                0.0, 1.0, vec2.data[1].clone(),
                0.0, 0.0, 1.0,
            ])))
        },
        Vector::Length3(vec3) => {
            Some(Matrix::SquareLength4(Mat4::new([
                1.0, 0.0, 0.0, vec3.data[0].clone(),
                0.0, 1.0, 0.0, vec3.data[1].clone(),
                0.0, 0.0, 1.0, vec3.data[2].clone(),
                0.0, 0.0, 0.0, 1.0,
            ])))
        },
        Vector::Length4(vec4) => {
            None
        }
    }
}

#[allow(unused)]
pub fn scalar_matrix(vec: Vector) -> Matrix {
    match vec {
        Vector::Length2(vec2) => {
            Matrix::SquareLength2(Mat2::new([
                vec2.data[0].clone(), 0.0,
                0.0, vec2.data[1].clone(),
            ]))
        },
        Vector::Length3(vec3) => {
            Matrix::SquareLength3(Mat3::new([
               vec3.data[0].clone(), 0.0, 0.0,
               0.0, vec3.data[1].clone(), 0.0,
               0.0, 0.0, vec3.data[2].clone() 
            ]))
        },
        Vector::Length4(vec4) => {
            Matrix::SquareLength4(Mat4::new([
                vec4.data[0].clone(), 0.0, 0.0, 0.0,
                0.0, vec4.data[1].clone(), 0.0, 0.0,
                0.0, 0.0, vec4.data[2].clone(), 0.0,
                0.0, 0.0, 0.0, vec4.data[3].clone(),
            ]))
        }
    }
}

#[allow(unused)]
pub fn rotation_mat2_euler(angle: f32) -> Matrix {
    Matrix::SquareLength2(Mat2::new([
        f32::cos(angle), -f32::sin(angle),
        f32::sin(angle),  f32::cos(angle),
    ]))
}

#[allow(unused)]
pub fn rotation_mat3_euler_x(angle: f32) -> Matrix {
    Matrix::SquareLength3(Mat3::new([
        1.0, 0.0, 0.0,
        0.0, f32::cos(angle), -f32::sin(angle),
        0.0, f32::sin(angle), f32::cos(angle),
    ]))
}

#[allow(unused)]
pub fn rotation_mat3_euler_y(angle: f32) -> Matrix {
    Matrix::SquareLength3(Mat3::new([
        f32::cos(angle), 0.0, f32::sin(angle),
        0.0, 1.0, 0.0,
        -f32::sin(angle), 0.0, f32::cos(angle),
    ]))
}

#[allow(unused)]
pub fn rotation_mat3_euler_z(angle: f32) -> Matrix {
    Matrix::SquareLength3(Mat3::new([
        f32::cos(angle), -f32::sin(angle), 0.0,
        f32::sin(angle), f32::cos(angle), 0.0,
        0.0, 0.0, 1.0,
    ]))
}

#[allow(unused)]
pub fn rotation_mat4_euler_x(angle: f32) -> Matrix {
    Matrix::SquareLength4(Mat4::new([
        1.0, 0.0, 0.0, 0.0,
        0.0, f32::cos(angle), -f32::sin(angle), 0.0,
        0.0, f32::sin(angle), f32::cos(angle), 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]))
}

#[allow(unused)]
pub fn rotation_mat4_euler_y(angle: f32) -> Matrix {
    Matrix::SquareLength4(Mat4::new([
        f32::cos(angle), 0.0, f32::sin(angle), 0.0,
        0.0, 1.0, 0.0, 0.0,
        -f32::sin(angle), 0.0, f32::cos(angle), 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]))
}

#[allow(unused)]
pub fn rotation_mat4_euler_z(angle: f32) -> Matrix {
    Matrix::SquareLength4(Mat4::new([
        f32::cos(angle), -f32::sin(angle), 0.0, 0.0,
        f32::sin(angle), f32::cos(angle), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]))
}

#[allow(unused)]
pub fn orthographic_matrix(right: f32, left: f32, top: f32, bottom: f32, near: f32, far: f32) -> Matrix {
    Matrix::SquareLength4(Mat4::new([
        2.0 / (right - left), 0.0, 0.0, -(right + left) / (right - left),
        0.0, 2.0 / (top -bottom), 0.0, -(top + bottom) / (top - bottom),
        0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near),
        0.0, 0.0, 0.0, 1.0,
    ]))
}

// #[allow(unused)]
// pub fn perspective_matrix(far: f32, near: f32, aspect_ration: f32, fov: f32) -> Matrix {
//     Matrix::SquareLength4(Mat4::new([
//         1.0 / (aspect_ration * (fov / 2.0).tan()), 0.0, 0.0, 0.0,
//         0.0, 1.0 / (fov / 2.0).tan(), 0.0, 0.0,
//         0.0, 0.0, -(far + near) / (far - near), -2.0 * (far * near) / (far - near),
//         0.0, 0.0, -1.0, 0.0,
//     ]))
// }

#[allow(unused)]
pub fn perspective_matrix(right: f32, left: f32, top: f32, bottom: f32, near: f32, far: f32) -> Matrix {
    Matrix::SquareLength4(Mat4::new([
        2.0 * near / (right - left), 0.0, (right + left) / (right - left), 0.0,
        0.0, 2.0 * near / (top - bottom), (top + bottom) / (top - bottom), 0.0,
        0.0, 0.0, -(far + near) / (far - near), -2.0 * far * near / (far - near),
        0.0, 0.0, -1.0, 0.0,
    ]))
}