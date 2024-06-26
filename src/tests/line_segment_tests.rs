#[cfg(test)]
mod line_segment_tests {
    use crate::geom::{coordinate::Coordinate, line_segment::LineSegment};

    const MAX_ABS_ERROR_INTERSECTION: f64 = 1e-5;

    #[test]
    fn test_projection_factor() {
        // zero-length line
        let seg = LineSegment::new_from_xy(10., 0., 10., 0.);
        assert!(f64::is_nan(
            seg.projection_factor(&Coordinate::new_xy(11., 0.))
        ));

        let seg2 = LineSegment::new_from_xy(10., 0., 20., 0.);
        assert!(seg2.projection_factor(&Coordinate::new_xy(11., 0.)) == 0.1);
    }

    #[test]
    fn test_line_intersection() {
        // simple case
        check_line_intersection(0., 0., 10., 10., 0., 10., 10., 0., 5., 5.);

        //Almost collinear - See JTS GitHub issue #464
        check_line_intersection(
            35613471.6165017,
            4257145.306132293,
            35613477.7705378,
            4257160.528222711,
            35613477.77505724,
            4257160.539653536,
            35613479.85607389,
            4257165.92369170,
            35613477.772841461,
            4257160.5339209242,
        );
    }

    fn check_line_intersection(
        p1x: f64,
        p1y: f64,
        p2x: f64,
        p2y: f64,
        q1x: f64,
        q1y: f64,
        q2x: f64,
        q2y: f64,
        expectedx: f64,
        expectedy: f64,
    ) {
        let seg1 = LineSegment::new_from_xy(p1x, p1y, p2x, p2y);
        let seg2 = LineSegment::new_from_xy(q1x, q1y, q2x, q2y);

        let actual = seg1.line_intersection(seg2);
        assert!(actual.is_some());

        let expected = Coordinate::new_xy(expectedx, expectedy);
        let dist = actual.unwrap().distance(&expected);
        //System.out.println("Expected: " + expected + "  Actual: " + actual + "  Dist = " + dist);
        assert!(dist <= MAX_ABS_ERROR_INTERSECTION);
    }

    #[test]
    fn test_distance_perpendicular() {
        check_distance_perpendicular(1., 1., 1., 3., 2., 4., 1.);
        check_distance_perpendicular(1., 1., 1., 3., 0., 4., 1.);
        check_distance_perpendicular(1., 1., 1., 3., 1., 4., 0.);
        check_distance_perpendicular(1., 1., 2., 2., 4., 4., 0.);
        //-- zero-length line segment
        check_distance_perpendicular(1., 1., 1., 1., 1., 2., 1.);
    }

    #[test]
    fn test_distance_perpendicular_oriented() {
        //-- right of line
        check_distance_perpendicular_oriented(1., 1., 1., 3., 2., 4., -1.);
        //-- left of line
        check_distance_perpendicular_oriented(1., 1., 1., 3., 0., 4., 1.);
        //-- on line
        check_distance_perpendicular_oriented(1., 1., 1., 3., 1., 4., 0.);
        check_distance_perpendicular_oriented(1., 1., 2., 2., 4., 4., 0.);
        //-- zero-length segment
        check_distance_perpendicular_oriented(1., 1., 1., 1., 1., 2., 1.);
    }

    fn check_distance_perpendicular(
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        px: f64,
        py: f64,
        expected: f64,
    ) {
        let seg = LineSegment::new_from_xy(x0, y0, x1, y1);
        let dist = seg.distance_perpendicular(&Coordinate::new_xy(px, py));
        assert_eq!(expected, dist);
    }

    fn check_distance_perpendicular_oriented(
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        px: f64,
        py: f64,
        expected: f64,
    ) {
        let seg = LineSegment::new_from_xy(x0, y0, x1, y1);
        let dist = seg.distance_perpendicular_oriented(&Coordinate::new_xy(px, py));
        assert_eq!(expected, dist);
    }

    #[test]
    fn test_offset_point() {
        let root2: f64 = f64::sqrt(2.);

        check_offset_point(0., 0., 10., 10., 0.0, root2, -1., 1.);
        check_offset_point(0., 0., 10., 10., 0.0, -root2, 1., -1.);
        check_offset_point(0., 0., 10., 10., 1.0, root2, 9., 11.);
        check_offset_point(0., 0., 10., 10., 0.5, root2, 4., 6.);
        check_offset_point(0., 0., 10., 10., 0.5, -root2, 6., 4.);
        check_offset_point(0., 0., 10., 10., 0.5, -root2, 6., 4.);
        check_offset_point(0., 0., 10., 10., 2.0, root2, 19., 21.);
        check_offset_point(0., 0., 10., 10., 2.0, -root2, 21., 19.);
        check_offset_point(0., 0., 10., 10., 2.0, 5. * root2, 15., 25.);
        check_offset_point(0., 0., 10., 10., -2.0, 5. * root2, -25., -15.);
    }

    #[test]
    fn test_offset_line() {
        let root2: f64 = f64::sqrt(2.);

        check_offset_line(0., 0., 10., 10., 0., 0., 0., 10., 10.);
        check_offset_line(0., 0., 10., 10., root2, -1., 1., 9., 11.);
        check_offset_line(0., 0., 10., 10., -root2, 1., -1., 11., 9.);
    }

    fn check_offset_point(
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        seg_frac: f64,
        offset: f64,
        expected_x: f64,
        expected_y: f64,
    ) {
        let seg = LineSegment::new_from_xy(x0, y0, x1, y1);
        let p = seg.point_along_offset(seg_frac, offset);

        assert!(equals_tolerance(
            &Coordinate::new_xy(expected_x, expected_y),
            &p,
            0.000001
        ));
    }

    fn check_offset_line(
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        offset: f64,
        expected_x0: f64,
        expected_y0: f64,
        expected_x1: f64,
        expected_y1: f64,
    ) {
        let seg = LineSegment::new_from_xy(x0, y0, x1, y1);
        let actual = seg.offset(offset);

        assert!(equals_tolerance(
            &Coordinate::new_xy(expected_x0, expected_y0),
            &actual.p0,
            0.000001
        ));
        assert!(equals_tolerance(
            &Coordinate::new_xy(expected_x1, expected_y1),
            &actual.p1,
            0.000001
        ));
    }

    pub fn equals_tolerance(p0: &Coordinate, p1: &Coordinate, tolerance: f64) -> bool {
        if f64::abs(p0.x - p1.x) > tolerance {
            return false;
        }
        if f64::abs(p0.y - p1.y) > tolerance {
            return false;
        }
        return true;
    }

    #[test]
    fn test_reflect() {
        check_reflect(0., 0., 10., 10., 1., 2., 2., 1.);
        check_reflect(0., 1., 10., 1., 1., 2., 1., 0.);
    }

    fn check_reflect(
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        x: f64,
        y: f64,
        expected_x: f64,
        expected_y: f64,
    ) {
        let seg = LineSegment::new_from_xy(x0, y0, x1, y1);
        let p = seg.reflect(&Coordinate::new_xy(x, y));
        assert!(equals_tolerance(
            &Coordinate::new_xy(expected_x, expected_y),
            &p,
            0.000001
        ));
    }

    #[test]
    fn test_orientation_index_coordinate() {
        let seg = LineSegment::new_from_xy(0., 0., 10., 10.);
        check_orientation_index_line_segment(&seg, 10., 11., 1);
        check_orientation_index_line_segment(&seg, 10., 9., -1);

        check_orientation_index_line_segment(&seg, 11., 11., 0);

        check_orientation_index_line_segment(&seg, 11., 11.0000001, 1);
        check_orientation_index_line_segment(&seg, 11., 10.9999999, -1);

        check_orientation_index_line_segment(&seg, -2., -1.9999999, 1);
        check_orientation_index_line_segment(&seg, -2., -2.0000001, -1);
    }

    #[test]
    fn test_orientation_index_segment() {
        let seg = LineSegment::new_from_xy(100., 100., 110., 110.);

        check_orientation_index2(&seg, 100., 101., 105., 106., 1);
        check_orientation_index2(&seg, 100., 99., 105., 96., -1);
        check_orientation_index2(&seg, 200., 200., 210., 210., 0);
        check_orientation_index2(&seg, 105., 105., 110., 100., -1);
    }

    fn check_orientation_index_line_segment(
        seg: &LineSegment,
        px: f64,
        py: f64,
        expected_orient: i32,
    ) {
        let p = Coordinate::new_xy(px, py);
        let orient = seg.orientation_index_coordinate(&p);
        assert!(orient == expected_orient);
    }

    fn check_orientation_index2(
        seg: &LineSegment,
        s0x: f64,
        s0y: f64,
        s1x: f64,
        s1y: f64,
        expected_orient: i32,
    ) {
        let seg2 = LineSegment::new_from_xy(s0x, s0y, s1x, s1y);
        let orient = seg.orientation_index_line_segment(&seg2);
        assert_eq!(expected_orient, orient);
    }
}
