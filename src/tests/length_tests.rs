#[cfg(test)]
mod length_tests {
    use crate::core::{
        algorithm::length::Length,
        geom::{
            coordinate::Coordinate, geometry_factory::GeometryFactory, line_string::LineString,
        },
    };

    #[test]
    fn test_area() {
        let coords: Vec<Coordinate> = vec![
            Coordinate::new_xy(100., 200.),
            Coordinate::new_xy(200., 200.),
            Coordinate::new_xy(200., 100.),
            Coordinate::new_xy(100., 100.),
            Coordinate::new_xy(100., 200.),
        ];
        let line = GeometryFactory::create_line_string_coordinates(&coords);
        check_length_of_line(&line, 400.0);
    }

    fn check_length_of_line(ring: &LineString, expected_len: f64) {
        let pts = ring.get_coordinate_sequence();
        let actual = Length::of_line(&pts);
        assert_eq!(actual, expected_len);
    }
}
