use crate::tools::math::vector::*;

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