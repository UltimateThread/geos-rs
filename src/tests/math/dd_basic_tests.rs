#[cfg(test)]
mod dd_basic_tests {
    use crate::core::math::dd::DD;

    const VALUE_DBL: f64 = 2.2;

    #[test]
    fn test_set_value_double() {
        let mut dd = DD::new_x(1.);
        dd.set_value_f64(VALUE_DBL);
        assert!(VALUE_DBL == dd.double_value());
    }

    #[test]
    fn test_set_value_dd() {
        let mut dd1 = DD::new_x(1.);
        dd1.set_value_dd(&DD::new_x(2.2));
        assert!((DD::new_x(VALUE_DBL)).equals(&dd1));

        let mut dd2 = DD::new_x(1.);
        dd2.set_value_dd(&DD::new_pi());
        assert!(DD::new_pi().equals(&dd2));
    }

    #[test]
    fn test_copy() {
        assert!((DD::new_x(VALUE_DBL)).equals(&DD::copy(&DD::new_x(VALUE_DBL))));
        assert!(DD::new_pi().equals(&DD::copy(&DD::new_pi())));
    }

    #[test]
    fn test_nan() {
        assert!(DD::value_of_f64(1.)
            .divide_dd(&DD::value_of_f64(0.))
            .is_nan());
        assert!(DD::value_of_f64(1.).divide_dd(&DD::create_nan()).is_nan());
    }

    #[test]
    fn test_add_mult2() {
        check_add_mult2(&DD::new_x(3.));
        check_add_mult2(&DD::new_pi());
    }

    #[test]
    fn test_multiply_divide() {
        check_multiply_divide(&DD::new_pi(), &DD::new_e(), 1e-30);
        check_multiply_divide(&DD::new_pi_2(), &DD::new_e(), 1e-30);
        check_multiply_divide(&DD::new_pi_2(), &DD::new_e(), 1e-30);
        check_multiply_divide(&DD::new_x(39.4), &DD::new_x(10.), 1e-30);
    }

    #[test]
    fn test_divide_multiply() {
        check_divide_multiply(&DD::new_pi(), &DD::new_e(), 1e-30);
        check_divide_multiply(&DD::new_x(39.4), &DD::new_x(10.), 1e-30);
    }

    #[test]
    fn test_sqrt() {
        // the appropriate error bound is determined empirically
        check_sqrt(&DD::new_pi(), 1e-30);
        check_sqrt(&DD::new_e(), 1e-30);
        check_sqrt(&DD::new_x(999.0), 1e-28);
    }

    fn check_sqrt(x: &DD, err_bound: f64) {
        let sqrt = x.sqrt_dd();
        let x2 = sqrt.multiply_dd(&sqrt);
        check_error_bound(x, &x2, err_bound);
    }

    #[test]
    fn test_trunc() {
        check_trunc(
            &DD::value_of_f64(1e16).subtract_dd(&DD::value_of_f64(1.)),
            &DD::value_of_f64(1e16).subtract_dd(&DD::value_of_f64(1.)),
        );
        // the appropriate error bound is determined empirically
        check_trunc(&DD::new_pi(), &DD::value_of_f64(3.));
        check_trunc(&DD::value_of_f64(999.999), &DD::value_of_f64(999.));

        check_trunc(&DD::new_e().negate(), &DD::value_of_f64(-2.));
        check_trunc(&DD::value_of_f64(-999.999), &DD::value_of_f64(-999.));
    }

    fn check_trunc(x: &DD, expected: &DD) {
        let trunc = x.trunc();
        let is_equal = trunc.equals(expected);
        assert!(is_equal);
    }

    #[test]
    fn test_pow() {
        check_pow(0., 3, 16. * DD::EPS);
        check_pow(14., 3, 16. * DD::EPS);
        check_pow(3., -5, 16. * DD::EPS);
        check_pow(-3., 5, 16. * DD::EPS);
        check_pow(-3., -5, 16. * DD::EPS);
        check_pow(0.12345, -5, 1e5 * DD::EPS);
    }

    #[test]
    fn test_reciprocal() {
        // error bounds are chosen to be "close enough" (i.e. heuristically)

        // for some reason many reciprocals are exact
        check_reciprocal(3.0, 0.);
        check_reciprocal(99.0, 1e-29);
        check_reciprocal(999.0, 0.);
        check_reciprocal(314159269.0, 0.);
    }

    /**
     * A basic test for determinant correctness
     */
    #[test]
    fn test_determinant() {
        check_determinant(3., 8., 4., 6., -14., 0.);
        check_determinant_dd(3., 8., 4., 6., -14., 0.);
    }

    #[test]
    fn test_determinant_robust() {
        check_determinant(1.0e9, 1.0e9 - 1., 1.0e9 - 1., 1.0e9 - 2., -1., 0.);
        check_determinant_dd(1.0e9, 1.0e9 - 1., 1.0e9 - 1., 1.0e9 - 2., -1., 0.);
    }

    fn check_determinant(x1: f64, y1: f64, x2: f64, y2: f64, expected: f64, err_bound: f64) {
        let det = &DD::determinant_xy_f64(x1, y1, x2, y2);
        check_error_bound(det, &DD::value_of_f64(expected), err_bound);
    }

    fn check_determinant_dd(x1: f64, y1: f64, x2: f64, y2: f64, expected: f64, err_bound: f64) {
        let det = &DD::determinant_xy_dd(
            &DD::value_of_f64(x1),
            &DD::value_of_f64(y1),
            &DD::value_of_f64(x2),
            &DD::value_of_f64(y2),
        );
        check_error_bound(det, &DD::value_of_f64(expected), err_bound);
    }

    #[test]
    fn test_binom() {
        check_binomial_square(100.0, 1.0);
        check_binomial_square(1000.0, 1.0);
        check_binomial_square(10000.0, 1.0);
        check_binomial_square(100000.0, 1.0);
        check_binomial_square(1000000.0, 1.0);
        check_binomial_square(1e8, 1.0);
        check_binomial_square(1e10, 1.0);
        check_binomial_square(1e14, 1.0);
        // Following call will fail, because it requires 32 digits of precision
        //  	checkBinomialSquare(1e16, 1.0);

        check_binomial_square(1e14, 291.0);
        check_binomial_square(5e14, 291.0);
        check_binomial_square(5e14, 345291.0);
    }

    fn check_add_mult2(dd: &DD) {
        let sum = dd.add_dd(dd);
        let prod = dd.multiply_dd(&DD::new_x(2.0));
        check_error_bound(&sum, &prod, 0.0);
    }

    fn check_multiply_divide(a: &DD, b: &DD, err_bound: f64) {
        let a2 = a.multiply_dd(b).divide_dd(b);
        check_error_bound(a, &a2, err_bound);
    }

    fn check_divide_multiply(a: &DD, b: &DD, err_bound: f64) {
        let a2 = a.divide_dd(b).multiply_dd(b);
        check_error_bound(a, &a2, err_bound);
    }

    pub fn delta(x: &DD, y: &DD) -> DD {
        return x.subtract_dd(y).abs();
    }

    fn check_error_bound(x: &DD, y: &DD, err_bound: f64) {
        let err = x.subtract_dd(y).abs();
        let is_within_eps = err.double_value() <= err_bound;
        assert!(is_within_eps);
    }

    /**
     * Computes (a+b)^2 in two different ways and compares the result.
     * For correct results, a and b should be integers.
     *
     * @param a
     * @param b
     */
    fn check_binomial_square(a: f64, b: f64) {
        // binomial square
        let add = DD::new_x(a);
        let bdd = DD::new_x(b);
        let a_plus_b = add.add_dd(&bdd);
        let ab_sq = a_plus_b.multiply_dd(&a_plus_b);

        // expansion
        let a2dd = add.multiply_dd(&add);
        let b2dd = bdd.multiply_dd(&bdd);
        let ab = add.multiply_dd(&bdd);
        let sum = b2dd.add_dd(&ab).add_dd(&ab);

        let diff = ab_sq.subtract_dd(&a2dd);
        let delta = diff.subtract_dd(&sum);

        let is_same = diff.equals(&sum);
        assert!(is_same);
        let is_delta_zero = delta.is_zero();
        assert!(is_delta_zero);
    }

    fn test_binomial2() {
        check_binomial2(100.0, 1.0);
        check_binomial2(1000.0, 1.0);
        check_binomial2(10000.0, 1.0);
        check_binomial2(100000.0, 1.0);
        check_binomial2(1000000.0, 1.0);
        check_binomial2(1e8, 1.0);
        check_binomial2(1e10, 1.0);
        check_binomial2(1e14, 1.0);

        check_binomial2(1e14, 291.0);

        check_binomial2(5e14, 291.0);
        check_binomial2(5e14, 345291.0);
    }

    fn check_binomial2(a: f64, b: f64) {
        // binomial product
        let add = DD::new_x(a);
        let bdd = DD::new_x(b);
        let a_plus_b = add.add_dd(&bdd);
        let a_subb = add.subtract_dd(&bdd);
        let ab_prod = a_plus_b.multiply_dd(&a_subb);

        // expansion
        let a2dd = add.multiply_dd(&add);
        let b2dd = bdd.multiply_dd(&bdd);

        // this should equal b^2
        let diff = ab_prod.subtract_dd(&a2dd).negate();
        let delta = diff.subtract_dd(&b2dd);

        let is_same = diff.equals(&b2dd);
        assert!(is_same);
        let is_delta_zero = delta.is_zero();
        assert!(is_delta_zero);
    }

    fn check_reciprocal(x: f64, err_bound: f64) {
        let xdd = DD::new_x(x);
        let rr = xdd.reciprocal().reciprocal();

        let err = xdd.subtract_dd(&rr).double_value();

        assert!(err <= err_bound);
    }

    fn check_pow(x: f64, exp: i32, err_bound: f64) {
        let xdd = DD::new_x(x);
        let pow = xdd.pow(exp);
        //System.out.println("Pow(" + x + ", " + exp + ") = " + pow);
        let pow2 = slow_pow(&xdd, exp);

        let err = pow.subtract_dd(&pow2).double_value();
        assert!(err <= err_bound);
    }

    fn slow_pow(x: &DD, exp: i32) -> DD {
        if exp == 0 {
            return DD::value_of_f64(1.0);
        }

        let n = i32::abs(exp);
        // MD - could use binary exponentiation for better precision & speed
        let mut pow = DD::new_from_dd(x);
        for _i in 1..n {
            pow = pow.multiply_dd(x);
        }
        if exp < 0 {
            return pow.reciprocal();
        }
        return pow;
    }
}
