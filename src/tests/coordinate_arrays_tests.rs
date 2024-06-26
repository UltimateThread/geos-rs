#[cfg(test)]
mod coordinate_arrays_tests {
    use crate::core::geom::{coordinate::Coordinate, coordinate_arrays::CoordinateArrays, envelope::Envelope, precision_model::PrecisionModel};

    #[test]
    fn test_pt_not_in_list1() {
        let v1 = vec![
            Coordinate::new_xy(1., 1.),
            Coordinate::new_xy(2., 2.),
            Coordinate::new_xy(3., 3.),
        ];
        let v2 = vec![
            Coordinate::new_xy(1., 1.),
            Coordinate::new_xy(1., 2.),
            Coordinate::new_xy(1., 3.),
        ];
        let coord = Coordinate::new_xy(2., 2.);
        let existing = CoordinateArrays::pt_not_in_list(&v1, &v2);
        match existing {
            Some(existing) => assert!(existing.equals_2d(&coord)),
            None => panic!(),
        }
    }

    #[test]
    fn test_pt_not_in_list2() {
        let v1 = vec![
            Coordinate::new_xy(1., 1.),
            Coordinate::new_xy(2., 2.),
            Coordinate::new_xy(3., 3.),
        ];
        let v2 = vec![
            Coordinate::new_xy(1., 1.),
            Coordinate::new_xy(2., 2.),
            Coordinate::new_xy(3., 3.),
        ];
        assert!(CoordinateArrays::pt_not_in_list(&v1, &v2).is_none());
    }

    #[test]
    fn test_envelope1() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xy(1., 1.),
            Coordinate::new_xy(2., 2.),
            Coordinate::new_xy(3., 3.),
        ];
        let coords_envelope = CoordinateArrays::envelope(&coords1);
        let envelope = Envelope::new_xy(1., 3., 1., 3.);

        compare_envelopes(&coords_envelope, &envelope);
    }

    #[test]
    fn test_envelope_empty() {
        let empty: Vec<Coordinate> = vec![];
        let empty_envelope = CoordinateArrays::envelope(&empty);
        let envelope = Envelope::default();

        compare_envelopes(&empty_envelope, &envelope);
    }

    #[test]
    fn test_intersection_envelope1() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xy(1., 1.),
            Coordinate::new_xy(2., 2.),
            Coordinate::new_xy(3., 3.),
        ];
        let envelope = Envelope::new_xy(1., 2., 1., 2.);
        let envelope_vec: Vec<Coordinate> =
            vec![Coordinate::new_xy(1., 1.), Coordinate::new_xy(2., 2.)];

        let intersection = CoordinateArrays::intersection(&coords1, &envelope);

        assert!(CoordinateArrays::equals(&intersection, &envelope_vec));
    }

    #[test]
    fn test_intersection_envelope_disjoint() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xy(1., 1.),
            Coordinate::new_xy(2., 2.),
            Coordinate::new_xy(3., 3.),
        ];
        let coords_empty: Vec<Coordinate> = vec![];
        let envelope = Envelope::new_xy(10., 20., 10., 20.);

        assert!(CoordinateArrays::equals(
            &CoordinateArrays::intersection(&coords1, &envelope),
            &coords_empty
        ));
    }

    #[test]
    fn test_intersection_empty_envelope() {
        let coords_empty: Vec<Coordinate> = vec![];
        let envelope = Envelope::new_xy(1., 2., 1., 2.);

        assert!(CoordinateArrays::equals(
            &CoordinateArrays::intersection(&coords_empty, &envelope),
            &coords_empty
        ));
    }

    #[test]
    fn test_intersection_coords_empty_envelope() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xy(1., 1.),
            Coordinate::new_xy(2., 2.),
            Coordinate::new_xy(3., 3.),
        ];
        let coords_empty: Vec<Coordinate> = vec![];
        let envelope = Envelope::default();

        assert!(CoordinateArrays::equals(
            &CoordinateArrays::intersection(&coords1, &envelope),
            &coords_empty
        ));
    }

    #[test]
    fn test_scroll_ring() {
        // arrange
        let sequence: Vec<Coordinate> = create_circle(Coordinate::new_xy(10., 10.), 9.);
        let mut scrolled: Vec<Coordinate> = create_circle(Coordinate::new_xy(10., 10.), 9.);

        // act
        CoordinateArrays::scroll_index(&mut scrolled, 12);

        // assert
        let mut io = 12;
        let mut is = 0;
        while is < scrolled.len() - 1 {
            check_coordinate_at(&sequence, io, &scrolled, is);
            io += 1;
            io %= scrolled.len() - 1;

            is += 1
        }
        check_coordinate_at(&scrolled, 0, &scrolled, scrolled.len() - 1);
    }

    #[test]
    fn test_scroll() {
        // arrange
        let sequence: Vec<Coordinate> =
            create_circular_string(Coordinate::new_xy(20., 20.), 7., 0.1, 22);
        let mut scrolled: Vec<Coordinate> =
            create_circular_string(Coordinate::new_xy(20., 20.), 7., 0.1, 22);

        // act
        CoordinateArrays::scroll_index(&mut scrolled, 12);

        // assert
        let mut io = 12;
        let mut is = 0;
        while is < scrolled.len() - 1 {
            check_coordinate_at(&sequence, io, &scrolled, is);
            io += 1;
            io %= scrolled.len();

            is += 1;
        }
    }

    #[test]
    fn test_enforce_consistency() {
        let mut array: Vec<Coordinate> = vec![
            Coordinate::new_xyz(1.0, 1.0, 0.0),
            Coordinate::new_xym(2.0, 2.0, 1.0),
        ];
        let mut array2: Vec<Coordinate> =
            vec![Coordinate::new_xy(1.0, 1.0), Coordinate::new_xy(2.0, 2.0)];
        // process into array with dimension 4 and measures 1
        CoordinateArrays::enforce_consistency(&mut array);
        let dimension = CoordinateArrays::dimension(&array);
        let measures = CoordinateArrays::measures(&array);
        assert_eq!(4, dimension);
        assert_eq!(1, measures);

        CoordinateArrays::enforce_consistency(&mut array2);

        let mut fixed = CoordinateArrays::enforce_consistency_dim_measures(&array2, 2, 0);
        assert_eq!(fixed.len(), array2.len());
        for i in 0..fixed.len() {
            assert_eq!(fixed[i].get_x(), array2[i].get_x());
            assert_eq!(fixed[i].get_x(), array2[i].get_x());
            assert!(f64::is_nan(fixed[i].get_z()) && f64::is_nan(array2[i].get_z()));
            assert!(f64::is_nan(fixed[i].get_m()) && f64::is_nan(array2[i].get_m()));
        }

        fixed = CoordinateArrays::enforce_consistency_dim_measures(&array, 3, 0);
        assert!(fixed.as_ptr() != array.as_ptr()); // copied into new array

        let equals0 = fixed[0].get_x() == array[0].get_x()
            && fixed[0].get_x() == array[0].get_x()
            && fixed[0].get_z() == array[0].get_z()
            && fixed[0].get_m() == array[0].get_m();
        assert!(!equals0);

        let equals1 = fixed[1].get_x() == array[1].get_x()
            && fixed[1].get_x() == array[1].get_x()
            && fixed[1].get_z() == array[1].get_z()
            && fixed[1].get_m() == array[0].get_m();
        assert!(!equals1);
    }

    fn check_coordinate_at(
        seq1: &Vec<Coordinate>,
        pos1: usize,
        seq2: &Vec<Coordinate>,
        pos2: usize,
    ) {
        let c1 = seq1[pos1];
        let c2 = seq2[pos2];

        assert_eq!(c1.get_x(), c2.get_x());
        assert_eq!(c1.get_y(), c2.get_y());
    }

    fn create_circle(center: Coordinate, radius: f64) -> Vec<Coordinate> {
        // Get a complete circular string
        let mut res: Vec<Coordinate> = create_circular_string(center, radius, 0., 49);

        // ensure it is closed
        res[48] = res[0];

        return res;
    }

    fn create_circular_string(
        center: Coordinate,
        radius: f64,
        start_angle: f64,
        num_points: usize,
    ) -> Vec<Coordinate> {
        let num_segments_circle = 48.;
        let angle_circle = 2. * std::f64::consts::PI;
        let angle_step = angle_circle / num_segments_circle;

        let mut sequence: Vec<Coordinate> = vec![Coordinate::default(); num_points];
        let mut pm = PrecisionModel::new_with_scale(1000.);
        let mut angle = start_angle;
        for i in 0..num_points {
            let dx = f64::cos(angle) * radius;
            let dy = f64::sin(angle) * radius;
            sequence[i] = Coordinate::new_xy(
                pm.make_precise(center.x + dx),
                pm.make_precise(center.y + dy),
            );

            angle += angle_step;
            angle %= angle_circle;
        }

        return sequence;
    }

    fn compare_envelopes(env1: &Envelope, env2: &Envelope) {
        assert_eq!(env1.get_min_x(), env2.get_min_x());
        assert_eq!(env1.get_max_x(), env2.get_max_x());
        assert_eq!(env1.get_min_y(), env2.get_min_y());
        assert_eq!(env1.get_max_y(), env2.get_max_y());
        assert_eq!(env1.get_area(), env2.get_area());
        assert_eq!(env1.get_diameter(), env2.get_diameter());
        assert_eq!(env1.get_width(), env2.get_width());
        assert_eq!(env1.get_height(), env2.get_height());
    }
}
