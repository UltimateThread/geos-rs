#[cfg(test)]
mod area_tests {
    use crate::core::{
        algorithm::area::Area,
        geom::{
            coordinate::Coordinate, geometry_factory::GeometryFactory, linear_ring::LinearRing,
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
        let ring = GeometryFactory::create_linear_ring_with_coordinates(&coords);
        check_area_of_ring(&ring, 10000.0);
    }

    #[test]
    fn test_area_signed_cw() {
        let coords: Vec<Coordinate> = vec![
            Coordinate::new_xy(100., 200.),
            Coordinate::new_xy(200., 200.),
            Coordinate::new_xy(200., 100.),
            Coordinate::new_xy(100., 100.),
            Coordinate::new_xy(100., 200.),
        ];
        let ring = GeometryFactory::create_linear_ring_with_coordinates(&coords);
        check_area_of_ring_signed(&ring, 10000.0);
    }

    #[test]
    fn test_areasigned_ccw() {
        let coords: Vec<Coordinate> = vec![
            Coordinate::new_xy(100., 200.),
            Coordinate::new_xy(100., 100.),
            Coordinate::new_xy(200., 100.),
            Coordinate::new_xy(200., 200.),
            Coordinate::new_xy(100., 200.),
        ];
        let ring = GeometryFactory::create_linear_ring_with_coordinates(&coords);
        check_area_of_ring_signed(&ring, -10000.0);
    }

    fn check_area_of_ring(ring: &LinearRing, expected_area: f64) {
        let ring_pts = ring.get_coordinates();
        let actual1 = Area::of_ring_vec(&ring_pts);
        assert_eq!(actual1, expected_area);

        let ring_seq = ring.get_coordinate_array_sequence();
        let actual2 = Area::of_ring_coordinate_sequence(&ring_seq);
        assert_eq!(actual2, expected_area);
    }

    fn check_area_of_ring_signed(ring: &LinearRing, expected_area: f64) {
        let ring_pts = ring.get_coordinates();
        let actual1 = Area::of_ring_signed_vec(&ring_pts);
        assert_eq!(actual1, expected_area);

        let ring_seq = ring.get_coordinate_array_sequence();
        let actual2 = Area::of_ring_signed_coordinate_sequence(&ring_seq);
        assert_eq!(actual2, expected_area);
    }
}
