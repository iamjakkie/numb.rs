#[cfg(test)]
mod tests {
    use numbrs::{vector, Vector};

    #[test]
    fn test_vector_macro() {
        let v = vector![1.0, 2.0, 3.0];
        assert_eq!(v, Vector::new(vec![1.0, 2.0, 3.0]));

        let empty = vector![];
        assert_eq!(empty, Vector::new(vec![]));
    }

    #[test]
    fn test_vector_add() {
        let v1 = vector![1.0, 2.0, 3.0];
        let v2 = vector![4.0, 5.0, 6.0];
        let v3 = v1 + v2;
        assert_eq!(v3, vector![5.0, 7.0, 9.0]);
    }

}