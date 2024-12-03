use crate::Matrix;
use num::{Num, Float};
use std::fmt::Debug;

pub enum Axis {
    X,
    Y,
    Z,
}

// Reflections placeholder

// Rotations
pub fn generate_rotation_2d<T: Float + Debug + Copy>(angle: f32) -> Matrix<f32, 2, 2> {
    let cos = angle.cos();
    let sin = angle.sin();
    Matrix::new([
        [cos, -sin],
        [sin, cos],
    ])
}


// pub fn generate_rotation_3d<T: Num + Debug + Copy>(angle: T, axis: Axis) -> Matrix<T, 3, 3> {
//     let cos = angle.cos();
//     let sin = angle.sin();
//     match axis {
//         Axis::X => Matrix::new([
//             [T::one(), T::zero(), T::zero()],
//             [T::zero(), cos, -sin],
//             [T::zero(), sin, cos],
//         ]),
//         Axis::Y => Matrix::new([
//             [cos, T::zero(), sin],
//             [T::zero(), T::one(), T::zero()],
//             [-sin, T::zero(), cos],
//         ]),
//         Axis::Z => Matrix::new([
//             [cos, -sin, T::zero()],
//             [sin, cos, T::zero()],
//             [T::zero(), T::zero(), T::one()],
//         ]),
//     }
// }