#[cfg(test)]
mod tests {
    use numbrs::{matrix, sqrt_vector, vector, Matrix, RowVector};

    #[test]
    fn test_vector_macro() {
        let v = vector![[1.0, 2.0, 3.0]];
        assert_eq!(v, matrix![[1.0, 2.0, 3.0]]);

        let v = vector![[1.0],[2.0],[3.0]];
        assert_eq!(v, matrix![[1.0],[2.0],[3.0]]);

        let empty = vector![[]];
        assert_eq!(empty, matrix![[]]);

        let int = vector![[1, 2, 3]];
        assert_eq!(int, matrix![[1, 2, 3]]);

        let zeros = vector![[0;3]];
        assert_eq!(zeros, matrix![[0, 0, 0]]);

        let zeros_float = vector![[0.0];2];
        assert_eq!(zeros_float, matrix![[0.0], [0.0]]);

        let ones = vector![[1; 5]];
        assert_eq!(ones, matrix![[1, 1, 1, 1, 1]]);

        let ones_float = vector![[1.0; 5]];
        assert_eq!(ones_float, matrix![[1.0, 1.0, 1.0, 1.0, 1.0]]);

        let filled = vector![[5.0; 4]];
        assert_eq!(filled, matrix![[5.0, 5.0, 5.0, 5.0]]);

        let filled_int = vector![[5; 4]];
        assert_eq!(filled_int, matrix![[5, 5, 5, 5]]);
    }

    #[test]
    fn test_vector_add() {
        let v1 = vector![[1.0, 2.0, 3.0]];
        let v2 = vector![[4.0, 5.0, 6.0]];
        let v3 = v1 + v2;
        let expected = vector![[5.0, 7.0, 9.0]];
        assert_eq!(v3, expected);

        let v1 = vector![[2, 5, -1]];
        let v2 = vector![[1, -1, 2]];
        let v3 = v1 + v2;
        assert_eq!(v3, vector![[3, 4, 1]]);

        let v1 = vector![[1, 2]];
        let v2 = vector![[3, 1]];
        let v3 = vector![[2, -1]];
        let result = v1 + v2 + v3;
        assert_eq!(result, vector![[6, 2]]);
    }

    #[test]
    fn test_vector_sub() {
        let v1 = vector![[1.0, 2.0, 3.0]];
        let v2 = vector![[4.0, 5.0, 6.0]];
        let v3 = v1 - v2;
        assert_eq!(v3, vector![[-3.0, -3.0, -3.0]]);
    }

    #[test]
    fn test_vector_mul() {
        let v1 = vector![[1.0, 2.0, 3.0]];
        let v2 = v1.clone() * 2.0;
        assert_eq!(v2, vector![[2.0, 4.0, 6.0]]);

        let v3 = v1.clone() * 5;
        assert_eq!(v3, vector![[5.0, 10.0, 15.0]]);

        // TODO: Implement Mul for Vector<T> where T: Num + Copy
        // let v4 = 5 * v1.clone();
        // assert_eq!(v4, vector![5.0, 10.0, 15.0]);
    }

    #[test]
    fn test_vector_operators() {
        let v1 = vector![[2, 1, -1]];
        let v2 = vector![[-1, 0, 3]];
        let result = v1*3 - v2*2;
        assert_eq!(result, vector![[8, 3, -9]]);

        let v1 = vector![[0,0,2]];
        let v2 = vector![[-1,2,1]];
        let v3 = vector![[1,2,0]];
        let v4 = vector![[3,2,-1]];
        let result = v1+v2;
        assert_eq!(result, vector![[-1,2,3]]);
        let result = v1+v2+v3;
        assert_eq!(result, vector![[0,4,3]]);
        let result = v4-v3*2;
        assert_eq!(result, vector![[1,-2,-1]]);
        let result = v1 + v2*2 + v3*2 + v4*2;
        assert_eq!(result, vector![[6, 12, 2]]);
        let result = v1 + v4;
        assert_eq!(result, vector![[3,2,1]]);
        let result = v2*4 + v2*3 - (v2*2 + v2*6);
        assert_eq!(result, vector![[1,-2,-1]]);
        let result = v3*4 - v2*2;
        assert_eq!(result, vector![[6,4,-2]]);
    }

    #[test]
    fn test_vector_dot() {
        let v1 = vector![[1.0, 2.0, 3.0]];
        let v2 = vector![[4.0, 5.0, 6.0]];
        let dot = v1 * v2;
        assert_eq!(dot, 32.0);
        // TODO: assert impossible
        // let v1 = vector![3,6,2];
        // let v2 = vector![-1,5,2,1];
        // let dot = v1 * v2;
        // assert_eq!(dot, 25);
        // let v = vector![-2,4];
        // let w = vector![2,1];
        // let dot = v * w;
        // assert_eq!(dot, 0.0);

        // let v = vector![1,2,3];
        // let w = vector![-3,2,-1];
        // let dot = v * w;
        // assert_eq!(dot, -2.0);

        // let v = vector![3,-1,0,1];
        // let w = vector![0,2,1,3];
        // let dot = v * w;
        // assert_eq!(dot, 1.0);

        // let v = sqrt_vector![2,3,5];
        // let w = v.clone();
        // println!("{:?} {:?}", v, w);
        // let dot = v * w;
        // assert_eq!(dot, 10.0);

        // let v: Vector<i32,9> = Vector::zeros();
        // let w = vector![8,1,5,-7,3,9,1,-3,2];
        // let dot = v * w;
        // assert_eq!(dot, 0.0);
    }

    // #[test]
    // fn test_vector_magnitude() {
    //     let v = vector![1.0, 2.0, 3.0];
    //     let mag = v.magnitude();
    //     assert_eq!(mag, (14.0 as f64).sqrt());

    //     let v = vector![1, 2, 3];
    //     let mag = v.magnitude();
    //     assert_eq!(mag, (14.0 as f64).sqrt());

    //     let v = vector![2,-5,4,6];
    //     let mag = v.magnitude();
    //     assert_eq!(mag, 9.0);

    //     let v = vector![3,4];
    //     let mag = v.magnitude();
    //     assert_eq!(mag, 5.0);

    //     let v = vector![2,1,-2];
    //     let mag = v.magnitude();
    //     assert_eq!(mag, 3.0);

    //     let elements = [
    //         -2.0 * (2.0_f64).sqrt(), // -2√2
    //         -3.0,                   // -3
    //         (10.0_f64).sqrt(),      // √10
    //         3.0                     // 3
    //     ];
    //     let v = Vector::new(elements);
    //     let mag = v.magnitude();
    //     assert_eq!(mag, 6.0);
    // }

    // #[test]
    // fn test_vector_angles() {
    //     let v = vector![1,2];
    //     let w = vector![3,4];
    //     let angle = (v.angle_between(&w) * 10000.0).round() / 10000.0;
    //     assert_eq!(angle, 0.1799);

    //     let v = vector![1,2,-1,-2];
    //     let w = vector![1,-1,1,-1];
    //     let angle = (v.angle_between(&w) * 10000.0).round() / 10000.0;
    //     assert_eq!(angle, 1.5708);

    //     let v_elements = [
    //         1.0,
    //         3.0_f64.sqrt(),
    //     ];
    //     let w_elements = [
    //         3.0_f64.sqrt(),
    //         1.0,
    //     ];
    //     let v = Vector::new(v_elements);
    //     let w = Vector::new(w_elements);
    //     let angle = (v.angle_between(&w) * 10000.0).round() / 10000.0;
    //     assert_eq!(angle, 0.5236);

    //     let v = vector![0,-2,2];
    //     let w = vector![1,0,1];
    //     let angle = (v.angle_between(&w) * 10000.0).round() / 10000.0;
    //     assert_eq!(angle, 1.0472);

    //     let v = vector![1,1,1,1];
    //     let w = vector![-2,0,-2,0];
    //     let angle = (v.angle_between(&w) * 10000.0).round() / 10000.0;
    //     assert_eq!(angle, 2.3562);
    // }

}