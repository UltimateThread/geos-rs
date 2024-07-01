#[cfg(test)]
mod point_location_tests {
    use crate::core::{
        algorithm::point_location::PointLocation,
        geom::{
            coordinate::Coordinate, geometry_factory::GeometryFactory, line_string::LineString,
        },
    };

    #[test]
    fn test_on_line_on_vertex() {
        let coords: Vec<Coordinate> = vec![
            Coordinate::new_xy(0., 0.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(30., 30.),
        ];
        let line = GeometryFactory::create_line_string_coordinates(&coords);
        check_on_line(20., 20., &line, true);
    }

    #[test]
    fn test_on_line_in_segment() {
        let coords: Vec<Coordinate> = vec![
            Coordinate::new_xy(0., 0.),
            Coordinate::new_xy(20., 20.),
            Coordinate::new_xy(0., 40.),
        ];
        let line = GeometryFactory::create_line_string_coordinates(&coords);
        check_on_line(10., 10., &line, true);
        check_on_line(10., 30., &line, true);
    }

    #[test]
    fn test_not_on_line() {
        let coords: Vec<Coordinate> = vec![
            Coordinate::new_xy(10., 10.),
            Coordinate::new_xy(20., 10.),
            Coordinate::new_xy(30., 10.),
        ];
        let line = GeometryFactory::create_line_string_coordinates(&coords);
        check_on_line(0., 100., &line, false);
    }

    #[test]
    fn test_on_segment() {
        let coords: Vec<Coordinate> = vec![Coordinate::new_xy(0., 0.), Coordinate::new_xy(9., 9.)];
        let line = GeometryFactory::create_line_string_coordinates(&coords);
        check_on_segment(5., 5., &line, true);
        check_on_segment(0., 0., &line, true);
        check_on_segment(9., 9., &line, true);
    }

    #[test]
    fn test_not_on_segment() {
        let coords: Vec<Coordinate> = vec![Coordinate::new_xy(0., 0.), Coordinate::new_xy(9., 9.)];
        let line = GeometryFactory::create_line_string_coordinates(&coords);
        check_on_segment(5., 6., &line, false);
        check_on_segment(10., 10., &line, false);
        check_on_segment(9., 9.00001, &line, false);
    }

    #[test]
    fn test_on_zero_length_segment() {
        let coords: Vec<Coordinate> = vec![Coordinate::new_xy(1., 1.), Coordinate::new_xy(1., 1.)];
        let line = GeometryFactory::create_line_string_coordinates(&coords);
        check_on_segment(1., 1., &line, true);
        check_on_segment(1., 2., &line, false);
    }

    fn check_on_segment(x: f64, y: f64, line: &LineString, expected: bool) {
        let p0 = line.get_coordinate_n(0);
        let p1 = line.get_coordinate_n(1);
        assert!(expected == PointLocation::is_on_segment(&Coordinate::new_xy(x, y), &p0, &p1));
    }

    #[test]
    fn test_on_vertex4d() {
        let coords: Vec<Coordinate> = vec![
            Coordinate::new_xyzm(0., 0., 0., 0.),
            Coordinate::new_xyzm(20., 20., 20., 20.),
            Coordinate::new_xyzm(30., 30., 30., 30.),
        ];
        let line = GeometryFactory::create_line_string_coordinates(&coords);
        check_on_line(20., 20., &line, true);
    }

    #[test]
    fn test_on_segment4d() {
        let coords: Vec<Coordinate> = vec![
            Coordinate::new_xyzm(0., 0., 0., 0.),
            Coordinate::new_xyzm(20., 20., 20., 20.),
            Coordinate::new_xyzm(0., 40., 40., 40.),
        ];
        let line = GeometryFactory::create_line_string_coordinates(&coords);
        check_on_line(10., 10., &line, true);
        check_on_line(10., 30., &line, true);
    }

    #[test]
    fn test_not_on_line4d() {
        let coords: Vec<Coordinate> = vec![
            Coordinate::new_xyzm(10., 10., 10., 10.),
            Coordinate::new_xyzm(20., 10., 10., 10.),
            Coordinate::new_xyzm(30., 10., 10., 10.),
        ];
        let line = GeometryFactory::create_line_string_coordinates(&coords);
        check_on_line(0., 100., &line, false);
    }

    fn check_on_line(x: f64, y: f64, line: &LineString, expected: bool) {
        assert!(
            expected
                == PointLocation::is_on_line_coordinates(
                    &Coordinate::new_xy(x, y),
                    &line.get_coordinates()
                )
        );

        assert!(
            expected
                == PointLocation::is_on_line_coordinate_array_sequence(
                    &Coordinate::new_xy(x, y),
                    &line.get_coordinate_sequence()
                )
        );
    }
}
