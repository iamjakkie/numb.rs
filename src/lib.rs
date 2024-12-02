mod matrix;
pub use matrix::{Matrix, RowVector, ColumnVector};

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
