use std::ops::{Add, Sub, Mul, Div};
use num::{Num, ToPrimitive};
use std::fmt::Debug;

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

    pub fn len(&self) -> usize {
        M*N
    }
}