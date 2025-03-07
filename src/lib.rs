// mod complex;
mod geometry;
mod matrix;
mod decomposition;

// pub use complex::Complex;
pub use num::Complex;
pub use matrix::{ColumnVector, Matrix, RowVector};
pub use decomposition::{PartialPivLu};
// #[macro_export]
// macro_rules! vector {
//     ($value:expr; $size:expr) => {
//         Vector::<_, $size>::new([$value; $size])
//     };
//     ($($x:expr),+ $(,)?) => {{
//         const SIZE: usize = [$($x),*].len();
//         Vector::<_, SIZE>::new([$($x),*])
//     }};
//     () => {{
//         const SIZE: usize = 0;
//         Vector::<i32, SIZE>::new([])
//     }};
// }
// #[macro_export]
// macro_rules! sqrt_vector {
//     ($($x:expr),+ $(,)?) => {
//         Vector::<f64, { [$($x),*].len() }>::new([
//             $(($x as f64).sqrt()),*
//         ])
//     };
// }

#[macro_export]
macro_rules! vector {
    // Case 1: Empty row vector
    ([]) => {{
        $crate::RowVector::new($crate::Matrix::<i32, 1, 0>::new([[]])) // 1 row, 0 columns
    }};
    // Case 2: Row vector filled with a repeated value
    ([ $value:expr ; $cols:expr ]) => {{
        $crate::RowVector::new($crate::Matrix::<_, 1, $cols>::new([[$value; $cols]])) // Row vector: 1 row, $cols columns
    }};
    // Case 3: Column vector filled with a repeated value
    ([ $value:expr ]; $rows:expr) => {{
        $crate::ColumnVector::new($crate::Matrix::<_, $rows, 1>::new([[ $value ]; $rows])) // Column vector: $rows rows, 1 column
    }};
    // Case 4: Explicit row vector
    ([ $($x:expr),+ $(,)? ]) => {{
        const SIZE: usize = [$($x),*].len();
        $crate::RowVector::new($crate::Matrix::<_, 1, SIZE>::new([[$($x),*]])) // Row vector with explicit elements
    }};
    // TODO: hardcode macro up to ?? elements, otherwise use constructor
    // ([ $x:expr, $y:expr ]) => {{
    //     $crate::RowVector::new($crate::Matrix::<_, 1, 2>::new([[ $x, $y ]])) // Row vector with explicit elements
    // }};

    // ([ $($arg:tt), ]) => {{
    //     let SIZE: usize = [$($arg),*].len();
    //     $crate::RowVector::new($crate::Matrix::<_, 1, SIZE>::new([[$($arg),*]])) // Row vector with explicit elements
    // }};
    // Case 5: Explicit column vector
    ($( [ $x:expr ] ),+ $(,)?) => {{
        const SIZE: usize = [$([$x]),*].len();
        $crate::ColumnVector::new($crate::Matrix::<_, SIZE, 1>::new([
            $([$x]),* // Column vector with explicit elements
        ]))
    }};
}

#[macro_export]
macro_rules! sqrt_vector {
    ($($x:expr),+ $(,)?) => {
        Matrix::<f64, 1, { [$($x),*].len() }>::new([[
            $(($x as f64).sqrt()),*
        ]]) // Row vector representation with square roots
    };
    ($value:expr; $N_size:expr, $M_size:expr) => {
        Matrix::<_, $N_size, $M_size>::new([[$value; $N_size]; $M_size])
    };
}

#[macro_export]
macro_rules! matrix {
    ($([$($x:expr),+ $(,)?]),+ $(,)?) => {{
        let elements = [
            $([$($x),*],)*
        ];
        Matrix::new(elements)
    }};
    ($value:expr; $N_size:expr, $M_size:expr) => {
        Matrix::<_, $N_size, $M_size>::new([[$value; $N_size]; $M_size])
    };
    () => {{
        const M: usize = 0;
        const N: usize = 0;
        Matrix::<i32, M, N>::new([])
    }};
    ([] $(,)? ) => {{
        $crate::Matrix::<_, 1, 0>::new([[]]) // Explicitly define 1 row, 0 columns
    }};
}

#[macro_export]
macro_rules! identity_matrix {
    ($type:ty, $size:expr) => {{
        let mut elements = [[<$type as num_traits::Zero>::zero(); $size]; $size];
        for i in 0..$size {
            elements[i][i] = <$type as num_traits::One>::one();
        }
        Matrix::<$type, $size, $size>::new(elements)
    }};
}

#[macro_export]
macro_rules! rotation_2d {
    ($angle:expr) => {{
        let cos = $angle.cos();
        let sin = $angle.sin();
        $crate::Matrix::new([[cos, -sin], [sin, cos]])
    }};
}

#[macro_export]
macro_rules! rotation_3d {
    ($angle:expr, $axis:expr) => {{
        $crate::generate_rotation_3d($angle, $axis)
    }};
}

#[macro_export]
macro_rules! complex {
    ($re:tt + $im:tt i) => {
        $crate::Complex::<f64>::new($re as _, $im as _)
    };
    ($re:tt - $im:tt i) => {
        $crate::Complex::<f64>::new($re as _, -$im as _)
    };
    ($re:tt + $im:tt) => {
        $crate::Complex::<f64>::new($re as _, $im as _)
    };
    ($re:tt - $im:tt) => {
        $crate::Complex::<f64>::new($re as _, -$im as _)
    };
    ($re:expr, $im:expr) => {
        $crate::Complex::<f64>::new($re as f64, $im as f64)
    };
    ($re:tt) => {
        $crate::Complex::<f64>::new($re as f64, 0.0)
    };
    (+ $im:tt i) => {
        $crate::Complex::<f64>::new(0.0, $im as f64)
    };
    (- $im:tt i) => {
        $crate::Complex::<f64>::new(0.0, -($im as f64))
    };
}
