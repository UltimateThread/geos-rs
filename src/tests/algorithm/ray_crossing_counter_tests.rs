#[cfg(test)]
mod ray_crossing_counter_tests {
    use crate::core::{
        algorithm::ray_crossing_counter::RayCrossingCounter,
        geom::{
            coordinate::Coordinate,
            implementation::packed_coordinate_sequence_factory::PackedCoordinateSequenceFactory,
            location::Location,
        },
    };

    #[test]
    fn test_run_pt_in_ring4d_packed() {
        let coords: Vec<f64> = vec![
            0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 5.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let cs = PackedCoordinateSequenceFactory::create_double_coordinate_dimension_measures(
            &coords, 4, 1,
        );
        assert_eq!(
            Location::INTERIOR,
            RayCrossingCounter::locate_point_in_ring_packed_coordinate_sequence(
                &Coordinate::new_xy(5.0, 2.0),
                &cs
            )
        );
    }
}
