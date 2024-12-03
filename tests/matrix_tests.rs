#[cfg(test)]
mod tests {
    use numbrs::{identity_matrix, matrix, vector, Matrix};

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
    fn test_matrix_transpose() {
        let A = matrix![
            [1,2],
            [3,4],
        ];
        let B = matrix![
            [-1,1,1],
            [0,1,0],
        ];
        let res = A.transpose();
        assert_eq!(res, matrix![
            [1,3],
            [2,4],
        ]);

        let res = B.transpose();
        assert_eq!(res, matrix![
            [-1,0],
            [1,1],
            [1,0],
        ]);

        let res = (A * B).transpose();
        assert_eq!(res, matrix![
            [-1,-3],
            [3,7],
            [1,3],
        ]);

        let res = B.transpose() * A.transpose();
        assert_eq!(res, matrix![
            [-1,-3],
            [3,7],
            [1,3],
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

        let res = A * 2 - B * 3;
        assert_eq!(res, matrix![
            [-4,3],
            [4,-5],
        ]);


        let A = matrix![
            [1,2],
            [3,4],
        ];
        let v = vector![[2],[-1]];
        let res = A * v;
        assert_eq!(res, vector![[0],[2]]);

        let A = matrix![
            [2,-1],
            [0,3],
        ];

        let B = matrix![
            [-1,1],
            [-2,0],
        ];

        let res = A + B;
        assert_eq!(res, matrix![
            [1,0],
            [-2,3],
        ]);

        let res  = B * 2 - A * 3;
        assert_eq!(res, matrix![
            [-8,5],
            [-4,-9],
        ]);

        let res = A - B;
        assert_eq!(res, matrix![
            [3,-2],
            [2,3],
        ]);

        let res = (A + B) * 2 - A;
        assert_eq!(res, matrix![
            [0,1],
            [-4,3],
        ]);

        let C = matrix![
            [1,2,-1],
            [1,0,1],
        ];
        let D = matrix![
            [0,-2,1],
            [2,2,0],
        ];

        let res = A * B;
        assert_eq!(res, matrix![
            [0,2],
            [-6,0],
        ]);

        let res = C * D.transpose();
        assert_eq!(res, matrix![
            [-5,6],
            [1,2],
        ]);

        let res = A * C;
        assert_eq!(res, matrix![
            [1,4,-3],
            [3,0,3],
        ]);

        let res = C.transpose() * D;
        assert_eq!(res, matrix![
            [2,0,1],
            [0,-4,2],
            [2,4,-1],
        ]);

        let res = A ^ 2;
        assert_eq!(res, matrix![
            [4, -5],
            [0, 9],
        ]);

        let res = (C.transpose() * D) ^ 2;
        assert_eq!(res, matrix![
            [6,4,1],
            [4,24,-10],
            [2,-20,11],
        ]);

        let res = B * D;
        assert_eq!(res, matrix![
            [2,4,-1],
            [0,4,-2],
        ]);

        let res = (C - D).transpose() * (A - B.transpose());
        assert_eq!(res, matrix![
            [4,-2],
            [14,-2],
            [-7,1],
        ]);

        let res = (A + B) * (C + D);
        assert_eq!(res, matrix![
            [1,0,0],
            [7,6,3],
        ]);

        let res = D.transpose() * (A.transpose() + B).transpose() * C;
        assert_eq!(res, matrix![
            [8,4,4],
            [12,0,12],
            [-2,2,-4],
        ]);

        let A = matrix![
            [0,1],
            [-1,0],
        ];
        let res = A ^ 1000;
        assert_eq!(res, matrix![
            [1,0],
            [0,1],
        ]);

        let A = matrix![
            [1,0,1,0],
            [0,1,0,1],
        ];
        let B = matrix![
            [1,2,2,1],
            [2,1,1,2],
            [2,1,1,2],
            [1,2,2,1],
        ];
        let res = A * B;
        assert_eq!(res, matrix![
            [3,3,3,3],
            [3,3,3,3],
        ]);

        let A = matrix![
            [1,2,0],
            [3,1,0],
            [0,0,2],
        ];
        let B = matrix![
            [-1,1,0,0],
            [2,2,0,0],
            [0,0,1,1],
        ];
        let res = A * B;
        assert_eq!(res, matrix![
            [3,5,0,0],
            [-1,5,0,0],
            [0,0,2,2],
        ]);

        let A = matrix![
            [1,0,0,1],
            [0,1,0,1],
            [0,0,1,1],
            [0,0,0,2],
        ];
        let B = matrix![
            [1,2,3],
            [2,3,4],
            [3,4,5],
            [1,2,3],
        ];
        let res = A * B;
        assert_eq!(res, matrix![
            [2,4,6],
            [3,5,7],
            [4,6,8],
            [2,4,6],
        ]);

        let A = matrix![
            [1,0,0,0,1,1,1],
            [0,1,0,0,1,1,1],
            [0,0,1,0,1,1,1],
            [0,0,0,1,1,1,1],
            [1,1,1,1,0,0,0],
            [1,1,1,1,0,0,0],
            [1,1,1,1,0,0,0],
        ];

        let res = A ^ 2;
        assert_eq!(res, matrix![
            [4,3,3,3,1,1,1],
            [3,4,3,3,1,1,1],
            [3,3,4,3,1,1,1],
            [3,3,3,4,1,1,1],
            [1,1,1,1,4,4,4],
            [1,1,1,1,4,4,4],
            [1,1,1,1,4,4,4],
        ]);

        let A = matrix![
            [0,1,0],
            [1,0,0],
            [0,0,1],
        ];
        let B = matrix![
            [0,2,4,0],
            [1,1,0,-1],
            [3,4,2,1],
        ];
        let res = A * B;
        assert_eq!(res, matrix![
            [1,1,0,-1],
            [0,2,4,0],
            [3,4,2,1],
        ]);
    }

}