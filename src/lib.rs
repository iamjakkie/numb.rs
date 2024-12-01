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
    ($($($x:expr),+);+ $(,)?) => {{
        const ROWS: usize = [$([$($x),*]),*].len();
        const COLS: usize = [$($($x),*),*].len() / ROWS;
        Matrix::<_, ROWS, COLS>::new([
            [$($($x),*),*]
        ])
    }};
    ($value:expr; $rows:expr, $cols:expr) => {
        Matrix::<_, $rows, $cols>::new([[$value; $cols]; $rows])
    };
    () => {{
        const ROWS: usize = 0;
        const COLS: usize = 0;
        Matrix::<i32, ROWS, COLS>::new([[]])
    }};
    () => {
        
    };
}