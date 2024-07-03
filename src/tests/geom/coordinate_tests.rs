#[cfg(test)]
mod coordinate_tests {
    use crate::core::geom::coordinate::Coordinate;

    #[test]
    fn test_constructor_3d() {
        let c = Coordinate::new_xyz(350.2, 4566.8, 5266.3);
        assert_eq!(c.x, 350.2);
        assert_eq!(c.y, 4566.8);
        assert_eq!(c.get_z(), 5266.3);
    }

    #[test]
    fn test_constructor_2d() {
        let c = Coordinate::new_xy(350.2, 4566.8);
        assert_eq!(c.x, 350.2);
        assert_eq!(c.y, 4566.8);
        assert!(f64::is_nan(c.get_z()));
    }

    #[test]
    fn test_default_constructor() {
        let c = Coordinate::default();
        assert_eq!(c.x, 0.0);
        assert_eq!(c.y, 0.0);
        assert!(f64::is_nan(c.get_z()));
    }

    #[test]
    pub fn test_copy_constructor_3d() {
        let orig = Coordinate::new_xyz(350.2, 4566.8, 5266.3);
        let c = Coordinate::from_coordinate(&orig);
        assert_eq!(c.x, 350.2);
        assert_eq!(c.y, 4566.8);
        assert_eq!(c.get_z(), 5266.3);
    }

    #[test]
    fn test_set_coordinate() {
        let orig = Coordinate::new_xyz(350.2, 4566.8, 5266.3);
        let mut c = Coordinate::default();
        c.set_coordinate(&orig);
        assert_eq!(c.x, 350.2);
        assert_eq!(c.y, 4566.8);
        assert_eq!(c.get_z(), 5266.3);
    }

    #[test]
    fn test_get_ordinate() {
        let c = Coordinate::new_xyz(350.2, 4566.8, 5266.3);
        assert_eq!(c.get_ordinate(Coordinate::X).unwrap(), 350.2);
        assert_eq!(c.get_ordinate(Coordinate::Y).unwrap(), 4566.8);
        assert_eq!(c.get_ordinate(Coordinate::Z).unwrap(), 5266.3);
    }

    #[test]
    fn test_set_ordinate() {
        let mut c = Coordinate::default();
        c.set_ordinate(Coordinate::X, 111.);
        c.set_ordinate(Coordinate::Y, 222.);
        c.set_ordinate(Coordinate::Z, 333.);
        assert_eq!(c.get_ordinate(Coordinate::X).unwrap(), 111.0);
        assert_eq!(c.get_ordinate(Coordinate::Y).unwrap(), 222.0);
        assert_eq!(c.get_ordinate(Coordinate::Z).unwrap(), 333.0);
    }

    #[test]
    fn test_equals_2d() {
        let c1 = Coordinate::new_xyz(1., 2., 3.);
        let c2 = Coordinate::new_xyz(1., 2., 3.);
        assert!(c1.equals_2d(&c2));

        let c3 = Coordinate::new_xyz(1., 22., 3.);
        assert!(!c1.equals_2d(&c3));
    }

    #[test]
    fn test_equals_3d() {
        let c1 = Coordinate::new_xyz(1., 2., 3.);
        let c2 = Coordinate::new_xyz(1., 2., 3.);
        assert!(c1.equals_3d(&c2));

        let c3 = Coordinate::new_xyz(1., 22., 3.);
        assert!(!c1.equals_3d(&c3));
    }

    #[test]
    fn test_equals_2d_within_tolerance() {
        let c = Coordinate::new_xyz(100.0, 200.0, 50.0);
        let a_bit_off = Coordinate::new_xyz(100.1, 200.1, 50.0);
        assert!(c.equals_2d_with_tolerance(&a_bit_off, 0.2));
    }

    #[test]
    fn test_equals_in_z() {
        let c = Coordinate::new_xyz(100.0, 200.0, 50.0);
        let with_same_z = Coordinate::new_xyz(100.1, 200.1, 50.1);
        assert!(c.equal_in_z(&with_same_z, 0.2));
    }

    #[test]
    fn test_compare_to() {
        let lowest = Coordinate::new_xyz(10.0, 100.0, 50.0);
        let highest = Coordinate::new_xyz(20.0, 100.0, 50.0);
        let equal_to_highest = Coordinate::new_xyz(20.0, 100.0, 50.0);
        let higher_still = Coordinate::new_xyz(20.0, 200.0, 50.0);

        assert_eq!(-1, lowest.compare_to(&highest));
        assert_eq!(1, highest.compare_to(&lowest));
        assert_eq!(-1, highest.compare_to(&higher_still));
        assert_eq!(0, highest.compare_to(&equal_to_highest));
    }

    #[test]
    fn test_to_string() {
        let expected_result = "(100.1, 200.2, 50.3)";
        let actual_result = Coordinate::new_xyz(100.1, 200.2, 50.3).to_string();
        assert_eq!(&expected_result, &actual_result);
    }

    #[test]
    fn test_clone() {
        let c = Coordinate::new_xyz(100.0, 200.0, 50.0);
        let clone = c;
        assert!(c.equals_3d(&clone));
    }

    #[test]
    fn test_distance() {
        let coord1 = Coordinate::new_xyz(0.0, 0.0, 0.0);
        let coord2 = Coordinate::new_xyz(100.0, 200.0, 50.0);
        let distance = coord1.distance(&coord2);
        assert_eq!(distance, 223.60679774997897);
    }

    #[test]
    fn test_distance_3d() {
        let coord1 = Coordinate::new_xyz(0.0, 0.0, 0.0);
        let coord2 = Coordinate::new_xyz(100.0, 200.0, 50.0);
        let distance = coord1.distance_3d(&coord2);
        assert_eq!(distance, 229.128784747792);
    }

    #[test]
    fn test_coordinatexy() {
        #[allow(unused_assignments)]
        let mut xy = Coordinate::new_coordinatexy_default();

        xy = Coordinate::new_coordinatexy(1.0, 1.0); // 2D
        let mut coord = Coordinate::from_coordinate(&xy); // copy
        assert!(xy.x == coord.x && xy.y == coord.y);

        coord = Coordinate::new_xyz(1.0, 1.0, 1.0); // 2.5d
        xy = Coordinate::from_coordinate(&coord); // copy
        assert!(xy.x == coord.x && xy.y == coord.y);
    }

    #[test]
    fn test_coordinatexym() {
        let mut xym = Coordinate::new_coordinatexym_default();

        xym.set_m(1.0);
        assert_eq!(1.0, xym.get_m());

        let mut coord = Coordinate::from_coordinate(&xym); // copy
        assert!(xym.x == coord.x && xym.y == coord.y);

        coord = Coordinate::new_xyz(1.0, 1.0, 1.0); // 2.5d
        xym = Coordinate::from_coordinate(&coord); // copy
        assert!(xym.x == coord.x && xym.y == coord.y);
    }

    #[test]
    fn test_coordinatexyzm() {
        let mut xyzm = Coordinate::new_coordinatexyzm_default();
        xyzm.set_z(1.0);
        assert_eq!(1.0, xyzm.get_z());
        xyzm.set_m(1.0);
        assert_eq!(1.0, xyzm.get_m());

        let mut coord = Coordinate::from_coordinate(&xyzm); // copy
        assert_eq!(xyzm.x, coord.x);
        assert_eq!(xyzm.y, coord.y);
        assert_eq!(xyzm.z, coord.z);
        assert_eq!(xyzm.m, coord.m);
        assert!(xyzm.equal_in_coordinate_z(&coord, 0.000001));

        coord = Coordinate::new_xyz(1.0, 1.0, 1.0); // 2.5d
        xyzm = Coordinate::from_coordinate(&coord); // copy
        assert_eq!(xyzm.x, coord.x);
        assert_eq!(xyzm.y, coord.y);
        assert_eq!(xyzm.z, coord.z);
        assert!(f64::is_nan(xyzm.m) && f64::is_nan(coord.m));
        assert!(xyzm.equal_in_coordinate_z(&coord, 0.000001));
    }
}