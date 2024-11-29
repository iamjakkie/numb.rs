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
    }

    #[test]
    fn test_vector_add() {
        let v1 = vector![1.0, 2.0, 3.0];
        let v2 = vector![4.0, 5.0, 6.0];
        let v3 = v1 + v2;
        assert_eq!(v3, vector![5.0, 7.0, 9.0]);
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