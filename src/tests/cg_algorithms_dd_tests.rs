#[cfg(test)]
mod cg_algorithms_dd_tests {
    use crate::core::algorithm::cg_algorithms_dd::CGAlgorithmsDD;

    #[test]
    fn test_sign_of_det2x2() {
        check_sign_of_det2x2(1., 1., 2., 2., 0);
        check_sign_of_det2x2(1., 1., 2., 3., 1);
        check_sign_of_det2x2(1., 1., 3., 2., -1);
    }

    fn check_sign_of_det2x2(x1: f64, y1: f64, x2: f64, y2: f64, sign: i32) {
        assert_eq!(sign, CGAlgorithmsDD::sign_of_det2x2_f64(x1, y1, x2, y2));
    }
}
