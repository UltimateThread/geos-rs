use crate::geom::coordinate::Coordinate;

use super::coordinate_array_sequence::CoordinateArraySequence;

pub struct CoordinateArraySequenceFactory {}

impl CoordinateArraySequenceFactory {
    /**
     * Returns a {@link CoordinateArraySequence} based on the given array (the array is
     * not copied).
     *
     * @param coordinates
     *            the coordinates, which may not be null nor contain null
     *            elements
     */
    pub fn create_from_coordinates(coordinates: &Vec<Coordinate>) -> CoordinateArraySequence {
        CoordinateArraySequence::new_with_coordinates(coordinates)
    }

    /**
     * @see org.locationtech.jts.geom.CoordinateSequenceFactory#create(org.locationtech.jts.geom.CoordinateSequence)
     */
    pub fn create_from_coordinate_array_sequence(
        coord_seq: &CoordinateArraySequence,
    ) -> CoordinateArraySequence {
        CoordinateArraySequence::new_from_coordinate_array_sequence(&coord_seq)
    }

    /**
     * The created sequence dimension is clamped to be &lt;= 3.
     *
     * @see org.locationtech.jts.geom.CoordinateSequenceFactory#create(int, int)
     *
     */
    pub fn create_with_size_dimension(size: usize, mut dimension: i32) -> CoordinateArraySequence {
        if dimension > 3 {
            dimension = 3;
        }

        // handle bogus dimension
        if dimension < 2 {
            dimension = 2;
        }

        CoordinateArraySequence::new_with_size_dimension(size, dimension)
    }

    pub fn create_with_size_dimension_measures(
        size: usize,
        dimension: i32,
        mut measures: i32,
    ) -> CoordinateArraySequence {
        let mut spatial = dimension - measures;

        if measures > 1 {
            measures = 1; // clip measures
        }
        if spatial > 3 {
            spatial = 3; // clip spatial dimension
        }

        if spatial < 2 {
            spatial = 2;
        } // handle bogus spatial dimension

        CoordinateArraySequence::new_with_size_dimension_measures(
            size,
            spatial + measures,
            measures,
        )
    }
}
