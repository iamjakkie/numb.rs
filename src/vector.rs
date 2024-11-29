use std::ops::{Add, Sub, Mul, Div};
use num::{Num, Float, ToPrimitive};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Vector<T, const N: usize> {
    pub elements: [T;N],
}

pub struct Scalar<U>(pub U);

impl<U> From<U> for Scalar<U> {
    fn from(value: U) -> Self {
        Scalar(value)
    }
}

impl<T, const N: usize> Vector<T, N> 
where
    T: Num + Copy,
{
    pub fn new(elements: [T;N]) -> Self {
        Self { elements }
    }

    pub fn zeros() -> Self {
        Self {
            elements: [T::zero(); N],
        }
    }

    pub fn ones() -> Self {
        Self {
            elements: [T::one(); N],
        }
    }

    pub fn filled(value: T) -> Self {
        Self {
            elements: [value; N],
        }
    }

    pub fn len(&self) -> usize {
        N
    }
}

// impl<T> Vector<T>
// where
//     T: Num + Copy + ToPrimitive,
// {
//     pub fn magnitude(&self) -> f64 {
//         self.elements
//             .iter()
//             .map(|&x| {
//                 let x_f64 = x.to_f64().expect("Conversion to f64 failed");
//                 x_f64 * x_f64
//             })
//             .sum::<f64>()
//             .sqrt()
//     }
// }

// impl<'a, T> Add for &'a Vector<T> 
// where
//     T: Num + Copy,
// {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         let elements = self.elements.iter().zip(other.elements.iter())
//             .map(|(a, b)| *a + *b)
//             .collect();
//         Self::new(elements)
//     }
// }


// impl<T> Sub for Vector<T> 
// where
//     T: Num + Copy,
// {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self {
//         assert_eq!(self.len(), other.len(), "Vectors must have the same length");
//         let elements = self
//             .elements
//             .iter()
//             .zip(other.elements.iter())
//             .map(|(a, b)| *a - *b)
//             .collect();
//         Self::new(elements)
//     }
// }

// impl<T, U> Mul<U> for Vector<T>
// where
//     T: Num + Copy + From<U>, 
//     U: Num + Copy,           
// {
//     type Output = Self;

//     fn mul(self, scalar: U) -> Self {
//         let scalar_t = T::from(scalar); // Convert scalar to vector's type
//         let elements = self.elements.iter().map(|&x| x * scalar_t).collect();
//         Self::new(elements)
//     }
// }

// impl<T, U> Mul<Vector<U>> for Vector<T>
// where
//     T: Num + Copy + From<U>, 
//     U: Num + Copy,
// {
//     type Output = T;

//     fn mul(self, other: Vector<U>) -> T {
//         assert_eq!(self.len(), other.len(), "Vectors must have the same length");
//         self.elements
//             .iter()
//             .zip(other.elements.iter())
//             .map(|(&a, &b)| a * T::from(b)) // Convert `b` to type `T`
//             .fold(T::zero(), |acc, x| acc + x) // Sum the results
//     }
// }

// impl<T, U> Mul<Vector<T>> for Scalar<U>
// where
//     T: Num + Copy + From<U>, 
//     U: Num + Copy,
// {
//     type Output = Vector<T>;

//     fn mul(self, vector: Vector<T>) -> Vector<T> {
//         let scalar_t = T::from(self.0); // Convert scalar to vector's type
//         let elements = vector.elements.iter().map(|&x| scalar_t * x).collect();
//         Vector::new(elements)
//     }
// }
