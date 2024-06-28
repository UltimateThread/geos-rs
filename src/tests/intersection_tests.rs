#[cfg(test)]
mod intersection_tests {
    use crate::core::{algorithm::intersection::Intersection, geom::coordinate::Coordinate};

    const MAX_ABS_ERROR: f64 = 1e-5;

    #[test]
    fn test_simple() {
        check_intersection(0., 0., 10., 10., 0., 10., 10., 0., 5., 5.);
    }

    #[test]
    fn test_collinear() {
        check_intersection_null(0., 0., 10., 10., 20., 20., 30., 30.);
    }

    #[test]
    fn test_parallel() {
        check_intersection_null(0., 0., 10., 10., 10., 0., 20., 10.);
    }

    #[test]
    // See JTS GitHub issue #464
    fn test_almost_collinear() {
        check_intersection(
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

    #[test]
    // same as above but conditioned manually
    fn test_almost_collinear_cond() {
        check_intersection(
            1.6165017,
            45.306132293,
            7.7705378,
            60.528222711,
            7.77505724,
            60.539653536,
            9.85607389,
            65.92369170,
            7.772841461,
            60.5339209242,
        );
    }

    //------------------------------------------------------------
    #[test]
    fn test_line_seg_cross() {
        check_intersection_line_segment(0., 0., 0., 1., -1., 9., 1., 9., 0., 9.);
        check_intersection_line_segment(0., 0., 0., 1., -1., 2., 1., 4., 0., 3.);
    }

    fn test_line_seg_touch() {
        check_intersection_line_segment(0., 0., 0., 1., -1., 9., 0., 9., 0., 9.);
        check_intersection_line_segment(0., 0., 0., 1., 0., 2., 1., 4., 0., 2.);
    }

    fn test_line_seg_collinear() {
        check_intersection_line_segment(0., 0., 0., 1., 0., 9., 0., 8., 0., 9.);
    }

    fn test_line_seg_none() {
        check_intersection_line_segment_null(0., 0., 0., 1., 2., 9., 1., 9.);
        check_intersection_line_segment_null(0., 0., 0., 1., -2., 9., -1., 9.);
        check_intersection_line_segment_null(0., 0., 0., 1., 2., 9., 1., 9.);
    }

    //==================================================

    fn check_intersection(
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
        let p1 = Coordinate::new_xy(p1x, p1y);
        let p2 = Coordinate::new_xy(p2x, p2y);
        let q1 = Coordinate::new_xy(q1x, q1y);
        let q2 = Coordinate::new_xy(q2x, q2y);
        let actual = Intersection::intersection(&p1, &p2, &q1, &q2);
        let expected = Coordinate::new_xy(expectedx, expectedy);
        let dist = actual.unwrap().distance(&expected);
        assert!(dist <= MAX_ABS_ERROR);
    }

    fn check_intersection_null(
        p1x: f64,
        p1y: f64,
        p2x: f64,
        p2y: f64,
        q1x: f64,
        q1y: f64,
        q2x: f64,
        q2y: f64,
    ) {
        let p1 = Coordinate::new_xy(p1x, p1y);
        let p2 = Coordinate::new_xy(p2x, p2y);
        let q1 = Coordinate::new_xy(q1x, q1y);
        let q2 = Coordinate::new_xy(q2x, q2y);
        let actual = Intersection::intersection(&p1, &p2, &q1, &q2);
        assert!(actual.is_none());
    }

    fn check_intersection_line_segment(
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
        let p1 = Coordinate::new_xy(p1x, p1y);
        let p2 = Coordinate::new_xy(p2x, p2y);
        let q1 = Coordinate::new_xy(q1x, q1y);
        let q2 = Coordinate::new_xy(q2x, q2y);
        //Coordinate actual = CGAlgorithmsDD.intersection(p1, p2, q1, q2);
        let actual = Intersection::line_segment(&p1, &p2, &q1, &q2);
        let expected = Coordinate::new_xy(expectedx, expectedy);
        let dist = actual.unwrap().distance(&expected);
        //System.out.println("Expected: " + expected + "  Actual: " + actual + "  Dist = " + dist);
        assert!(dist <= MAX_ABS_ERROR);
    }

    fn check_intersection_line_segment_null(
        p1x: f64,
        p1y: f64,
        p2x: f64,
        p2y: f64,
        q1x: f64,
        q1y: f64,
        q2x: f64,
        q2y: f64,
    ) {
        let p1 = Coordinate::new_xy(p1x, p1y);
        let p2 = Coordinate::new_xy(p2x, p2y);
        let q1 = Coordinate::new_xy(q1x, q1y);
        let q2 = Coordinate::new_xy(q2x, q2y);
        let actual = Intersection::line_segment(&p1, &p2, &q1, &q2);
        assert!(actual.is_none());
    }
}
