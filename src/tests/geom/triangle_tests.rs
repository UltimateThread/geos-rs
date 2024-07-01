#[cfg(test)]
mod triangle_tests {
    use crate::core::geom::{
        coordinate::Coordinate, geometry_factory::GeometryFactory, line_string::LineString,
        point::Point, polygon::Polygon, triangle::Triangle,
    };

    const TOLERANCE: f64 = 1E-5;

    #[test]
    fn test_interpolate_z() {
        let coords: Vec<Coordinate> = vec![
            Coordinate::new_xyz(1., 1., 0.),
            Coordinate::new_xyz(2., 1., 0.),
            Coordinate::new_xyz(1., 2., 10.),
        ];
        let ls1 = GeometryFactory::create_line_string_coordinates(&coords);
        check_interpolate_z(&ls1, &Coordinate::new_xy(1.5, 1.5), 5.);
        check_interpolate_z(&ls1, &Coordinate::new_xy(1.2, 1.2), 2.);
        check_interpolate_z(&ls1, &Coordinate::new_xy(0., 0.), -10.);
    }

    fn check_interpolate_z(line_string: &LineString, p: &Coordinate, expected_value: f64) {
        let pt = line_string.get_coordinates();

        let t = Triangle::new(&pt[0], &pt[1], &pt[2]);
        let z = t.interpolate_z(p);
        assert_eq_with_tolerance(expected_value, z, 0.000001);
    }

    fn assert_eq_with_tolerance(a: f64, b: f64, tolerance: f64) {
        assert!(a >= (b - tolerance) && a <= (b + tolerance))
    }

    #[test]
    fn test_area_3d() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xyz(0., 0., 10.),
            Coordinate::new_xyz(100., 0., 110.),
            Coordinate::new_xyz(100., 100., 110.),
            Coordinate::new_xyz(0., 0., 10.),
        ];
        let p1 = GeometryFactory::create_polygon_with_coordinates(&coords1);
        check_area_3d(&p1, 7071.067811865475);

        let coords2: Vec<Coordinate> = vec![
            Coordinate::new_xyz(0., 0., 10.),
            Coordinate::new_xyz(100., 0., 10.),
            Coordinate::new_xyz(50., 100., 110.),
            Coordinate::new_xyz(0., 0., 10.),
        ];
        let p2 = GeometryFactory::create_polygon_with_coordinates(&coords2);
        check_area_3d(&p2, 7071.067811865475);
    }

    fn check_area_3d(polygon: &Polygon, expected_value: f64) {
        let pt = polygon.get_coordinates();
        let t = Triangle::new(&pt[0], &pt[1], &pt[2]);
        let area_3d = t.area_3d();
        // System.out.println("area3D = " + area3D);
        assert_eq_with_tolerance(expected_value, area_3d, TOLERANCE);
    }

    #[test]
    fn test_area() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon1 = GeometryFactory::create_polygon_with_coordinates(&coords1);
        // CW
        check_area(&polygon1, 50.);

        let coords2: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon2 = GeometryFactory::create_polygon_with_coordinates(&coords2);
        // CCW
        check_area(&polygon2, -50.);

        let coords3: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon3 = GeometryFactory::create_polygon_with_coordinates(&coords3);
        // degenerate point triangle
        check_area(&polygon3, 0.);

        let coords4: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(15., 10.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon4 = GeometryFactory::create_polygon_with_coordinates(&coords4);
        // degenerate line triangle
        check_area(&polygon4, 0.);
    }

    fn check_area(polygon: &Polygon, expected_value: f64) {
        let pt = polygon.get_coordinates();

        let t = Triangle::new(&pt[0], &pt[1], &pt[2]);
        let signed_area = t.signed_area();
        //System.out.println("signed area = " + signedArea);
        assert_eq_with_tolerance(expected_value, signed_area, TOLERANCE);

        let area = t.area();
        assert_eq_with_tolerance(f64::abs(expected_value), area, TOLERANCE);
    }

    #[test]
    fn test_acute() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon1 = GeometryFactory::create_polygon_with_coordinates(&coords1);
        // right triangle
        check_acute(&polygon1, false);

        let coords2: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon2 = GeometryFactory::create_polygon_with_coordinates(&coords2);
        // CCW right tri
        check_acute(&polygon2, false);

        let coords3: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(15., 20.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon3 = GeometryFactory::create_polygon_with_coordinates(&coords3);
        // acute
        check_acute(&polygon3, true);
    }

    fn check_acute(polygon: &Polygon, expected_value: bool) {
        let pt = polygon.get_coordinates();

        let t = Triangle::new(&pt[0], &pt[1], &pt[2]);
        let is_acute = t.is_acute();
        //System.out.println("isAcute = " + isAcute);
        assert_eq!(expected_value, is_acute);
    }

    #[test]
    fn test_circum_centre() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon1 = GeometryFactory::create_polygon_with_coordinates(&coords1);
        // right triangle
        check_circum_centre(&polygon1, &Coordinate::new_xy(15.0, 15.0));

        let coords2: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon2 = GeometryFactory::create_polygon_with_coordinates(&coords2);
        // CCW right tri
        check_circum_centre(&polygon2, &Coordinate::new_xy(15.0, 15.0));

        let coords3: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(15., 20.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon3 = GeometryFactory::create_polygon_with_coordinates(&coords3);
        // acute
        check_circum_centre(&polygon3, &Coordinate::new_xy(15.0, 13.75));
    }

    #[test]
    fn test_circumradius() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon1 = GeometryFactory::create_polygon_with_coordinates(&coords1);
        // right triangle
        check_circumradius(&polygon1);

        let coords2: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon2 = GeometryFactory::create_polygon_with_coordinates(&coords2);
        // CCW right tri
        check_circumradius(&polygon2);

        let coords3: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(15., 20.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon3 = GeometryFactory::create_polygon_with_coordinates(&coords3);
        // acute
        check_circumradius(&polygon3);
    }

    #[test]
    fn test_centroid() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon1 = GeometryFactory::create_polygon_with_coordinates(&coords1);
        // right triangle
        check_centroid(
            &polygon1,
            &Coordinate::new_xy((10.0 + 20.0 + 20.0) / 3.0, (10.0 + 20.0 + 10.0) / 3.0),
        );

        let coords2: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon2 = GeometryFactory::create_polygon_with_coordinates(&coords2);
        // CCW right tri
        check_centroid(
            &polygon2,
            &Coordinate::new_xy((10.0 + 20.0 + 20.0) / 3.0, (10.0 + 10.0 + 20.0) / 3.0),
        );

        let coords3: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(15., 20.),
            Coordinate::new_xy(10., 10.),
        ];
        let polygon3 = GeometryFactory::create_polygon_with_coordinates(&coords3);
        // acute
        check_centroid(
            &polygon3,
            &Coordinate::new_xy((10.0 + 20.0 + 15.0) / 3.0, (10.0 + 10.0 + 20.0) / 3.0),
        );
    }

    fn check_centroid(polygon: &Polygon, expected_value: &Coordinate) {
        let pt = polygon.get_coordinates();

        let mut centroid = Triangle::centroid_coordinates(&pt[0], &pt[1], &pt[2]);
        //System.out.println("(Static) centroid = " + centroid);
        assert_eq!(expected_value.to_string(), centroid.to_string());

        // Test Instance version
        //
        let t = Triangle::new(&pt[0], &pt[1], &pt[2]);
        centroid = t.centroid();
        //System.out.println("(Instance) centroid = " + centroid.toString());
        assert_eq!(expected_value.to_string(), centroid.to_string());
    }

    fn check_circum_centre(polygon: &Polygon, expected_value: &Coordinate) {
        let pt = polygon.get_coordinates();

        let mut circumcentre = Triangle::circumcentre_coordinates(&pt[0], &pt[1], &pt[2]);
        //System.out.println("(Static) circumcentre = " + circumcentre);
        assert_eq!(expected_value.to_string(), circumcentre.to_string());

        // Test Instance version
        //
        let t = Triangle::new(&pt[0], &pt[1], &pt[2]);
        circumcentre = t.circumcentre();
        //System.out.println("(Instance) circumcentre = " + circumcentre.toString());
        assert_eq!(expected_value.to_string(), circumcentre.to_string());
    }

    fn check_circumradius(polygon: &Polygon) {
        let pt = polygon.get_coordinates();

        let circumcentre = Triangle::circumcentre_coordinates(&pt[0], &pt[1], &pt[2]);
        let circumradius = Triangle::circumradius_coordinates(&pt[0], &pt[1], &pt[2]);
        //System.out.println("(Static) circumcentre = " + circumcentre);
        let rad0 = pt[0].distance(&circumcentre);
        let rad1 = pt[1].distance(&circumcentre);
        let rad2 = pt[2].distance(&circumcentre);
        assert_eq_with_tolerance(rad0, circumradius, 0.00001);
        assert_eq_with_tolerance(rad1, circumradius, 0.00001);
        assert_eq_with_tolerance(rad2, circumradius, 0.00001);
    }

    #[test]
    fn test_longest_side_length() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xyz(10., 10., 1.),
            Coordinate::new_xyz(20., 20., 2.),
            Coordinate::new_xyz(20., 10., 3.),
            Coordinate::new_xyz(10., 10., 1.),
        ];
        let polygon1 = GeometryFactory::create_polygon_with_coordinates(&coords1);
        // right triangle
        check_longest_side_length(&polygon1, 14.142135623730951);

        let coords2: Vec<Coordinate> = vec![
            Coordinate::new_xyz(10., 10., 1.),
            Coordinate::new_xyz(20., 10., 2.),
            Coordinate::new_xyz(20., 20., 3.),
            Coordinate::new_xyz(10., 10., 1.),
        ];
        let polygon2 = GeometryFactory::create_polygon_with_coordinates(&coords2);
        // CCW right tri
        check_longest_side_length(&polygon2, 14.142135623730951);

        let coords3: Vec<Coordinate> = vec![
            Coordinate::new_xyz(10., 10., 1.),
            Coordinate::new_xyz(20., 10., 2.),
            Coordinate::new_xyz(15., 20., 3.),
            Coordinate::new_xyz(10., 10., 1.),
        ];
        let polygon3 = GeometryFactory::create_polygon_with_coordinates(&coords3);
        // acute
        check_longest_side_length(&polygon3, 11.180339887498949);
    }

    fn check_longest_side_length(polygon: &Polygon, expected_value: f64) {
        let pt = polygon.get_coordinates();

        let mut length = Triangle::longest_side_length_coordinates(&pt[0], &pt[1], &pt[2]);
        //System.out.println("(Static) longestSideLength = " + length);
        assert_eq_with_tolerance(expected_value, length, 0.00000001);

        // Test Instance version
        //
        let t = Triangle::new(&pt[0], &pt[1], &pt[2]);
        length = t.longest_side_length();
        //System.out.println("(Instance) longestSideLength = " + length);
        assert_eq_with_tolerance(expected_value, length, 0.00000001);
    }

    //===============================================================

    #[test]
    fn test_is_ccw() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xy(30., 90.),
            Coordinate::new_xy(80., 50.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(30., 90.),
        ];
        let polygon1 = GeometryFactory::create_polygon_with_coordinates(&coords1);
        check_is_ccw(&polygon1, false);

        let coords2: Vec<Coordinate> = vec![
            Coordinate::new_xy(90., 90.),
            Coordinate::new_xy(20., 40.),
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(90., 90.),
        ];
        let polygon2 = GeometryFactory::create_polygon_with_coordinates(&coords2);
        check_is_ccw(&polygon2, true);
    }

    fn check_is_ccw(polygon: &Polygon, expected_value: bool) {
        let pt = polygon.get_coordinates();
        let actual = Triangle::is_ccw_coordinates(&pt[0], &pt[1], &pt[2]);
        assert_eq!(expected_value, actual);
    }

    //===============================================================

    #[test]
    fn test_intersects() {
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xy(30., 90.),
            Coordinate::new_xy(80., 50.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(30., 90.),
        ];
        let polygon1 = GeometryFactory::create_polygon_with_coordinates(&coords1);
        let point = GeometryFactory::create_point_from_coordinate(&Coordinate::new_xy(70., 20.));
        check_intersects(&polygon1, &point, false);

        let coords2: Vec<Coordinate> = vec![
            Coordinate::new_xy(30., 90.),
            Coordinate::new_xy(80., 50.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(30., 90.),
        ];
        let polygon2 = GeometryFactory::create_polygon_with_coordinates(&coords2);
        let point2 = GeometryFactory::create_point_from_coordinate(&Coordinate::new_xy(30., 90.));
        // triangle vertex
        check_intersects(&polygon2, &point2, true);

        let coords3: Vec<Coordinate> = vec![
            Coordinate::new_xy(30., 90.),
            Coordinate::new_xy(80., 50.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(30., 90.),
        ];
        let polygon3 = GeometryFactory::create_polygon_with_coordinates(&coords3);
        let point3 = GeometryFactory::create_point_from_coordinate(&Coordinate::new_xy(40., 40.));
        check_intersects(&polygon3, &point3, true);

        let coords4: Vec<Coordinate> = vec![
            Coordinate::new_xy(30., 90.),
            Coordinate::new_xy(70., 50.),
            Coordinate::new_xy(71.5, 16.5),
            Coordinate::new_xy(30., 90.),
        ];
        let polygon4 = GeometryFactory::create_polygon_with_coordinates(&coords4);
        let point4 = GeometryFactory::create_point_from_coordinate(&Coordinate::new_xy(50., 70.));
        // on an edge
        check_intersects(&polygon4, &point4, true);
    }

    fn check_intersects(polygon: &Polygon, point: &Point, expected_value: bool) {
        let tri = polygon.get_coordinates();
        let pt = point.get_coordinate();

        let actual = Triangle::intersects_coordinates(&tri[0], &tri[1], &tri[2], &pt.unwrap());
        assert_eq!(expected_value, actual);
    }
}
