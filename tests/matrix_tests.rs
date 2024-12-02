#[cfg(test)]
mod tests {
    use numbrs::{identity_matrix, matrix, Matrix};

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

    #[test]
    fn test_matrix_add() {
        let A = matrix![
            [1,3],
            [2,-1],
        ];
        let B = matrix![
            [2,1],
            [0,1],
        ];
        let C = matrix![
            [1,0,1],
            [0,-1,1],
        ];

        let res = A + B;
        assert_eq!(res, matrix![
            [3,4],
            [2,0],
        ]);
    }

    #[test]
    fn test_matrix_mul() {
        let A = matrix![
            [1,3],
            [2,-1],
        ];
        let res = A * 2;
        assert_eq!(res, matrix![
            [2,6],
            [4,-2],
        ]);

        let B = matrix![
            [2,1],
            [0,1],
        ];
        let res = B * 3;
        assert_eq!(res, matrix![
            [6,3],
            [0,3],
        ]);

        let A = matrix![
            [1,2],
            [3,4],
        ];
        let B = matrix![
            [5,6,7],
            [8,9,10],
        ];
        let C = matrix![
            [1,0],
            [0,-1],
            [2,-1],
        ];

        let res = A * B;
        assert_eq!(res, matrix![
            [21,24,27],
            [47,54,61],
        ]);

        let res = B * C;
        assert_eq!(res, matrix![
            [19,-13],
            [28,-19],
        ]);
    }

    #[test]
    fn test_matrix_identity() {
        let I = identity_matrix!(f64, 3);
        assert_eq!(I, matrix![
            [1.0,0.0,0.0],
            [0.0,1.0,0.0],
            [0.0,0.0,1.0],
        ]);

        let I = identity_matrix!(i32, 4);
        assert_eq!(I, matrix![
            [1,0,0,0],
            [0,1,0,0],
            [0,0,1,0],
            [0,0,0,1],
        ]);
    }

    #[test]
    fn test_matrix_power() {
        let A = matrix![
            [2,1],
            [-1,3],
        ];
        let res = A ^ 2;
        assert_eq!(res, matrix![
            [3,5],
            [-5,8],
        ]);

        let res = A ^ 4;
        assert_eq!(res, matrix![
            [-16,55],
            [-55,39],
        ]);

        let I = identity_matrix!(i32, 2);
        let res = I ^ 3;
        assert_eq!(res, matrix![
            [1,0],
            [0,1],
        ]);
    }

    #[test]
    fn test_matrix_operators() {
        let A = matrix![
            [1,3],
            [2,-1],
        ];
        let B = matrix![
            [2,1],
            [0,1],
        ];
        let C = matrix![
            [1,0,1],
            [0,-1,1],
        ];

        let res = A * 2 - B * 3;
        assert_eq!(res, matrix![
            [-4,3],
            [4,-5],
        ]);

        // let res = A + C * 2;
        // assert_!(res, matrix![
        //     [3,3],
        //     [2,-3],
        // ]);

        
    }

}