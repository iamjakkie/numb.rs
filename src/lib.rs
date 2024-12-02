
mod vector;
mod matrix;

pub use vector::Vector;
pub use matrix::Matrix;

#[macro_export]
macro_rules! vector {
    ($value:expr; $size:expr) => {
        Vector::<_, $size>::new([$value; $size])
    };
    ($($x:expr),+ $(,)?) => {{
        const SIZE: usize = [$($x),*].len();
        Vector::<_, SIZE>::new([$($x),*])
    }};
    () => {{
        const SIZE: usize = 0;
        Vector::<i32, SIZE>::new([])
    }};
}
#[macro_export]
macro_rules! sqrt_vector {
    ($($x:expr),+ $(,)?) => {
        Vector::<f64, { [$($x),*].len() }>::new([
            $(($x as f64).sqrt()),*
        ])
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
    }}
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
