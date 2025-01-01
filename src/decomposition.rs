use crate::Matrix;
use num::{Num, Signed};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct PartialPivLu<T, const N: usize>
where
    T: Num + Copy + Debug,
{
    pub l: Matrix<T, N, N>,
    pub u: Matrix<T, N, N>,
    pub p: Matrix<T, N, N>, // Permutation matrix
}

impl<T, const N: usize> Matrix<T, N, N>
where
    T: Num + Copy + Debug + Signed + PartialOrd,
{
    pub fn lu_decomposition(&self) -> (Matrix<T, N, N>, Matrix<T, N, N>) {
        let mut l = Matrix::<T, N, N>::identity(); // Lower triangular matrix
        let mut u = *self; // Upper triangular matrix (copied from the original matrix)

        for k in 0..N {
            // Partial pivoting: Find the row with the largest absolute value in column `k`
            let mut pivot_row = k;
            for i in (k + 1)..N {
                if u.elements[i][k].abs() > u.elements[pivot_row][k].abs() {
                    pivot_row = i;
                }
            }

            // Swap rows in U
            if pivot_row != k {
                u.elements.swap(k, pivot_row);

                // Also swap the rows in L (but only the columns before k)
                for j in 0..k {
                    l.elements[k][j] = l.elements[pivot_row][j];
                    l.elements[pivot_row][j] = T::zero();
                }
            }

            // Perform Gaussian elimination to zero out below-diagonal elements in column `k`
            for i in (k + 1)..N {
                let factor = u.elements[i][k] / u.elements[k][k];
                l.elements[i][k] = factor; // Update L

                for j in k..N {
                    u.elements[i][j] = u.elements[i][j] - factor * u.elements[k][j];
                }
            }
        }

        (l, u)
    }
}