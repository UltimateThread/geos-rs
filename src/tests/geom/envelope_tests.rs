#[cfg(test)]
mod envelope_tests {
    use crate::core::geom::{coordinate::Coordinate, envelope::Envelope};

    #[test]
    fn test_everything() {
        let mut e1 = Envelope::default();
        assert!(e1.is_null());
        assert_eq!(0., e1.get_width());
        assert_eq!(0., e1.get_height());
        e1.expand_to_include_xy(100., 101.);
        e1.expand_to_include_xy(200., 202.);
        e1.expand_to_include_xy(150., 151.);
        assert_eq!(200., e1.get_max_x());
        assert_eq!(202., e1.get_max_y());
        assert_eq!(100., e1.get_min_x());
        assert_eq!(101., e1.get_min_y());
        assert!(e1.contains_xy(120., 120.));
        assert!(e1.contains_xy(120., 101.));
        assert!(!e1.contains_xy(120., 100.));
        assert_eq!(101., e1.get_height());
        assert_eq!(100., e1.get_width());
        assert!(!e1.is_null());

        let e2 = Envelope::new_xy(499., 500., 500., 501.);
        assert!(!e1.contains_envelope(&e2));
        assert!(!e1.intersects_envelope(&e2));
        e1.expand_to_include_envelope(&e2);
        assert!(e1.contains_envelope(&e2));
        assert!(e1.intersects_envelope(&e2));
        assert_eq!(500., e1.get_max_x());
        assert_eq!(501., e1.get_max_y());
        assert_eq!(100., e1.get_min_x());
        assert_eq!(101., e1.get_min_y());

        let e3 = Envelope::new_xy(300., 700., 300., 700.);
        assert!(!e1.contains_envelope(&e3));
        assert!(e1.intersects_envelope(&e3));

        let e4 = Envelope::new_xy(300., 301., 300., 301.);
        assert!(e1.contains_envelope(&e4));
        assert!(e1.intersects_envelope(&e4));
    }

    #[test]
    fn test_intersects() {
        check_intersects_permuted(1., 1., 2., 2., 2., 2., 3., 3., true);
        check_intersects_permuted(1., 1., 2., 2., 3., 3., 4., 4., false);
    }

    #[test]
    fn test_intersects_empty() {
        assert!(!Envelope::new_xy(-5., 5., -5., 5.).intersects_envelope(&Envelope::default()));
        assert!(!Envelope::default().intersects_envelope(&Envelope::new_xy(-5., 5., -5., 5.)));
        assert!(!Envelope::default().intersects_envelope(&Envelope::new_xy(100., 101., 100., 101.)));
        assert!(!Envelope::new_xy(100., 101., 100., 101.).intersects_envelope(&Envelope::default()));
    }

    #[test]
    fn test_disjoint_empty() {
        assert!(Envelope::new_xy(-5., 5., -5., 5.).disjoint_envelope(&Envelope::default()));
        assert!(Envelope::default().disjoint_envelope(&Envelope::new_xy(-5., 5., -5., 5.)));
        assert!(Envelope::default().disjoint_envelope(&Envelope::new_xy(100., 101., 100., 101.)));
        assert!(Envelope::new_xy(100., 101., 100., 101.).disjoint_envelope(&Envelope::default()));
    }

    #[test]
    fn test_contains_empty() {
        assert!(!Envelope::new_xy(-5., 5., -5., 5.).contains_envelope(&Envelope::default()));
        assert!(!Envelope::default().contains_envelope(&Envelope::new_xy(-5., 5., -5., 5.)));
        assert!(!Envelope::default().contains_envelope(&Envelope::new_xy(100., 101., 100., 101.)));
        assert!(!Envelope::new_xy(100., 101., 100., 101.).contains_envelope(&Envelope::default()));
    }

    #[test]
    fn test_expand_to_include_empty() {
        assert_envelope_equals(
            &Envelope::new_xy(-5., 5., -5., 5.),
            &expand_to_include(
                &mut Envelope::new_xy(-5., 5., -5., 5.),
                &Envelope::default(),
            ),
        );
        assert_envelope_equals(
            &Envelope::new_xy(-5., 5., -5., 5.),
            &expand_to_include(
                &mut Envelope::default(),
                &Envelope::new_xy(-5., 5., -5., 5.),
            ),
        );
        assert_envelope_equals(
            &Envelope::new_xy(100., 101., 100., 101.),
            &expand_to_include(
                &mut Envelope::default(),
                &Envelope::new_xy(100., 101., 100., 101.),
            ),
        );
        assert_envelope_equals(
            &Envelope::new_xy(100., 101., 100., 101.),
            &expand_to_include(
                &mut Envelope::new_xy(100., 101., 100., 101.),
                &Envelope::default(),
            ),
        );
    }

    fn expand_to_include(a: &mut Envelope, b: &Envelope) -> Envelope {
        a.expand_to_include_envelope(b);
        return a.clone();
    }

    fn assert_envelope_equals(a: &Envelope, b: &Envelope) {
        assert_eq!(a.get_min_x(), b.get_min_x());
        assert_eq!(a.get_max_x(), b.get_max_x());
        assert_eq!(a.get_min_y(), b.get_min_y());
        assert_eq!(a.get_max_y(), b.get_max_y());
        assert_eq!(a.get_width(), b.get_width());
        assert_eq!(a.get_height(), b.get_height());
        assert_eq!(a.get_area(), b.get_area());
        assert_eq!(a.get_diameter(), b.get_diameter());
    }

    #[test]
    fn test_empty() {
        assert_eq!(0., Envelope::default().get_height());
        assert_eq!(0., Envelope::default().get_width());
        assert_envelope_equals(&Envelope::default(), &Envelope::default());
        let mut e = Envelope::new_xy(100., 101., 100., 101.);
        e.init(&Envelope::default());
        assert_envelope_equals(&Envelope::default(), &e);
    }

    // #[test]
    //     fn test_as_geometry() {
    //         geometryFactory = new GeometryFactory(precisionModel,
    //             0);

    //         assertTrue(geometryFactory.createPoint((Coordinate) null).getEnvelope()
    //                 .isEmpty());

    //         Geometry g = geometryFactory.createPoint(Coordinate::new_xy(5., 6.)).getEnvelope();
    //         assertTrue(!g.isEmpty());
    //         assertTrue(g instanceof Point);

    //         Point p = (Point) g;
    //         assertEquals(5, p.getX(), 1E-1);
    //         assertEquals(6, p.getY(), 1E-1);

    //         LineString l = (LineString) reader.read("LINESTRING(10 10, 20 20, 30 40)");
    //         Geometry g2 = l.getEnvelope();
    //         assertTrue(!g2.isEmpty());
    //         assertTrue(g2 instanceof Polygon);

    //         Polygon poly = (Polygon) g2;
    //         poly.normalize();
    //         assertEquals(5, poly.getExteriorRing().getNumPoints());
    //         assertEquals(new Coordinate(10, 10), poly.getExteriorRing().getCoordinateN(
    //                 0));
    //         assertEquals(new Coordinate(10, 40), poly.getExteriorRing().getCoordinateN(
    //                 1));
    //         assertEquals(new Coordinate(30, 40), poly.getExteriorRing().getCoordinateN(
    //                 2));
    //         assertEquals(new Coordinate(30, 10), poly.getExteriorRing().getCoordinateN(
    //                 3));
    //         assertEquals(new Coordinate(10, 10), poly.getExteriorRing().getCoordinateN(
    //                 4));
    //     }

    #[test]
    fn test_set_to_null() {
        let mut e1 = Envelope::default();
        assert!(e1.is_null());
        e1.expand_to_include_xy(5., 5.);
        assert!(!e1.is_null());
        e1.set_to_null();
        assert!(e1.is_null());
    }

    #[test]
    fn test_equals() {
        let mut e1 = Envelope::new_xy(1., 2., 3., 4.);
        let mut e2 = Envelope::new_xy(1., 2., 3., 4.);
        assert_envelope_equals(&e1, &e2);
        // assertEquals(e1.hashCode(), e2.hashCode());

        let e3 = Envelope::new_xy(1., 2., 3., 5.);
        assert!(!e1.equals(&e3));
        // assert!(e1.hashCode() != e3.hashCode());
        e1.set_to_null();
        assert!(!e1.equals(&e2));
        // assert!(e1.hashCode() != e2.hashCode());
        e2.set_to_null();
        assert_envelope_equals(&e1, &e2);
        // assertEquals(e1.hashCode(), e2.hashCode());
    }

    #[test]
    fn test_equals2() {
        assert!(Envelope::default().equals(&Envelope::default()));
        assert!(Envelope::new_xy(1., 2., 1., 2.).equals(&Envelope::new_xy(1., 2., 1., 2.)));
        assert!(!Envelope::new_xy(1., 2., 1.5, 2.).equals(&Envelope::new_xy(1., 2., 1., 2.)));
    }

    #[test]
    fn test_copy_constructor() {
        let e1 = Envelope::new_xy(1., 2., 3., 4.);
        let e2 = Envelope::new_envelope(&e1);
        assert_eq!(1., e2.get_min_x());
        assert_eq!(2., e2.get_max_x());
        assert_eq!(3., e2.get_min_y());
        assert_eq!(4., e2.get_max_y());
    }

    #[test]
    fn test_copy() {
        let e1 = Envelope::new_xy(1., 2., 3., 4.);
        let e2 = e1.clone();
        assert_eq!(1., e2.get_min_x());
        assert_eq!(2., e2.get_max_x());
        assert_eq!(3., e2.get_min_y());
        assert_eq!(4., e2.get_max_y());

        let e_null = Envelope::default();
        let e_null_copy = e_null.clone();
        assert!(e_null_copy.is_null());
    }

    // #[test]
    // fn test_geometry_factory_create_envelope() {
    //     checkExpectedEnvelopeGeometry("POINT (0 0)");
    //     checkExpectedEnvelopeGeometry("POINT (100 13)");
    //     checkExpectedEnvelopeGeometry("LINESTRING (0 0, 0 10)");
    //     checkExpectedEnvelopeGeometry("LINESTRING (0 0, 10 0)");

    //     let poly10 = "POLYGON ((0 10, 10 10, 10 0, 0 0, 0 10))";
    //     checkExpectedEnvelopeGeometry(poly10);

    //     checkExpectedEnvelopeGeometry("LINESTRING (0 0, 10 10)", poly10);
    //     checkExpectedEnvelopeGeometry("POLYGON ((5 10, 10 6, 5 0, 0 6, 5 10))", poly10);
    // }

    #[test]
    fn test_metrics() {
        let env = Envelope::new_xy(0., 4., 0., 3.);
        assert_eq!(env.get_width(), 4.0);
        assert_eq!(env.get_height(), 3.0);
        assert_eq!(env.get_diameter(), 5.0);
    }

    #[test]
    fn test_empty_metrics() {
        let env = Envelope::default();
        assert_eq!(env.get_width(), 0.0);
        assert_eq!(env.get_height(), 0.0);
        assert_eq!(env.get_diameter(), 0.0);
    }

    fn check_intersects_permuted(
        a1x: f64,
        a1y: f64,
        a2x: f64,
        a2y: f64,
        b1x: f64,
        b1y: f64,
        b2x: f64,
        b2y: f64,
        expected: bool,
    ) {
        check_intersects(a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y, expected);
        check_intersects(a1x, a2y, a2x, a1y, b1x, b1y, b2x, b2y, expected);
        check_intersects(a1x, a1y, a2x, a2y, b1x, b2y, b2x, b1y, expected);
        check_intersects(a1x, a2y, a2x, a1y, b1x, b2y, b2x, b1y, expected);
    }

    fn check_intersects(
        a1x: f64,
        a1y: f64,
        a2x: f64,
        a2y: f64,
        b1x: f64,
        b1y: f64,
        b2x: f64,
        b2y: f64,
        expected: bool,
    ) {
        let a = Envelope::new_xy(a1x, a2x, a1y, a2y);
        let b = Envelope::new_xy(b1x, b2x, b1y, b2y);
        assert_eq!(expected, a.intersects_envelope(&b));
        assert_eq!(expected, !a.disjoint_envelope(&b));

        let a1 = Coordinate::new_xy(a1x, a1y);
        let a2 = Coordinate::new_xy(a2x, a2y);
        let b1 = Coordinate::new_xy(b1x, b1y);
        let b2 = Coordinate::new_xy(b2x, b2y);
        assert_eq!(expected, Envelope::intersects_4(&a1, &a2, &b1, &b2));

        assert_eq!(expected, a.intersects_coordinate_ab(&b1, &b2));
    }

    // fn checkExpectedEnvelopeGeometry(wktInput: &str) {
    //     checkExpectedEnvelopeGeometry(wktInput, wktInput);
    // }

    // fn checkExpectedEnvelopeGeometry(wktInput: &str, wktEnvGeomExpected: &str) {
    //     Geometry input = reader.read(wktInput);
    //     Geometry envGeomExpected = reader.read(wktEnvGeomExpected);

    //     Envelope env = input.getEnvelopeInternal();
    //     Geometry envGeomActual = geometryFactory.toGeometry(env);
    //     boolean isEqual = envGeomActual.equalsNorm(envGeomExpected);
    //     assertTrue(isEqual);
    // }

    #[test]
    fn test_compare_to() {
        check_compare_to(0, &Envelope::default(), &Envelope::default());
        check_compare_to(
            0,
            &Envelope::new_xy(1., 2., 1., 2.),
            &Envelope::new_xy(1., 2., 1., 2.),
        );
        check_compare_to(
            1,
            &Envelope::new_xy(2., 3., 1., 2.),
            &Envelope::new_xy(1., 2., 1., 2.),
        );
        check_compare_to(
            -1,
            &Envelope::new_xy(1., 2., 1., 2.),
            &Envelope::new_xy(2., 3., 1., 2.),
        );
        check_compare_to(
            1,
            &Envelope::new_xy(1., 2., 1., 3.),
            &Envelope::new_xy(1., 2., 1., 2.),
        );
        check_compare_to(
            1,
            &Envelope::new_xy(2., 3., 1., 3.),
            &Envelope::new_xy(1., 3., 1., 2.),
        );
    }

    fn check_compare_to(expected: i32, env1: &Envelope, env2: &Envelope) {
        assert!(expected == env1.compare_to_envelope(env2));
        assert!(-expected == env2.compare_to_envelope(env1));
    }
}
