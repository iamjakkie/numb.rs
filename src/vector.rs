use std::ops::{Add, Sub, Mul, Div};
use num::{Num, Float, ToPrimitive};

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

    pub fn zeros(length: usize) -> Self {
        Self {
            elements: vec![T::zero(); length],
        }
    }

    pub fn ones(length: usize) -> Self {
        Self {
            elements: vec![T::one(); length],
        }
    }

    pub fn filled(value: T, length: usize) -> Self {
        Self {
            elements: vec![value; length],
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<T> Vector<T>
where
    T: Num + Copy + ToPrimitive,
{
    pub fn magnitude(&self) -> f64 {
        self.elements
            .iter()
            .map(|&x| {
                let x_f64 = x.to_f64().expect("Conversion to f64 failed");
                x_f64 * x_f64
            })
            .sum::<f64>()
            .sqrt()
    }
}

impl<T> Add for Vector<T> 
where
    T: Num + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let elements = self.elements.iter().zip(other.elements.iter())
            .map(|(a, b)| *a + *b)
            .collect();
        Self::new(elements)
    }
}


impl<T> Sub for Vector<T> 
where
    T: Num + Copy,
{
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

impl<T, U> Mul<Vector<U>> for Vector<T>
where
    T: Num + Copy + From<U>, 
    U: Num + Copy,
{
    type Output = T;

    fn mul(self, other: Vector<U>) -> T {
        assert_eq!(self.len(), other.len(), "Vectors must have the same length");
        self.elements
            .iter()
            .zip(other.elements.iter())
            .map(|(&a, &b)| a * T::from(b)) // Convert `b` to type `T`
            .fold(T::zero(), |acc, x| acc + x) // Sum the results
    }
}
