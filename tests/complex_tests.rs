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
        let d = complex!(3, -5);

        let result = c + d;
        assert_eq!(result, complex!(4.0, -3.0));
    }

    #[test]
    fn test_complex_subtraction() {
        let c = complex!(1, 2);
        let d = complex!(3, -5);

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

        let a = complex!(2, -3);
        let b = complex!(5, 1);

        let result = a * b;
        assert_eq!(result, complex!(13.0, -13.0));

        let a = complex!(1, -1);
        let b = complex!(2, 1);

        let result = a * b;
        assert_eq!(result, complex!(3.0, -1.0));

        let a = complex!(-2, 3);
        let b = complex!(-4, -1);

        let result = a * b;
        assert_eq!(result, complex!(11.0, -10.0));
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
    fn test_complex_conjugate() {
        let c = complex!(3, 2);
        let result = c.conj();
        assert_eq!(result, complex!(3.0, -2.0));
    }

    #[test]
    fn test_complex_power() {
        let c = complex!(3, 2);
        let result = c.powi(2);
        assert_eq!(result, complex!(5.0, 12.0));

        let i = complex!(0, 1);

        let power_0 = i.powi(0);
        let power_1 = i.powi(1);
        let power_2 = i.powi(2);
        let power_3 = i.powi(3);
        let power_4 = i.powi(4);
        let power_5 = i.powi(5);
        let power_6 = i.powi(6);
        let power_7 = i.powi(7);

        assert_eq!(power_0, complex!(1.0, 0.0));
        assert_eq!(power_1, complex!(0.0, 1.0));
        assert_eq!(power_2, complex!(-1.0, 0.0));
        assert_eq!(power_3, complex!(0.0, -1.0));
        assert_eq!(power_4, complex!(1.0, 0.0));
        assert_eq!(power_5, complex!(0.0, 1.0));
        assert_eq!(power_6, complex!(-1.0, 0.0));
        assert_eq!(power_7, complex!(0.0, -1.0));

        let power_neg_1 = i.powi(-1);
        let power_neg_2 = i.powi(-2);
        let power_neg_3 = i.powi(-3);
        let power_neg_4 = i.powi(-4);
        let power_neg_5 = i.powi(-5);
        let power_neg_6 = i.powi(-6);
        let power_neg_7 = i.powi(-7);

        assert_eq!(power_neg_1, complex!(0.0, -1.0));
        assert_eq!(power_neg_2, complex!(-1.0, 0.0));
        assert_eq!(power_neg_3, complex!(0.0, 1.0));
        assert_eq!(power_neg_4, complex!(1.0, 0.0));
        assert_eq!(power_neg_5, complex!(0.0, -1.0));
        assert_eq!(power_neg_6, complex!(-1.0, 0.0));
        assert_eq!(power_neg_7, complex!(0.0, 1.0));
    }

    #[test]
    fn test_complex_modulus() {
        let c = complex!(3, 4);
        let result = c.norm();
        assert_eq!(result, 5.0);

        let a = complex!(5, 6);
        let result = a.norm();
        assert_eq!(result, 61.0_f64.sqrt());
    }

    #[test]
    fn test_complex_polar_form() {
        let c = complex!(-4, 4);
        let (r, theta) = c.to_polar();
        assert_eq!(r, 4.0 * 2_f64.sqrt());
        assert_eq!(theta, 3.0 * std::f64::consts::FRAC_PI_4);
    }

    // #[test]
    // fn test_complex_exponential_form() {
    //     let c = complex!(-4, 4);
    //     c.exp()
    // }

    #[test]
    fn test_complex_root() {
        let c = complex!(0, -1);
        // c.r
    }

    #[test]
    fn test_complex_operator() {
        let operation = (complex!(5, -1) - complex!(3, -2))
            * (complex!(-2, 1) - complex!(3, -1) * complex!(3, 1));
        assert_eq!(operation, complex!(-25.0, -10));
    }
}
