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

impl<T> std::ops::Add for Complex<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.re + other.re, self.im + other.im)
    }
}

impl<T> std::ops::Sub for Complex<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.re - other.re, self.im - other.im)
    }
}

impl<T> std::ops::Mul for Complex<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + Copy
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}

impl<T> std::ops::Mul<T> for Complex<T>
where
    T: std::ops::Mul<Output = T> + Copy
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self::new(self.re * other, self.im * other)
    }
}

// impl<T> std::ops::Mul<Complex<T>> for T
// where
//     T: std::ops::Mul<Output = T> + Copy
// {
//     type Output = Complex<T>;

//     fn mul(self, other: Complex<T>) -> Complex<T> {
//         Complex::new(self * other.re, self * other.im)
//     }
// }

impl<T> std::ops::Div for Complex<T>
where
    T: std::ops::Div<Output = T> + std::ops::Add<Output = T> + std::ops::Mul<Output = T>  + std::ops::Sub<Output = T> + Copy
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denominator = other.re * other.re + other.im * other.im;
        Self::new(
            (self.re * other.re + self.im * other.im) / denominator,
            (self.im * other.re - self.re * other.im) / denominator,
        )
    }
}

impl<T> std::ops::BitXor<i32> for Complex<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + Copy
{
    type Output = Self;

    fn bitxor(self, power: i32) -> Self {
        let mut result = self;
        // naive greedy
        for _ in 1..power {
            result = result * self;
        }
        result
    }
}