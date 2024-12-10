#[cfg(test)]
mod tests {
    use numbrs::complex;

    #[test]
    fn test_complex_macro() {
        let c = complex!(1.0 + 2.0 i);
        assert_eq!(c, complex!(1.0, 2.0));

        let c = complex!(2 - 4 i);
        assert_eq!(c, complex!(2.0, -4.0));

        let c = complex!(2, 3);
        assert_eq!(c, complex!(2.0, 3.0));

        let c = complex!(1 - 1);
        assert_eq!(c, complex!(1.0, -1.0));
    }

    #[test]
    fn test_complex_addition() {
        let c = complex!(1, 2);
        let d = complex!(3,-5);

        let result = c + d;
        assert_eq!(result, complex!(4.0, -3.0));
    }

    fn test_complex_subtraction() {
        let c = complex!(1, 2);
        let d = complex!(3,-5);

        let result = c - d;
        assert_eq!(result, complex!(-2.0, 7.0));
    }

    #[test]
    fn test_complex_operator() {

    }
}
