#[cfg(test)]
mod vector_2d_tests {
    use crate::core::math::vector_2d::Vector2D;

    const TOLERANCE: f64 = 1E-5;

    #[test]
    fn test_length() {
        assert_equals_with_tolerance(Vector2D::create_from_xy(0., 1.).length(), 1.0, TOLERANCE);
        assert_equals_with_tolerance(Vector2D::create_from_xy(0., -1.).length(), 1.0, TOLERANCE);
        assert_equals_with_tolerance(
            Vector2D::create_from_xy(1., 1.).length(),
            f64::sqrt(2.0),
            TOLERANCE,
        );
        assert_equals_with_tolerance(Vector2D::create_from_xy(3., 4.).length(), 5., TOLERANCE);
    }

    #[test]
    pub fn test_is_parallel() {
        assert!(Vector2D::create_from_xy(0., 1.).is_parallel(&Vector2D::create_from_xy(0., 2.)));
        assert!(Vector2D::create_from_xy(1., 1.).is_parallel(&Vector2D::create_from_xy(2., 2.)));
        assert!(Vector2D::create_from_xy(-1., -1.).is_parallel(&Vector2D::create_from_xy(2., 2.)));
        assert!(!Vector2D::create_from_xy(1., -1.).is_parallel(&Vector2D::create_from_xy(2., 2.)));
    }

    #[test]
    fn test_to_coordinate() {
        assert_eq_vector_2d_with_tolerance(
            &&Vector2D::create_from_coodinate(&Vector2D::create_from_xy(1., 2.).to_coordinate()),
            &Vector2D::create_from_xy(1., 2.),
            TOLERANCE,
        );
    }

    fn assert_eq_vector_2d_with_tolerance(v1: &Vector2D, v2: &Vector2D, tolerance: f64) {
        assert_equals_with_tolerance(v1.get_x(), v2.get_x(), tolerance);
        assert_equals_with_tolerance(v1.get_y(), v2.get_y(), tolerance);
    }

    fn assert_equals_with_tolerance(a: f64, b: f64, tolerance: f64) {
        assert!(a >= b - tolerance && a <= b + tolerance);
    }
}
