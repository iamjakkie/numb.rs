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

}