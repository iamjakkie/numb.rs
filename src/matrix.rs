use std::ops::{Add, Sub, Mul, BitXor};
use num::{Num, ToPrimitive};
use std::fmt::Debug;

use crate::Vector;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Matrix<T, const M: usize, const N: usize> 
where 
    T: Debug,
{
    pub elements: [[T;N];M],
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> 
where
    T: Debug + Num + Copy,
{
    pub fn new(elements: [[T;N];M]) -> Self {
        Self { elements }
    }

    pub fn zeros() -> Self {
        Self {
            elements: [[T::zero(); N]; M],
        }
    }

    pub fn ones() -> Self {
        Self {
            elements: [[T::one(); N]; M],
        }
    }

    pub fn filled(value: T) -> Self {
        Self {
            elements: [[value; N]; M],
        }
    }

    pub fn identity() -> Self {
        let mut elements = [[T::zero(); N]; M];
        for i in 0..M {
            elements[i][i] = T::one();
        }
        Self { elements }
    }

    pub fn transpose(self) -> Matrix<T, N, M> {
        let mut elements = [[T::zero(); M]; N];
        for i in 0..M {
            for j in 0..N {
                elements[j][i] = self.elements[i][j];
            }
        }
        Matrix::new(elements)
    }

    pub fn len(&self) -> usize {
        M*N
    }
}

impl<T, const M: usize, const N: usize> Add for Matrix<T, M, N>
where
    T: Debug + Num + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Ensure the matrices have the same dimensions
        assert_eq!(self.len(), other.len(), "Matrices must have the same dimensions");

        let elements = self
            .elements
            .iter()
            .zip(other.elements.iter())
            .map(|(row_a, row_b)| {
                row_a
                    .iter()
                    .zip(row_b.iter())
                    .map(|(a, b)| *a + *b)
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Row size mismatch")
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("Matrix size mismatch");

        Self::new(elements)
    }
}

impl<T, const M: usize, const N: usize> Sub for Matrix<T, M, N>
where
    T: Debug + Num + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // Ensure the matrices have the same dimensions
        assert_eq!(self.len(), other.len(), "Matrices must have the same dimensions");

        let elements = self
            .elements
            .iter()
            .zip(other.elements.iter())
            .map(|(row_a, row_b)| {
                row_a
                    .iter()
                    .zip(row_b.iter())
                    .map(|(a, b)| *a - *b)
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Row size mismatch")
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("Matrix size mismatch");

        Self::new(elements)
    }
}

impl<T, U, const M: usize, const N: usize> Mul<U> for Matrix<T, M, N>
where
    T: Debug + Num + Copy + From<U>,
    U: Num + Copy,
{
    type Output = Self;

    fn mul(self, scalar: U) -> Self {
        let scalar_t = T::from(scalar);
        let elements = self
            .elements
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&x| x * scalar_t)
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Row size mismatch")
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("Matrix size mismatch");
        Self::new(elements)
    }
}

// this is the basic multiplication, naive, greedy, slow and inefficient
impl<T, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for Matrix<T, M, N>
where
    T: Debug + Num + Copy,
{
    type Output = Matrix<T, M, P>;

    fn mul(self, other: Matrix<T, N, P>) -> Self::Output {
        // Ensure the matrices can be multiplied
        let mut result = [[T::zero(); P]; M];

        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    result[i][j] = result[i][j] + (self.elements[i][k] * other.elements[k][j]);
                }
            }
        }

        Matrix::new(result)
    }
}

impl<T, const N: usize> BitXor<u32> for Matrix<T, N, N>
where
    T: Debug + Num + Copy,
{
    type Output = Self;

    fn bitxor(self, exponent: u32) -> Self::Output {
        // If exponent is zero, return the identity matrix
        if exponent == 0 {
            let mut identity = Self::zeros();
            for i in 0..N {
                identity.elements[i][i] = T::one();
            }
            return identity;
        }

        // Initialize result to the identity matrix
        let mut result = Self::zeros();
        for i in 0..N {
            result.elements[i][i] = T::one();
        }

        // Start base as the current matrix
        let mut base = self;
        let mut exp = exponent;

        // Repeated squaring algorithm
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base.clone(); // Multiply when the current bit is set
            }
            base = base.clone() * base; // Square the base
            exp /= 2;
        }

        result
    }
}

impl<T, const M: usize, const N: usize> Mul<Vector<T, N>> for Matrix<T, M, N>
where
    T: Debug + Num + Copy,
{
    type Output = Vector<T, M>;

    fn mul(self, vector: Vector<T, N>) -> Self::Output {
        let mut result = [T::zero(); M];

        for i in 0..M {
            for j in 0..N {
                result[i] = result[i] + (self.elements[i][j] * vector.elements[j]);
            }
        }

        Vector::new(result)
    }
}