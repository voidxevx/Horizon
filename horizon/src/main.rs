use crate::tools::math::{matrix::{Mat3, Matrix}, vector::*};

mod tools {
    pub mod math {
        pub mod vector;
        pub mod matrix; 
    }
}

fn main() {

    let my_vec_a: Vector = Vector::Length4(Vec4::new([1.0, 2.0, 3.0, 4.0]));
    let my_vec_b: Vector = Vector::Length3(Vec3::new([4.0, 5.0, 6.0]));

    let my_mat: Matrix = Matrix::SquareLength3(Mat3::new([
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    ]));

    let res_vec: Vector = my_vec_a + my_vec_b;
    println!("resulting vector = {:?}", res_vec);
    println!("Matrix: {:?}", my_mat);
}
