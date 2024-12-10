// TO BE MOVED TO A SEPARATE CRATE

use std::ops::Neg;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }

    pub fn conjugate(&self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        Self::new(self.re, -self.im)
    }

    // pub fn
}
