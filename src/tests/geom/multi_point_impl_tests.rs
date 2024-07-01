#[cfg(test)]
mod multi_point_impl_tests {
    use crate::core::geom::{coordinate::Coordinate, multi_point::MultiPoint, point::Point};

    #[test]
    fn test_get_geometry_n() {
        let points: Vec<Point> = vec![
            Point::new_with_coordinate(&Coordinate::new_xy(1.111, 2.222), None),
            Point::new_with_coordinate(&Coordinate::new_xy(3.333, 4.444), None),
            Point::new_with_coordinate(&Coordinate::new_xy(3.333, 4.444), None),
        ];
        let m = MultiPoint::new_with_points(&points);
        let p = m.get_point_at_index(1);
        assert!(p.is_some());
        let mut external_coordinate = Coordinate::default();
        let internal = p.unwrap().get_coordinate();
        assert!(internal.is_some());
        external_coordinate.x = internal.unwrap().x;
        external_coordinate.y = internal.unwrap().y;
        assert_eq!(3.333, external_coordinate.x);
        assert_eq!(4.444, external_coordinate.y);
    }

    #[test]
    fn test_get_envelope() {
        let points: Vec<Point> = vec![
            Point::new_with_coordinate(&Coordinate::new_xy(1.111, 2.222), None),
            Point::new_with_coordinate(&Coordinate::new_xy(3.333, 4.444), None),
            Point::new_with_coordinate(&Coordinate::new_xy(3.333, 4.444), None),
        ];
        let mut m = MultiPoint::new_with_points(&points);
        let e = m.get_envelope_internal();
        assert_eq!(1.111, e.get_min_x());
        assert_eq!(3.333, e.get_max_x());
        assert_eq!(2.222, e.get_min_y());
        assert_eq!(4.444, e.get_max_y());
    }

    // TODO: Implment ME!
    // #[test]
    //   fn test_equals() {
    //     let points: Vec<Point> = vec![
    //         Point::new_with_coordinate(&Coordinate::new_xy(5., 6.), None),
    //         Point::new_with_coordinate(&Coordinate::new_xy(7., 8.), None),
    //     ];
    //     let mut m1 = MultiPoint::new_with_points(&points);
    //     let mut m2 = MultiPoint::new_with_points(&points);
    //     assert!(m1.equals(&mut m2));
    //   }
}
