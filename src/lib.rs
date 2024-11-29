mod vector;

pub use vector::Vector;

#[macro_export]
macro_rules! vector {
    ($value:expr; $length:expr) => {
        Vector::filled($value, $length)
    };
    [] => {
        Vector::<f64>::new(Vec::new())
    };
    ($($elem:expr),* $(,)?) => {
        Vector::new(vec![$($elem),*])
    };
}