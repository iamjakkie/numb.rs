mod vector;

pub use vector::Vector;

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