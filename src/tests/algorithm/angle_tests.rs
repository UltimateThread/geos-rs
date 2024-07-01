#[cfg(test)]
mod angle_tests {
    use crate::core::{
        algorithm::{angle::Angle, orientation::Orientation},
        geom::{
            coordinate::Coordinate, coordinate_array_sequences::CoordinateArraySequences, envelope::Envelope, geometry_factory::GeometryFactory, implementation::coordinate_array_sequence_factory::CoordinateArraySequenceFactory, multi_point::MultiPoint, precision_model::PrecisionModel
        },
    };
    use rand::Rng;

    const TOLERANCE: f64 = 1E-5;

    #[test]
    fn test_angle() {
        assert_eq!(Angle::angle_coordinate(&p(10., 0.)), 0.0);
        assert_eq!(
            Angle::angle_coordinate(&p(10., 10.)),
            std::f64::consts::PI / 4.
        );
        assert_eq!(
            Angle::angle_coordinate(&p(0., 10.)),
            std::f64::consts::PI / 2.
        );
        assert_eq!(
            Angle::angle_coordinate(&p(-10., 10.)),
            0.75 * std::f64::consts::PI
        );
        assert_eq!(Angle::angle_coordinate(&p(-10., 0.)), std::f64::consts::PI);
        assert_eq!(Angle::angle_coordinate(&p(-10., -0.1)), -3.131592986903128);
        assert_eq!(
            Angle::angle_coordinate(&p(-10., -10.)),
            -0.75 * std::f64::consts::PI
        );
    }

    #[test]
    fn test_is_acute() {
        assert_eq!(Angle::is_acute(&p(10., 0.), &p(0., 0.), &p(5., 10.)), true);
        assert_eq!(Angle::is_acute(&p(10., 0.), &p(0., 0.), &p(5., -10.)), true);
        // angle of 0
        assert_eq!(Angle::is_acute(&p(10., 0.), &p(0., 0.), &p(10., 0.)), true);

        assert_eq!(
            Angle::is_acute(&p(10., 0.), &p(0., 0.), &p(-5., 10.)),
            false
        );
        assert_eq!(
            Angle::is_acute(&p(10., 0.), &p(0., 0.), &p(-5., -10.)),
            false
        );
    }

    #[test]
    fn test_normalize_positive() {
        assert_eq!(Angle::normalize_positive(0.0), 0.0);

        assert_eq!(
            Angle::normalize_positive(-0.5 * std::f64::consts::PI),
            1.5 * std::f64::consts::PI
        );
        assert_eq!(
            Angle::normalize_positive(-std::f64::consts::PI),
            std::f64::consts::PI
        );
        assert_eq!(
            Angle::normalize_positive(-1.5 * std::f64::consts::PI),
            0.5 * std::f64::consts::PI
        );
        assert_eq!(Angle::normalize_positive(-2. * std::f64::consts::PI), 0.0);
        assert_eq!(
            Angle::normalize_positive(-2.5 * std::f64::consts::PI),
            1.5 * std::f64::consts::PI
        );
        assert_eq!(
            Angle::normalize_positive(-3. * std::f64::consts::PI),
            std::f64::consts::PI
        );
        assert_eq!(Angle::normalize_positive(-4. * std::f64::consts::PI), 0.0);

        assert_eq!(
            Angle::normalize_positive(0.5 * std::f64::consts::PI),
            0.5 * std::f64::consts::PI
        );
        assert_eq!(
            Angle::normalize_positive(std::f64::consts::PI),
            std::f64::consts::PI
        );
        assert_eq!(
            Angle::normalize_positive(1.5 * std::f64::consts::PI),
            1.5 * std::f64::consts::PI
        );
        assert_eq!(Angle::normalize_positive(2. * std::f64::consts::PI), 0.0);
        assert_eq!(
            Angle::normalize_positive(2.5 * std::f64::consts::PI),
            0.5 * std::f64::consts::PI
        );
        assert_eq!(
            Angle::normalize_positive(3. * std::f64::consts::PI),
            std::f64::consts::PI
        );
        assert_eq!(Angle::normalize_positive(4. * std::f64::consts::PI), 0.0);
    }

    #[test]
    fn test_normalize() {
        assert_eq!(Angle::normalize(0.0), 0.0);

        assert_eq!(
            Angle::normalize(-0.5 * std::f64::consts::PI),
            -0.5 * std::f64::consts::PI
        );
        assert_eq!(
            Angle::normalize(-std::f64::consts::PI),
            std::f64::consts::PI
        );
        assert_eq!(
            Angle::normalize(-1.5 * std::f64::consts::PI),
            0.5 * std::f64::consts::PI
        );
        assert_eq!(Angle::normalize(-2. * std::f64::consts::PI), 0.0);
        assert_eq!(
            Angle::normalize(-2.5 * std::f64::consts::PI),
            -0.5 * std::f64::consts::PI
        );
        assert_eq!(
            Angle::normalize(-3. * std::f64::consts::PI),
            std::f64::consts::PI
        );
        assert_eq!(Angle::normalize(-4. * std::f64::consts::PI), 0.0);

        assert_eq!(
            Angle::normalize(0.5 * std::f64::consts::PI),
            0.5 * std::f64::consts::PI
        );
        assert_eq!(Angle::normalize(std::f64::consts::PI), std::f64::consts::PI);
        assert_eq!(
            Angle::normalize(1.5 * std::f64::consts::PI),
            -0.5 * std::f64::consts::PI
        );
        assert_eq!(Angle::normalize(2. * std::f64::consts::PI), 0.0);
        assert_eq!(
            Angle::normalize(2.5 * std::f64::consts::PI),
            0.5 * std::f64::consts::PI
        );
        assert_eq!(
            Angle::normalize(3. * std::f64::consts::PI),
            std::f64::consts::PI
        );
        assert_eq!(Angle::normalize(4. * std::f64::consts::PI), 0.0);
    }

    #[test]
    fn test_interior_angle() {
        let p1 = p(1., 2.);
        let p2 = p(3., 2.);
        let p3 = p(2., 1.);

        // Tests all interior angles of a triangle "POLYGON ((1 2, 3 2, 2 1, 1 2))"
        assert_eq!(45., f64::to_degrees(Angle::interior_angle(&p1, &p2, &p3)));
        assert_eq!(90., f64::to_degrees(Angle::interior_angle(&p2, &p3, &p1)));
        assert_eq!(45., f64::to_degrees(Angle::interior_angle(&p3, &p1, &p2)));
        // Tests interior angles greater than 180 degrees
        assert_eq!(315., f64::to_degrees(Angle::interior_angle(&p3, &p2, &p1)));
        assert_eq!(270., f64::to_degrees(Angle::interior_angle(&p1, &p3, &p2)));
        assert_eq!(315., f64::to_degrees(Angle::interior_angle(&p2, &p1, &p3)));
    }

    /**
     * Tests interior angle calculation using a number of random triangles
     */
    #[test]
    fn test_interior_angle_random_triangles() {
        for _i in 0..100 {
            let three_random_points = get_random_points(3);
            let triangle = GeometryFactory::create_polygon_with_coordinate_array_sequence(
                &CoordinateArraySequences::ensure_valid_ring(
                    &CoordinateArraySequenceFactory::create_from_coordinates(
                        &three_random_points.get_coordinates(),
                    ),
                ),
            );
            // Triangle coordinates in clockwise order
            let c: Vec<Coordinate>;
            let triangle_coords = triangle.get_coordinates();
            let is_ccw = Orientation::is_ccw_vec(&triangle_coords);
            if is_ccw {
                c = triangle.reverse().get_coordinates();
            } else {
                c = triangle.get_coordinates();
            }

            let ia1 = Angle::interior_angle(&c[0], &c[1], &c[2]);
            let ia2 = Angle::interior_angle(&c[1], &c[2], &c[0]);
            let ia3 = Angle::interior_angle(&c[2], &c[0], &c[1]);

            let sum_of_interior_angles = ia1 + ia2 + ia3;
            println!("{}", sum_of_interior_angles);
            if sum_of_interior_angles > 3.15 {
                println!("")
            }
            assert!(
                sum_of_interior_angles >= (std::f64::consts::PI - 0.01)
                    && sum_of_interior_angles <= (std::f64::consts::PI + 0.01)
            );
        }
    }

    fn get_random_points(num_pts: usize) -> MultiPoint {
        let mut pts: Vec<Coordinate> = vec![Coordinate::default(); num_pts];
        let mut i = 0;
        while i < num_pts {
            let env = Envelope::new_xy(0., 1., 0., 1.);
            let mut rng = rand::thread_rng();
            let x = env.get_min_x() + env.get_width() * rng.gen_range(0.0..1.0);
            let y = env.get_min_y() + env.get_height() * rng.gen_range(0.0..1.0);
            let p = create_coord(x, y);

            pts[i] = p;
            i += 1;
        }
        return GeometryFactory::create_multi_point_with_coordinates(&pts);
    }

    fn create_coord(x: f64, y: f64) -> Coordinate {
        let mut pt = Coordinate::new_xy(x, y);
        PrecisionModel::default().make_precise_coordinate(&mut pt);
        return pt;
    }

    #[test]
    fn test_angle_bisector() {
        assert_eq!(
            45.,
            f64::to_degrees(Angle::bisector(&p(0., 1.), &p(0., 0.), &p(1., 0.)))
        );
        assert_eq!(
            22.5,
            f64::to_degrees(Angle::bisector(&p(1., 1.), &p(0., 0.), &p(1., 0.)))
        );
        assert_eq!(
            67.5,
            f64::to_degrees(Angle::bisector(&p(-1., 1.), &p(0., 0.), &p(1., 0.)))
        );
        assert_eq!(
            -45.,
            f64::to_degrees(Angle::bisector(&p(0., -1.), &p(0., 0.), &p(1., 0.)))
        );
        assert_eq!(
            180.,
            f64::to_degrees(Angle::bisector(&p(-1., -1.), &p(0., 0.), &p(-1., 1.)))
        );

        assert_eq!(
            45.,
            f64::to_degrees(Angle::bisector(&p(13., 10.), &p(10., 10.), &p(10., 20.)))
        );
    }

    #[test]
    fn test_sin_cos_snap() {
        // -720 to 720 degrees with 1 degree increments
        for angdeg in -720..=720 {
            let ang = Angle::to_radians(angdeg as f64);

            let r_sin = Angle::sin_snap(ang);
            let r_cos = Angle::cos_snap(ang);

            let c_sin = f64::sin(ang);
            let c_cos = f64::cos(ang);
            if (angdeg % 90) == 0 {
                // not always the same for multiples of 90 degrees
                assert!(f64::abs(r_sin - c_sin) < 1e-15);
                assert!(f64::abs(r_cos - c_cos) < 1e-15);
            } else {
                assert_eq!(r_sin, c_sin);
                assert_eq!(r_cos, c_cos);
            }
        }

        // use radian increments that don't snap to exact degrees or zero
        let mut angrad = -6.3;
        while angrad < 6.3 {
            let r_sin = Angle::sin_snap(angrad);
            let r_cos = Angle::cos_snap(angrad);

            assert_eq!(r_sin, f64::sin(angrad));
            assert_eq!(r_cos, f64::cos(angrad));

            angrad += 0.013;
        }
    }

    fn p(x: f64, y: f64) -> Coordinate {
        return Coordinate::new_xy(x, y);
    }
}
