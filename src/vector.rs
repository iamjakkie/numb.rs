use std::ops::{Add, Sub, Mul, Div};
use num::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T> {
    elements: Vec<T>,
}

impl<T: Float> Vector<T> {
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