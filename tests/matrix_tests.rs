#[cfg(test)]
mod tests {
    use numbrs::{matrix, Matrix};

    #[test]
    fn test_matrix_macro() {
        let float = matrix![
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        assert_eq!(float, Matrix::new([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]));

        let A = matrix![];
        assert_eq!(A, Matrix::new([]));

        let int = matrix![
            [1,2,3],
            [4,5,6],
        ];
        assert_eq!(int, Matrix::new([
            [1,2,3],
            [4,5,6],
        ]));

        let zeros = matrix![0; 4, 4];
        assert_eq!(zeros, Matrix::new([
            [0,0,0,0],
            [0,0,0,0],
            [0,0,0,0],
            [0,0,0,0],
        ]));

        let zeros_float = matrix![0.0; 4, 4];
        assert_eq!(zeros_float, Matrix::new([
            [0.0,0.0,0.0,0.0],
            [0.0,0.0,0.0,0.0],
            [0.0,0.0,0.0,0.0],
            [0.0,0.0,0.0,0.0],
        ]));

        let ones = matrix![1; 3, 3];
        assert_eq!(ones, Matrix::new([
            [1,1,1],
            [1,1,1],
            [1,1,1],
        ]));

        let ones_float = matrix![1.0; 3, 3];
        assert_eq!(ones_float, Matrix::new([
            [1.0,1.0,1.0],
            [1.0,1.0,1.0],
            [1.0,1.0,1.0],
        ]));

        let filled = matrix![5.0; 2, 2];
        assert_eq!(filled, Matrix::new([
            [5.0,5.0],
            [5.0,5.0],
        ]));
            
    }
}