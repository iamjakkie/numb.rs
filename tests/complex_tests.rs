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

        let c = complex!(12);
        assert_eq!(c, complex!(12.0, 0.0));

        let c = complex!(+ 12 i);
        assert_eq!(c, complex!(0.0, 12.0));

        let c = complex!(- 12 i);
        assert_eq!(c, complex!(0.0, -12.0));
    }

    #[test]
    fn test_complex_addition() {
        let c = complex!(1, 2);
        let d = complex!(3,-5);

        let result = c + d;
        assert_eq!(result, complex!(4.0, -3.0));
    }

    #[test]
    fn test_complex_subtraction() {
        let c = complex!(1, 2);
        let d = complex!(3,-5);

        let result = c - d;
        assert_eq!(result, complex!(-2.0, 7.0));
    }

    #[test]
    fn test_complex_multiplication() {
        let c = complex!(2, 3);
        let d = complex!(4, 1);

        let result = c * d;
        assert_eq!(result, complex!(5.0, 14.0));

        let result = c * 10.0;
        assert_eq!(result, complex!(20.0, 30.0));

        // let result = 10.0 * c;
        // assert_eq!(result, complex!(20.0, 30.0));
    }

    #[test]
    fn test_complex_division() {
        let c = complex!(3, 2);
        let d = complex!(2, -3);

        let result = c / d;
        assert_eq!(result, complex!(+ 1.0 i));

        // let result = c / 10.0;
        // assert_eq!(result, complex!(0.2, 0.3));

        // let result = 10.0 / c;
        // assert_eq!(result, complex!(1.5384615384615385, -2.3076923076923075));
    }

    #[test]
    fn test_complex_operator() {
        let operation = (complex!(5, -1) - complex!(3, -2)) * (complex!(-2, 1) - complex!(3, -1) * complex!(3, 1));
        assert_eq!(operation, complex!(-25.0, -10));
    }
}
