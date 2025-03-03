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
    T: Debug + Num + Copy,
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

    pub fn inverse(self) -> Option<Matrix<f64, M, N>>
    where
        T: Num + Copy + ToPrimitive + PartialOrd + Debug,
    {
        let precision = 6;
        // Ensure the matrix is square
        if M != N {
            return None;
        }

        let aug_cols = 2 * N; // Calculate augmented matrix columns at runtime

        // Create an augmented matrix [A | I] as f64
        let mut augmented = vec![vec![0.0_f64; aug_cols]; M];
        for i in 0..M {
            for j in 0..N {
                augmented[i][j] = self.elements[i][j].to_f64().expect("Conversion to f64 failed");
                augmented[i][j + N] = if i == j { 1.0 } else { 0.0 };
            }
        }

        // Perform row operations to transform the left side into the identity matrix
        for i in 0..N {
            // Find the pivot
            let mut pivot = i;
            for j in i + 1..M {
                if augmented[j][i].abs() > augmented[pivot][i].abs() {
                    pivot = j;
                }
            }

            // Swap the rows
            augmented.swap(i, pivot);

            // Ensure the pivot is not zero
            if augmented[i][i] == 0.0 {
                return None;
            }

            // Scale the row to make the pivot 1
            let divisor = augmented[i][i];
            for j in 0..aug_cols {
                augmented[i][j] /= divisor;
            }

            // Perform row operations to make the other elements in the column zero
            for j in 0..M {
                if i != j {
                    let factor = augmented[j][i];
                    for k in 0..aug_cols {
                        augmented[j][k] -= factor * augmented[i][k];
                    }
                }
            }
        }

        // Extract the right side of the augmented matrix and round the results
        let mut result = [[0.0; N]; M];
        for i in 0..M {
            for j in 0..N {
                result[i][j] = (augmented[i][j + N] * 10f64.powi(precision as i32)).round()
                    / 10f64.powi(precision as i32);
            }
        }

        Some(Matrix::new(result))
    }

    fn determinant_2x2(&self) -> T {
        assert!(M == 2 && N == 2, "Matrix must be 2x2 for this function.");
        self.elements[0][0] * self.elements[1][1] - self.elements[0][1] * self.elements[1][0]
    }

    /// Determinant for 3x3 matrix
    fn determinant_3x3(&self) -> T {
        assert!(M == 3 && N == 3, "Matrix must be 3x3 for this function.");
        let a = self.elements[0][0];
        let b = self.elements[0][1];
        let c = self.elements[0][2];
        let d = self.elements[1][0];
        let e = self.elements[1][1];
        let f = self.elements[1][2];
        let g = self.elements[2][0];
        let h = self.elements[2][1];
        let i = self.elements[2][2];

        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }

    /// Determinant for NxN matrix
    // pub fn determinant(&self) -> T {
    //     assert!(M == N, "Matrix must be square to compute determinant.");

    //     match M {
    //         1 => self.elements[0][0],
    //         2 => self.determinant_2x2(),
    //         3 => self.determinant_3x3(),
    //         _ => {
    //             let mut det = T::zero();
    //             for i in 0..N {
    //                 let sign = if i % 2 == 0 { T::one() } else { -T::one() };
    //                 let minor = self.minor(0, i);
    //                 det = det + sign * self.elements[0][i] * minor.determinant();
    //             }
    //             det
    //         }
    //     }
    // }

    /// Compute the minor matrix by removing the specified row and column
    // fn minor(&self, row: usize, col: usize) -> Matrix<T, M, N> {
    //     assert!(M > 1 && N > 1, "Matrix must be larger than 1x1 to compute a minor.");

    //     let mut minor_elements = vec![vec![T::zero(); N - 1]; M - 1];
    //     for i in 0..M {
    //         if i == row {
    //             continue;
    //         }
    //         for j in 0..N {
    //             if j == col {
    //                 continue;
    //             }
    //             let minor_row = if i < row { i } else { i - 1 };
    //             let minor_col = if j < col { j } else { j - 1 };
    //             minor_elements[minor_row][minor_col] = self.elements[i][j];
    //         }
    //     }

    //     let minor_elements: [[T; N - 1]; M - 1] = minor_elements.into_iter().map(|row| row.try_into().unwrap()).collect::<Vec<_>>().try_into().unwrap();
    //     Matrix::new(minor_elements)
    // }


    // pub fn determinant(self) -> T
    // where
    //     T: Num + Copy + Signed,
    // {
    //     // Ensure the matrix is square
    //     assert_eq!(M, N, "Matrix must be square");

    //     // Base case: 1x1 matrix
    //     if M == 1 {
    //         return self.elements[0][0];
    //     }

    //     // Base case: 2x2 matrix
    //     if M == 2 {
    //         return self.elements[0][0] * self.elements[1][1] - self.elements[0][1] * self.elements[1][0];
    //     }

    //     // Recursive case: use Laplace expansion
    //     let mut det = T::zero();
    //     for i in 0..N {
    //         let sign = if i % 2 == 0 { T::one() } else { -T::one() };
    //         let minor = self.minor(0, i);
    //         det = det + sign * self.elements[0][i] * minor.determinant();
    //     }

    //     det
    // }

    // pub fn conjugate_transpose(self) -> Matrix<T, N, M> {
    //     let transposed = self.transpose();
    //     // for each element do the conjugate, if the element is complex
    //     let mut result = [[T::zero(); M]; N];

    //     for i in 0..N {
    //         for j in 0..M {
    //             result[i][j] = if let Some(conjugate) = transposed.elements[i][j].try_conjugate() {
    //                 conjugate
    //             } else {
    //                 transposed.elements[i][j] // No conjugation for real numbers
    //             };
    //         }
    //     }
    
    //     Matrix::new(result)
    // }

    pub fn len(&self) -> usize {
        M * N
    }

    
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
        let cos_theta = dot_product / magnitudes;
        cos_theta.acos()
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
    T: Debug + Num + Copy,
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
    T: Debug + Num + Copy + ToPrimitive,
{
    type Output = RowVector<T, N>;

    fn add(self, other: Self) -> Self {
        RowVector(self.0 + other.0) // Use `Matrix`'s `Add` for owned values
    }
}

impl<T, const N: usize> Add for ColumnVector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive,
{
    type Output = ColumnVector<T, N>;

    fn add(self, other: Self) -> Self {
        ColumnVector(self.0 + other.0) // Use the `Matrix`'s `Add` implementation
    }
}

impl<T, const M: usize, const N: usize> Sub for Matrix<T, M, N>
where
    T: Debug + Num + Copy,
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
    T: Debug + Num + Copy + ToPrimitive,
{
    type Output = RowVector<T, N>;

    fn sub(self, other: Self) -> Self {
        RowVector(self.0 - other.0) // Delegate to `Matrix`'s `Sub` implementation
    }
}

impl<T, const N: usize> Sub for ColumnVector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive,
{
    type Output = ColumnVector<T, N>;

    fn sub(self, other: Self) -> Self {
        ColumnVector(self.0 - other.0) // Delegate to `Matrix`'s `Sub` implementation
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

impl<T, const M: usize, const N: usize> Mul<ColumnVector<T, N>> for Matrix<T, M, N>
where
    T: Debug + Num + Copy + ToPrimitive,
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
    T: Debug + Num + Copy + ToPrimitive,
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
    T: Debug + Num + Copy + ToPrimitive,
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
    T: Debug + Num + Copy + From<U> + ToPrimitive,
    U: Num + Copy,
{
    type Output = Self;

    fn mul(self, scalar: U) -> Self {
        Self(self.0 * scalar) // Delegate to `Matrix`'s `Mul` implementation
    }
}

impl<T, U, const N: usize> Mul<U> for ColumnVector<T, N>
where
    T: Debug + Num + Copy + From<U> + ToPrimitive,
    U: Num + Copy,
{
    type Output = Self;

    fn mul(self, scalar: U) -> Self {
        Self(self.0 * scalar) // Delegate to `Matrix`'s `Mul` implementation
    }
}

impl<T, const N: usize, const M: usize> Mul<RowVector<T, M>> for ColumnVector<T, N>
where
    T: Debug + Num + Copy + ToPrimitive,
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
