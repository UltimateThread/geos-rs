#[cfg(test)]
mod triangle_cercumcentre_tests {
    use crate::core::geom::{coordinate::Coordinate, triangle::Triangle};

    #[test]
    fn test_square_diagonal_dd() {
        let cc1 = circumcentre_dd(
            193600.80333333334,
            469345.355,
            193600.80333333334,
            469345.0175,
            193601.10666666666,
            469345.0175,
        );
        let cc2 = circumcentre_dd(
            193600.80333333334,
            469345.355,
            193601.10666666666,
            469345.0175,
            193601.10666666666,
            469345.355,
        );
        check_cc_equal(&cc1, &cc2);
    }

    fn circumcentre(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> Coordinate {
        let a = Coordinate::new_xy(ax, ay);
        let b = Coordinate::new_xy(bx, by);
        let c = Coordinate::new_xy(cx, cy);
        return Triangle::circumcentre_coordinates(&a, &b, &c);
    }

    fn circumcentre_dd(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> Coordinate {
        let a = Coordinate::new_xy(ax, ay);
        let b = Coordinate::new_xy(bx, by);
        let c = Coordinate::new_xy(cx, cy);
        return Triangle::circumcentre_dd_coordinates(&a, &b, &c);
    }

    fn check_cc_equal(cc1: &Coordinate, cc2: &Coordinate) {
        let is_equal = cc1.equals_2d(cc2);
        assert!(is_equal);
    }
}
