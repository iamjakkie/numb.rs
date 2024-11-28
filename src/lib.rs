mod vector;

pub use vector::Vector;

#[macro_export]
macro_rules! vector {
    [] => {
        Vector::<f64>::new(Vec::new())
    };
    ($($elem:expr),* $(,)?) => {
        Vector::new(vec![$($elem),*])
    };
}