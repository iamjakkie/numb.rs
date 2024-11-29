#[cfg(test)]
mod tests {
    use numbrs::{vector, Vector};

    #[test]
    fn test_vector_macro() {
        let v = vector![1.0, 2.0, 3.0];
        assert_eq!(v, Vector::new(vec![1.0, 2.0, 3.0]));

        let empty = vector![];
        assert_eq!(empty, Vector::new(vec![]));

        let int = vector![1, 2, 3];
        assert_eq!(int, Vector::new(vec![1, 2, 3]));

        let zeros = vector![0; 3];
        assert_eq!(zeros, Vector::new(vec![0, 0, 0]));

        let ones = vector![1; 5];
        assert_eq!(ones, Vector::new(vec![1, 1, 1, 1, 1]));

        let filled = vector![5.0; 4];
        assert_eq!(filled, Vector::new(vec![5.0, 5.0, 5.0, 5.0]));

        let filled_int = vector![5; 4];
        assert_eq!(filled_int, Vector::new(vec![5, 5, 5, 5]));
    }

    #[test]
    fn test_vector_add() {
        let v1 = vector![1.0, 2.0, 3.0];
        let v2 = vector![4.0, 5.0, 6.0];
        let v3 = v1 + v2;
        assert_eq!(v3, vector![5.0, 7.0, 9.0]);

        let v1 = vector![2, 5, -1];
        let v2 = vector![1, -1, 2];
        let v3 = v1 + v2;
        assert_eq!(v3, vector![3, 4, 1]);

        let v1 = vector![1, 2];
        let v2 = vector![3, 1];
        let v3 = vector![2, -1];
        let result = v1 + v2 + v3;
        assert_eq!(result, vector![6, 2]);
    }

    #[test]
    fn test_vector_sub() {
        let v1 = vector![1.0, 2.0, 3.0];
        let v2 = vector![4.0, 5.0, 6.0];
        let v3 = v1 - v2;
        assert_eq!(v3, vector![-3.0, -3.0, -3.0]);
    }

    #[test]
    fn test_vector_mul() {
        let v1 = vector![1.0, 2.0, 3.0];
        let v2 = v1.clone() * 2.0;
        assert_eq!(v2, vector![2.0, 4.0, 6.0]);

        let v3 = v1.clone() * 5;
        assert_eq!(v3, vector![5.0, 10.0, 15.0]);
    }

    #[test]
    fn test_vector_operators() {
        let v1 = vector![2, 1, -1];
        let v2 = vector![-1, 0, 3];
        let result = v1*3 - v2*2;
        assert_eq!(result, vector![8, 3, -9]);


    }

    #[test]
    fn test_vector_dot() {
        let v1 = vector![1.0, 2.0, 3.0];
        let v2 = vector![4.0, 5.0, 6.0];
        let dot = v1 * v2;
        assert_eq!(dot, 32.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let v = vector![1.0, 2.0, 3.0];
        let mag = v.magnitude();
        assert_eq!(mag, (14.0 as f64).sqrt());

        let v = vector![1, 2, 3];
        let mag = v.magnitude();
        assert_eq!(mag, (14.0 as f64).sqrt());
    }

}