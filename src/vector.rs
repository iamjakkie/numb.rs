use std::ops::{Add, Sub, Mul, Div};
use num::{Num, Float};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T> {
    elements: Vec<T>,
}

impl<T> Vector<T> 
where
    T: Num + Copy,
{
    pub fn new(elements: Vec<T>) -> Self {
        Self { elements }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<T: Float> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let elements = self.elements.iter().zip(other.elements.iter())
            .map(|(a, b)| *a + *b)
            .collect();
        Self::new(elements)
    }
}


impl<T: Float> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(self.len(), other.len(), "Vectors must have the same length");
        let elements = self
            .elements
            .iter()
            .zip(other.elements.iter())
            .map(|(a, b)| *a - *b)
            .collect();
        Self::new(elements)
    }
}

impl<T, U> Mul<U> for Vector<T>
where
    T: Num + Copy + From<U>, 
    U: Num + Copy,           
{
    type Output = Self;

    fn mul(self, scalar: U) -> Self {
        let scalar_t = T::from(scalar); // Convert scalar to vector's type
        let elements = self.elements.iter().map(|&x| x * scalar_t).collect();
        Self::new(elements)
    }
}