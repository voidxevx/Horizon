use crate::tools::math::vector::*;
// use std::ops::Mul;

#[allow(unused)]
pub trait Matrixable {
    fn get(&self, row: usize, col: usize) -> &f32;
    fn row(&self, col: usize) -> Vector;
    fn col(&self, row: usize) -> Vector;
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Matrix {
    SquareLength4(Mat4),
    SquareLength3(Mat3),
    SquareLength2(Mat2),
}

impl Matrixable for Matrix {
    fn get(&self, row: usize, col: usize) -> &f32 {
        match self {
            Self::SquareLength2(mat2) => {
                mat2.get(row, col)
            }
            Self::SquareLength3(mat3) => {
                mat3.get(row, col)
            }
            Self::SquareLength4(mat4) => {
                mat4.get(row, col)
            }
        }
    }

    fn row(&self, col: usize) -> Vector {
        match self {
            Self::SquareLength2(mat2) => {
                mat2.row(col)
            }
            Self::SquareLength3(mat3) => {
                mat3.row(col)
            }
            Self::SquareLength4(mat4) => {
                mat4.row(col)
            }
        }
    }

    fn col(&self, row: usize) -> Vector {
        match self {
            Self::SquareLength2(mat2) => {
                mat2.col(row)
            }
            Self::SquareLength3(mat3) => {
                mat3.col(row)
            }
            Self::SquareLength4(mat4) => {
                mat4.col(row)
            }
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub data: [f32; 16]
}

#[allow(unused)]
impl Mat4 {
    pub fn new (vals: [f32; 16]) -> Self {
        Self {
            data: vals
        }
    }

    pub fn from(mat: Matrix) -> Self {
        match mat {
            Matrix::SquareLength2(mat2) => 
                Mat4::new([
                    mat2.data[0].clone(), mat2.data[1].clone(), 0.0, 0.0,
                    mat2.data[2].clone(), mat2.data[3].clone(), 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ]),
            Matrix::SquareLength3(mat3) => 
                Mat4::new([
                    mat3.data[0].clone(), mat3.data[1].clone(), mat3.data[2].clone(), 0.0,
                    mat3.data[3].clone(), mat3.data[4].clone(), mat3.data[5].clone(), 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ]),
            Matrix::SquareLength4(mat4) => mat4.clone(),
        }
    }
}

impl Matrixable for Mat4 {
    fn get(&self, row: usize, col: usize) -> &f32 {
        match self.data.get(col * 4 + row) {
            Some(index) => {
                index
            }
            _ => {
                &0.0
            }
        }
    }

    fn row(&self, col: usize) -> Vector {
        Vector::Length4(
            Vec4::new([
                self.data[col * 4]    .clone(),
                self.data[col * 4 + 1].clone(),
                self.data[col * 4 + 2].clone(),
                self.data[col * 4 + 3].clone(),
            ])
        )
    }

    fn col(&self, row: usize) -> Vector {
        Vector::Length4(
            Vec4::new([
                self.data[row]     .clone(),
                self.data[row + 4] .clone(),
                self.data[row + 8] .clone(),
                self.data[row + 12].clone(),
            ])
        )
    }
}

// impl Mul for Mat4 {
//     type Output = Self;
//     fn mul(self, other: Self) -> Output {
//         // TODO: matrix multilipcation
//     }
// }

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub data: [f32; 9]
}

#[allow(unused)]
impl Mat3 {
    pub fn new (vals: [f32; 9]) -> Self {
        Self {
            data: vals
        }
    }

    pub fn from(mat: Matrix) -> Self {
        match mat {
            Matrix::SquareLength2(mat2) => Mat3::new([
                mat2.data[0].clone(), mat2.data[1].clone(), 0.0,
                mat2.data[2].clone(), mat2.data[3].clone(), 0.0,
                0.0, 0.0, 1.0,
            ]),
            Matrix::SquareLength3(mat3) => mat3.clone(),
            Matrix::SquareLength4(mat4) => Mat3::new([
                mat4.data[0].clone(), mat4.data[1].clone(), mat4.data[3].clone(),
                mat4.data[5].clone(), mat4.data[6].clone(), mat4.data[7].clone(),
                mat4.data[9].clone(), mat4.data[10].clone(), mat4.data[11].clone(),
            ])
        }
    }
}

impl Matrixable for Mat3 {
    fn get(&self, row: usize, col: usize) -> &f32 {
        match self.data.get(col * 3 + row)
        {
            Some(index) => {
                index
            }
            _ => {
                &0.0
            }
        }
    }

    fn row(&self, col: usize) -> Vector {
        Vector::Length3(
            Vec3::new([
                self.data[col * 3]    .clone(),
                self.data[col * 3 + 1].clone(),
                self.data[col * 3 + 2].clone(),
            ])
        )
    }

    fn col(&self, row: usize) -> Vector {
        Vector::Length3(
            Vec3::new([
                self.data[row]    .clone(),
                self.data[row + 3].clone(),
                self.data[row + 6].clone(),
            ])
        )
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Mat2 {
    pub data: [f32; 4]
}

#[allow(unused)]
impl Mat2 {
    pub fn new(vals: [f32; 4]) -> Self {
        Self {
            data: vals
        }
    }

    pub fn from(mat: Matrix) -> Self {
        match mat {
            Matrix::SquareLength2(mat2) => mat2.clone(),
            Matrix::SquareLength3(mat3) => Mat2::new([
                mat3.data[0].clone(), mat3.data[1].clone(),
                mat3.data[3].clone(), mat3.data[4].clone(),
            ]),
            Matrix::SquareLength4(mat4) => Mat2::new([
                mat4.data[0].clone(), mat4.data[1].clone(),
                mat4.data[4].clone(), mat4.data[5].clone(),
            ])
        }
    }
}

impl Matrixable for Mat2 {
    fn get(&self, row: usize, col: usize) -> &f32 {
        match self.data.get(col * 2 + row) {
            Some(index) => {
                index
            }
            _ => {
                &0.0
            }
        }
    }

    fn row(&self, col: usize) -> Vector {
        Vector::Length2(
            Vec2::new([
                self.data[col * 2]    .clone(),
                self.data[col * 2 + 1].clone(),
            ])
        )
    }

    fn col(&self, row: usize) -> Vector {
        Vector::Length2(
            Vec2::new([
                self.data[row]    .clone(),
                self.data[row + 2].clone(),
            ])
        )
    }
}