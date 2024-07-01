#[cfg(test)]
mod distance_tests {
    use crate::core::{algorithm::distance::Distance, geom::coordinate::Coordinate};

    #[test]
    fn test_distance_point_line_perpendicular() {
        equals_with_tolerance(
            0.5,
            Distance::point_to_line_perpendicular(
                &Coordinate::new_xy(0.5, 0.5),
                &Coordinate::new_xy(0., 0.),
                &Coordinate::new_xy(1., 0.),
            ),
            0.000001,
        );
        equals_with_tolerance(
            0.5,
            Distance::point_to_line_perpendicular(
                &Coordinate::new_xy(3.5, 0.5),
                &Coordinate::new_xy(0., 0.),
                &Coordinate::new_xy(1., 0.),
            ),
            0.000001,
        );
        equals_with_tolerance(
            0.707106,
            Distance::point_to_line_perpendicular(
                &Coordinate::new_xy(1., 0.),
                &Coordinate::new_xy(0., 0.),
                &Coordinate::new_xy(1., 1.),
            ),
            0.000001,
        );
    }

    #[test]
    fn test_distance_point_line() {
        equals_with_tolerance(
            0.5,
            Distance::point_to_segment(
                &Coordinate::new_xy(0.5, 0.5),
                &Coordinate::new_xy(0., 0.),
                &Coordinate::new_xy(1., 0.),
            ),
            0.000001,
        );
        equals_with_tolerance(
            1.0,
            Distance::point_to_segment(
                &Coordinate::new_xy(2., 0.),
                &Coordinate::new_xy(0., 0.),
                &Coordinate::new_xy(1., 0.),
            ),
            0.000001,
        );
    }

    #[test]
    fn test_distance_line_line_disjoint_collinear() {
        equals_with_tolerance(
            1.999699,
            Distance::segment_to_segment(
                &Coordinate::new_xy(0., 0.),
                &Coordinate::new_xy(9.9, 1.4),
                &Coordinate::new_xy(11.88, 1.68),
                &Coordinate::new_xy(21.78, 3.08),
            ),
            0.000001,
        );
    }

    fn equals_with_tolerance(distance: f64, expected: f64, tolerance: f64) {
        assert!(distance >= expected - tolerance && distance <= expected + tolerance)
    }
}
