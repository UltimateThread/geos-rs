#[cfg(test)]
mod vector_3d_tests {
    use crate::core::{geom::coordinate::Coordinate, math::vector_3d::Vector3D};

    const TOLERANCE: f64 = 1E-5;

    #[test]
    fn test_length() {
        assert_equals_with_tolerance(
            1.0,
            Vector3D::create_from_xyz(0., 1., 0.).length(),
            TOLERANCE,
        );
        assert_equals_with_tolerance(
            1.0,
            Vector3D::create_from_xyz(0., -1., 0.).length(),
            TOLERANCE,
        );
        assert_equals_with_tolerance(
            f64::sqrt(2.0),
            Vector3D::create_from_xyz(1., 1., 0.).length(),
            TOLERANCE,
        );
        assert_equals_with_tolerance(
            5.,
            Vector3D::create_from_xyz(3., 4., 0.).length(),
            TOLERANCE,
        );
        assert_equals_with_tolerance(
            f64::sqrt(3.),
            Vector3D::create_from_xyz(1., 1., 1.).length(),
            TOLERANCE,
        );
        assert_equals_with_tolerance(
            f64::sqrt(1. + 4. + 9.),
            Vector3D::create_from_xyz(1., 2., 3.).length(),
            TOLERANCE,
        );
    }

    #[test]
    fn test_add() {
        assert_equals_vectors(
            &Vector3D::create_from_xyz(5., 7., 9.),
            &Vector3D::create_from_xyz(1., 2., 3.).add(&Vector3D::create_from_xyz(4., 5., 6.)),
        );
    }

    #[test]
    fn test_subtract() {
        assert_equals_vectors(
            &Vector3D::create_from_xyz(-3., 0., 3.),
            &Vector3D::create_from_xyz(1., 5., 9.).subtract(&Vector3D::create_from_xyz(4., 5., 6.)),
        );
    }

    #[test]
    fn test_divide() {
        assert_equals_vectors(
            &Vector3D::create_from_xyz(1., 2., 3.),
            &Vector3D::create_from_xyz(2., 4., 6.).divide(2.),
        );
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            20.0,
            Vector3D::create_from_xyz(2., 3., 4.).dot(&Vector3D::create_from_xyz(1., 2., 3.))
        );
    }

    #[test]
    fn test_dot_abcd() {
        let dot = Vector3D::dot_4(
            &Coordinate::new_xyz(2., 3., 4.),
            &Coordinate::new_xyz(3., 4., 5.),
            &Coordinate::new_xyz(0., 1., -1.),
            &Coordinate::new_xyz(1., 5., 2.),
        );
        assert_eq!(8.0, dot);
        assert_eq!(
            dot,
            Vector3D::new_from_xyz(1., 1., 1.).dot(&Vector3D::new_from_xyz(1., 4., 3.))
        );
    }

    #[test]
    fn test_normalize() {
        assert_equals_vectors(
            &Vector3D::create_from_xyz(-0.5773502691896258, 0.5773502691896258, 0.5773502691896258),
            &Vector3D::create_from_xyz(-1., 1., 1.).normalize(),
        );
        assert_equals_vectors(
            &Vector3D::create_from_xyz(0.5773502691896258, 0.5773502691896258, 0.5773502691896258),
            &Vector3D::create_from_xyz(2., 2., 2.).normalize(),
        );
        assert_equals_vectors(
            &Vector3D::create_from_xyz(0.2672612419124244, 0.5345224838248488, 0.8017837257372732),
            &Vector3D::create_from_xyz(1., 2., 3.).normalize(),
        );
    }

    fn assert_equals_vectors(expected: &Vector3D, actual: &Vector3D) {
        let is_equal = expected.equals(actual);
        assert!(is_equal);
    }

    fn assert_equals_vectors_with_tolerance(
        expected: &Vector3D,
        actual: &Vector3D,
        tolerance: f64,
    ) {
        assert_equals_with_tolerance(expected.get_x(), actual.get_x(), tolerance);
        assert_equals_with_tolerance(expected.get_y(), actual.get_y(), tolerance);
        assert_equals_with_tolerance(expected.get_z(), actual.get_z(), tolerance);
    }

    fn assert_equals_with_tolerance(a: f64, b: f64, tolerance: f64) {
        assert!(a >= b - tolerance && a <= b + tolerance);
    }
}
