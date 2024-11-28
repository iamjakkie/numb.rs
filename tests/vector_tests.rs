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

}