use num::{Num, Signed, ToPrimitive};
use std::fmt::Debug;
use std::ops::{Add, BitXor, Mul, Sub};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Matrix<T, const M: usize, const N: usize>
where
    T: Debug,
{
    pub elements: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Debug + Num + Copy + Signed,
{
    pub fn new(elements: [[T; N]; M]) -> Self {
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
        M * N
    }

    // pub fn lu_decomposition(&self) -> (Matrix<T, N, N>, Matrix<T, N, N>, T) 
    // where T: std::cmp::PartialOrd + Signed
    // {
    //     let mut l = Matrix::<T, N, N>::identity(); // Lower triangular matrix
    //     let mut u = *self; // Upper triangular matrix (copied from the original matrix)
    //     let mut sign = T::one(); // Keeps track of sign adjustments due to row swaps

    //     for k in 0..N {
    //         // Partial pivoting: find the row with the largest absolute value in column `k`
    //         let mut pivot_row = k;
    //         for i in (k + 1)..N {
    //             if u.elements[i][k].abs() > u.elements[pivot_row][k].abs() {
    //                 pivot_row = i;
    //             }
    //         }

    //         // If pivot_row is not the current row, swap rows and adjust the sign
    //         if pivot_row != k {
    //             u.elements.swap(k, pivot_row);
    //             sign = -sign;
    //         }

    //         // Perform Gaussian elimination
    //         for i in (k + 1)..N {
    //             let factor = u.elements[i][k] / u.elements[k][k];
    //             l.elements[i][k] = factor; // Store the factor in `L`

    //             for j in k..N {
    //                 u.elements[i][j] = u.elements[i][j] - factor * u.elements[k][j];
    //             }
    //         }
    //     }

    //     (l, u, sign)
    // }
}

#[derive(Debug)]
pub struct NumWrapper<T: Num>(T)
where
    T: Debug + Num + Copy;

#[derive(Debug)]
pub struct RowVector<T, const N: usize>(pub Matrix<T, 1, N>)
where
    T: Debug + Num + Copy + ToPrimitive;

impl<T, const N: usize> RowVector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive,
{
    pub fn new(matrix: Matrix<T, 1, N>) -> Self {
        RowVector(matrix)
    }

    pub fn magnitude(&self) -> f64 {
        self.0.elements[0]
            .iter()
            .map(|&x| {
                let x_f64 = x.to_f64().expect("Conversion to f64 failed");
                x_f64 * x_f64
            })
            .sum::<f64>()
            .sqrt()
    }

    pub fn dot(&self, other: &RowVector<T, N>) -> f64 {
        self.0.elements[0]
            .iter()
            .zip(other.0.elements[0].iter())
            .map(|(&a, &b)| {
                let a_f64 = a.to_f64().expect("Conversion to f64 failed");
                let b_f64 = b.to_f64().expect("Conversion to f64 failed");
                a_f64 * b_f64
            })
            .sum()
    }

    pub fn angle_between(&self, other: &RowVector<T, N>) -> f64 {
        let dot_product = self.dot(other);
        let magnitudes = self.magnitude() * other.magnitude();
        dot_product.acos() / magnitudes
    }

}

#[derive(Debug)]
pub struct ColumnVector<T, const N: usize>(Matrix<T, N, 1>)
where
    T: Debug + Num + Copy + ToPrimitive;

impl<T, const M: usize> ColumnVector<T, M>
where
    T: Debug + Num + Copy + ToPrimitive,
{
    pub fn new(matrix: Matrix<T, M, 1>) -> Self {
        ColumnVector(matrix)
    }

    pub fn magnitude(&self) -> f64 {
        self.0.elements[0]
            .iter()
            .map(|&x| {
                let x_f64 = x.to_f64().expect("Conversion to f64 failed");
                x_f64 * x_f64
            })
            .sum::<f64>()
            .sqrt()
    }
}

impl<T, const N: usize> Copy for RowVector<T, N> where T: Copy + Debug + Num + ToPrimitive {}

impl<T, const N: usize> Clone for RowVector<T, N>
where
    T: Copy + Debug + Num + ToPrimitive,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const N: usize> PartialEq<Matrix<T, 1, N>> for RowVector<T, N>
where
    T: PartialEq + Copy + Debug + Num + ToPrimitive,
{
    fn eq(&self, other: &Matrix<T, 1, N>) -> bool {
        self.0 == *other
    }
}

impl<T, const N: usize> PartialEq<RowVector<T, N>> for RowVector<T, N>
where
    T: PartialEq + Copy + Debug + Num + ToPrimitive,
{
    fn eq(&self, other: &RowVector<T, N>) -> bool {
        self.0 == other.0
    }
}

impl<T, const N: usize> PartialEq<Matrix<T, N, 1>> for ColumnVector<T, N>
where
    T: PartialEq + Copy + Debug + Num + ToPrimitive,
{
    fn eq(&self, other: &Matrix<T, N, 1>) -> bool {
        self.0 == *other
    }
}

impl<T, const N: usize> PartialEq<ColumnVector<T, N>> for ColumnVector<T, N>
where
    T: PartialEq + Copy + Debug + Num + ToPrimitive,
{
    fn eq(&self, other: &ColumnVector<T, N>) -> bool {
        self.0 == other.0
    }
}

impl<T, const N: usize> Copy for ColumnVector<T, N> where T: Copy + Debug + Num + ToPrimitive {}

impl<T, const N: usize> Clone for ColumnVector<T, N>
where
    T: Copy + Debug + Num + ToPrimitive,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const M: usize, const N: usize> Add for Matrix<T, M, N>
where
    T: Debug + Num + Copy + Signed,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Ensure the matrices have the same dimensions
        assert_eq!(
            self.len(),
            other.len(),
            "Matrices must have the same dimensions"
        );

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

// impl<T, const N: usize> Add for RowVector<T, N>
// where
//     T: Debug + Num + Copy,
// {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         Self(self.0 + other.0) // Use the `Matrix`'s `Add` implementation
//     }
// }
// impl<T, const M: usize, const N: usize> Add for &Matrix<T, M, N>
// where
//     T: Debug + Num + Copy,
// {
//     type Output = Matrix<T, M, N>;

//     fn add(self, other: Self) -> Self::Output {
//         let mut result = [[T::zero(); N]; M];
//         for i in 0..M {
//             for j in 0..N {
//                 result[i][j] = self.elements[i][j] + other.elements[i][j];
//             }
//         }
//         Matrix::new(result)
//     }
// }
impl<T, const N: usize> Add for RowVector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive + Signed,
{
    type Output = RowVector<T, N>;

    fn add(self, other: Self) -> Self {
        RowVector(self.0 + other.0) // Use `Matrix`'s `Add` for owned values
    }
}

impl<T, const N: usize> Add for ColumnVector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive + Signed,
{
    type Output = ColumnVector<T, N>;

    fn add(self, other: Self) -> Self {
        ColumnVector(self.0 + other.0) // Use the `Matrix`'s `Add` implementation
    }
}

impl<T, const M: usize, const N: usize> Sub for Matrix<T, M, N>
where
    T: Debug + Num + Copy + Signed,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // Ensure the matrices have the same dimensions
        assert_eq!(
            self.len(),
            other.len(),
            "Matrices must have the same dimensions"
        );

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

impl<T, const N: usize> Sub for RowVector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive + Signed,
{
    type Output = RowVector<T, N>;

    fn sub(self, other: Self) -> Self {
        RowVector(self.0 - other.0) // Delegate to `Matrix`'s `Sub` implementation
    }
}

impl<T, const N: usize> Sub for ColumnVector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive + Signed,
{
    type Output = ColumnVector<T, N>;

    fn sub(self, other: Self) -> Self {
        ColumnVector(self.0 - other.0) // Delegate to `Matrix`'s `Sub` implementation
    }
}

impl<T, U, const M: usize, const N: usize> Mul<U> for Matrix<T, M, N>
where
    T: Debug + Num + Copy + From<U> + Signed,
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
    T: Debug + Num + Copy + Signed,
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

impl<T, const M: usize, const N: usize> Mul<ColumnVector<T, N>> for Matrix<T, M, N>
where
    T: Debug + Num + Copy + ToPrimitive + Signed,
{
    type Output = ColumnVector<T, M>;

    fn mul(self, vector: ColumnVector<T, N>) -> Self::Output {
        let mut result = [[T::zero(); 1]; M]; // M rows, 1 column

        for i in 0..M {
            for j in 0..N {
                result[i][0] = result[i][0] + (self.elements[i][j] * vector.0.elements[j][0]);
            }
        }

        ColumnVector::new(Matrix::new(result))
    }
}

impl<T, const M: usize, const N: usize> Mul<Matrix<T, M, N>> for RowVector<T, M>
where
    T: Debug + Num + Copy + ToPrimitive + Signed,
{
    type Output = Matrix<T, 1, N>; // Result is a row vector

    fn mul(self, matrix: Matrix<T, M, N>) -> Self::Output {
        let mut result = [[T::zero(); N]; 1]; // 1 row, N columns

        for i in 0..N {
            for j in 0..M {
                result[0][i] = result[0][i] + (self.0.elements[0][j] * matrix.elements[j][i]);
            }
        }

        Matrix::new(result)
    }
}

impl<T, const N: usize> Mul<RowVector<T, N>> for RowVector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive + Signed,
{
    type Output = f64;

    fn mul(self, other: RowVector<T, N>) -> f64 {
        self.0.elements[0]
            .iter()
            .zip(other.0.elements[0].iter())
            .map(|(&a, &b)| {
                let a_f64 = a.to_f64().expect("Conversion to f64 failed");
                let b_f64 = b.to_f64().expect("Conversion to f64 failed");
                a_f64 * b_f64
            })
            .sum()
    }
}

impl<T, const N: usize> Mul<ColumnVector<T, N>> for ColumnVector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive,
{
    type Output = f64;

    fn mul(self, other: ColumnVector<T, N>) -> f64 {
        self.0
            .elements
            .iter()
            .zip(other.0.elements.iter())
            .map(|([a], [b])| {
                let a_f64 = a.to_f64().expect("Conversion to f64 failed");
                let b_f64 = b.to_f64().expect("Conversion to f64 failed");
                a_f64 * b_f64
            })
            .sum()
    }
}

impl<T, U, const N: usize> Mul<U> for RowVector<T, N>
where
    T: Debug + Num + Copy + From<U> + ToPrimitive + Signed,
    U: Num + Copy,
{
    type Output = Self;

    fn mul(self, scalar: U) -> Self {
        Self(self.0 * scalar) // Delegate to `Matrix`'s `Mul` implementation
    }
}

impl<T, U, const N: usize> Mul<U> for ColumnVector<T, N>
where
    T: Debug + Num + Copy + From<U> + ToPrimitive + Signed,
    U: Num + Copy,
{
    type Output = Self;

    fn mul(self, scalar: U) -> Self {
        Self(self.0 * scalar) // Delegate to `Matrix`'s `Mul` implementation
    }
}

impl<T, const N: usize, const M: usize> Mul<RowVector<T, M>> for ColumnVector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive + Signed,
{
    type Output = Matrix<T, N, M>; // Result is a matrix

    fn mul(self, other: RowVector<T, M>) -> Self::Output {
        let mut result = [[T::zero(); M]; N]; // Create a zero-initialized matrix of size N x M

        for i in 0..N {
            for j in 0..M {
                result[i][j] = self.0.elements[i][0] * other.0.elements[0][j];
            }
        }

        Matrix::new(result)
    }
}

impl<T, const N: usize> Mul<ColumnVector<T, N>> for RowVector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive,
{
    type Output = T; // Result is a scalar (dot product)

    fn mul(self, other: ColumnVector<T, N>) -> Self::Output {
        self.0.elements[0]
            .iter()
            .zip(other.0.elements.iter())
            .map(|(&row_val, &[col_val])| row_val * col_val)
            .fold(T::zero(), |acc, x| acc + x)
    }
}

// implement multiplication for 5 * RowVector
// impl<T, U, const N: usize> Mul<RowVector<T, N>> for U
// where
//     T: Debug + Num + Copy + From<U> + ToPrimitive,
//     U: Debug + Num + Copy,
// {
//     type Output = RowVector<T, N>;

//     fn mul(self, vector: RowVector<T, N>) -> Self::Output {
//         vector * self
//     }
// }

impl<T, const N: usize> BitXor<u32> for Matrix<T, N, N>
where
    T: Debug + Num + Copy + Signed,
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

// impl<T, const M: usize, const N: usize> Mul<Vector<T, N>> for Matrix<T, M, N>
// where
//     T: Debug + Num + Copy,
// {
//     type Output = Vector<T, M>;

//     fn mul(self, vector: Vector<T, N>) -> Self::Output {
//         let mut result = [T::zero(); M];

//         for i in 0..M {
//             for j in 0..N {
//                 result[i] = result[i] + (self.elements[i][j] * vector.elements[j]);
//             }
//         }

//         Vector::new(result)
//     }
// }
