use std::ops::{Add, Sub, Mul};
use num::{Num, Float, ToPrimitive};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Vector<T, const N: usize> 
where 
    T: Debug,
{
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
    T: Debug + Num + Copy,
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

impl<T, const N: usize> Vector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive,
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

    pub fn angle_between(&self, other: &Self) -> f64 {
        assert_eq!(self.len(), other.len(), "Vectors must have the same length");

        let dot_product: f64 = self
            .elements
            .iter()
            .zip(other.elements.iter())
            .map(|(&a, &b)| {
                let a_f64 = a.to_f64().expect("Conversion to f64 failed");
                let b_f64 = b.to_f64().expect("Conversion to f64 failed");
                a_f64 * b_f64
            })
            .sum();

        let magnitude_a = self.magnitude();
        let magnitude_b = other.magnitude();

        // Compute cos(theta) and then use arccos
        let cos_theta = dot_product / (magnitude_a * magnitude_b);
        cos_theta.acos()
    }
}

impl<T, const N: usize> Add for Vector<T, N> 
where
    T: Debug + Num + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(self.len(), other.len(), "Vectors must have the same length");
        let elements = self
            .elements
            .iter()
            .zip(other.elements.iter())
            .map(|(a, b)| *a + *b)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Array conversion failed");
        Self::new(elements)
    }
}


impl<T, const N: usize> Sub for Vector<T, N> 
where
    T: Debug + Num + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(self.len(), other.len(), "Vectors must have the same length");
        let elements = self
            .elements
            .iter()
            .zip(other.elements.iter())
            .map(|(a, b)| *a - *b)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Array conversion failed");
        Self::new(elements)
    }
}

impl<T, U, const N: usize> Mul<U> for Vector<T, N>
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
            .map(|&x| x * scalar_t)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Array conversion failed");
        Self::new(elements)
    }
}

impl<T, U, const N: usize> Mul<Vector<U, N>> for Vector<T, N>
where
    T: Num + Copy + ToPrimitive + Debug,
    U: Num + Copy + ToPrimitive + Debug,
{
    type Output = f64; // Dot product results in f64 for precision

    fn mul(self, other: Vector<U, N>) -> f64 {
        self.elements
            .iter()
            .zip(other.elements.iter())
            .map(|(&a, &b)| {
                let a_f64 = a.to_f64().expect("Conversion to f64 failed");
                let b_f64 = b.to_f64().expect("Conversion to f64 failed");
                a_f64 * b_f64
            })
            .sum()
    }
}


impl<T, U, const N: usize> Mul<Vector<T, N>> for Scalar<U>
where
    T: Debug + Num + Copy + From<U>,
    U: Num + Copy,
{
    type Output = Vector<T, N>;

    fn mul(self, vector: Vector<T, N>) -> Vector<T, N> {
        let scalar_t = T::from(self.0);
        let elements = vector
            .elements
            .iter()
            .map(|&x| scalar_t * x)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Array conversion failed");
        Vector::new(elements)
    }
}

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
