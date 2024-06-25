use crate::geom::{coordinate::Coordinate, coordinates::Coordinates};

use super::{packed_coordinate_sequence::PackedCoordinateSequence, packed_coordinate_sequence_double::PackedCoordinateSequenceDouble};



/**
 * Builds packed array coordinate sequences.
 * The array data type can be either
 * <code>double</code> or <code>float</code>,
 * and defaults to <code>double</code>.
 */
pub struct PackedCoordinateSequenceFactory {}

impl PackedCoordinateSequenceFactory {
    const DEFAULT_MEASURES: i32 = 0;

    const DEFAULT_DIMENSION: i32 = 3;

    /**
     * @see CoordinateSequenceFactory#create(Coordinate[])
     */
    pub fn create_double_from_coordinates(
        coordinates: &Vec<Coordinate>,
    ) -> PackedCoordinateSequenceDouble {
        let mut dimension = PackedCoordinateSequenceFactory::DEFAULT_DIMENSION;
        let mut measures = PackedCoordinateSequenceFactory::DEFAULT_MEASURES;
        if coordinates.len() > 0 {
            let first = coordinates[0];
            dimension = Coordinates::dimension(&first);
            measures = Coordinates::measures(&first);
        }
        PackedCoordinateSequenceDouble::new_coordinates_with_dimension_measures(
            coordinates,
            dimension,
            measures,
        )
    }

    /**
     * @see CoordinateSequenceFactory#create(CoordinateSequence)
     */
    pub fn create_double_from_coordinate_sequence(
        coord_seq: &PackedCoordinateSequence,
    ) -> PackedCoordinateSequenceDouble {
        let dimension = coord_seq.get_dimension();
        let measures = coord_seq.get_measures();
        PackedCoordinateSequenceDouble::new_coordinates_with_dimension_measures(
            &coord_seq.to_coordinate_array(),
            dimension,
            measures,
        )
    }

    /**
     * Creates a packed coordinate sequence of type {@link #DOUBLE}
     * from the provided array
     * using the given coordinate dimension and a measure count of 0.
     *
     * @param packedCoordinates the array containing coordinate values
     * @param dimension the coordinate dimension
     * @return a packed coordinate sequence of type {@link #DOUBLE}
     */
    pub fn create_double_coordinates_dimension(
        packed_coordinates: &Vec<f64>,
        dimension: i32,
    ) -> PackedCoordinateSequenceDouble {
        PackedCoordinateSequenceFactory::create_double_coordinate_dimension_measures(
            packed_coordinates,
            dimension,
            PackedCoordinateSequenceFactory::DEFAULT_MEASURES,
        )
    }

    /**
     * Creates a packed coordinate sequence of type {@link #DOUBLE}
     * from the provided array
     * using the given coordinate dimension and measure count.
     *
     * @param packedCoordinates the array containing coordinate values
     * @param dimension the coordinate dimension
     * @param measures the coordinate measure count
     * @return a packed coordinate sequence of type {@link #DOUBLE}
     */
    pub fn create_double_coordinate_dimension_measures(
        packed_coordinates: &Vec<f64>,
        dimension: i32,
        measures: i32,
    ) -> PackedCoordinateSequenceDouble {
        PackedCoordinateSequenceDouble::new_f64_with_coordinates_dimension_measures(
            packed_coordinates,
            dimension,
            measures,
        )
    }

    /**
     * @see org.locationtech.jts.geom.CoordinateSequenceFactory#create(int, int)
     */
    pub fn create_with_size_dimension(
        size: usize,
        dimension: i32,
    ) -> PackedCoordinateSequenceDouble {
        PackedCoordinateSequenceDouble::new_with_size_dimension_measures(
            size,
            dimension,
            i32::max(
                PackedCoordinateSequenceFactory::DEFAULT_MEASURES,
                dimension - 3,
            ),
        )
    }

    /**
     * @see org.locationtech.jts.geom.CoordinateSequenceFactory#create(int, int, int)
     */
    pub fn create_with_size_dimension_measures(
        size: usize,
        dimension: i32,
        measures: i32,
    ) -> PackedCoordinateSequenceDouble {
        PackedCoordinateSequenceDouble::new_with_size_dimension_measures(size, dimension, measures)
    }
}
