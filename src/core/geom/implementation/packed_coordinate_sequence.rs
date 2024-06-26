use crate::core::geom::coordinate::Coordinate;

use super::packed_coordinate_sequence_double::PackedCoordinateSequenceDouble;



/**
 * A {@link CoordinateSequence} implementation based on a packed arrays.
 * In this implementation, {@link Coordinate}s returned by #toArray and #get are copies
 * of the internal values.
 * To change the actual values, use the provided setters.
 * <p>
 * For efficiency, created Coordinate arrays
 * are cached using a soft reference.
 * The cache is cleared each time the coordinate sequence contents are
 * modified through a setter method.
 *
 * @version 1.7
 */

#[derive(Clone)]
pub struct PackedCoordinateSequence {
    /**
     * The dimensions of the coordinates held in the packed array
     */
    dimension: i32,

    /**
     * The number of measures of the coordinates held in the packed array.
     */
    measures: i32,

    /**
     * A soft reference to the Coordinate[] representation of this sequence.
     * Makes repeated coordinate array accesses more efficient.
     */
    coords: Vec<Coordinate>,
}

impl PackedCoordinateSequence {
    /** Standard ordinate index value for, where X is 0 */
    pub const X: i32 = 0;

    /** Standard ordinate index value for, where Y is 1 */
    pub const Y: i32 = 1;

    /**
     * Standard ordinate index value for, where Z is 2.
     *
     * <p>This constant assumes XYZM coordinate sequence definition, please check this assumption
     * using {@link #getDimension()} and {@link #getMeasures()} before use.
     */
    /** Standard z-ordinate index */
    pub const Z: i32 = 2;

    /**
     * Standard ordinate index value for, where M is 3.
     *
     * <p>This constant assumes XYZM coordinate sequence definition, please check this assumption
     * using {@link #getDimension()} and {@link #getMeasures()} before use.
     */
    pub const M: i32 = 3;

    /**
     * Creates an instance of this class
     * @param dimension the total number of ordinates that make up a {@link Coordinate} in this sequence.
     * @param measures the number of measure-ordinates each {@link Coordinate} in this sequence has.
     */
    pub fn new_with_dimension_measures(dimension: i32, measures: i32) -> Option<Self> {
        if dimension - measures < 2 {
            return None;
        }

        Some(Self {
            dimension,
            measures,
            coords: vec![],
        })
    }

    /**
     * @see PackedCoordinateSequence#copy()
     */
    pub fn copy(&self) -> PackedCoordinateSequenceDouble {
        let clone = self.coords.to_vec();
        return PackedCoordinateSequenceDouble::new_coordinates_with_dimension(
            &clone,
            self.get_dimension(),
        );
    }

    /**
     * @see CoordinateSequence#getDimension()
     */
    pub fn get_dimension(&self) -> i32 {
        self.dimension
    }

    /**
     * @see CoordinateSequence#getMeasures()
     */
    pub fn get_measures(&self) -> i32 {
        self.measures
    }

    /**
     * @see CoordinateSequence#getCoordinate(int)
     */
    pub fn get_coordinate_by_index(&self, i: usize) -> Coordinate {
        return self.coords[i];
    }

    /**
     * @see CoordinateSequence#getCoordinate(int)
     */
    pub fn get_coordinate_copy_by_index(&self, i: usize) -> Option<Coordinate> {
        return self.get_coordinate_internal(i);
    }

    /**
     * @see CoordinateSequence#getCoordinate(int)
     */
    pub fn get_coordinate_by_index_coordinate(&self, i: usize, coord: &mut Coordinate) {
        coord.x = self.get_ordinate(i, 0);
        coord.y = self.get_ordinate(i, 1);
        if self.has_z() {
            coord.set_z(self.get_z(i));
        }
        if self.has_m() {
            coord.set_m(self.get_m(i));
        }
    }

    /**
     * @see CoordinateSequence#toCoordinateArray()
     */
    pub fn to_coordinate_array(&self) -> Vec<Coordinate> {
        let mut coords = vec![Coordinate::default(); self.coords.len()];
        for i in 0..coords.len() {
            coords[i] = self.get_coordinate_internal(i).unwrap();
        }

        return coords;
    }

    /**
     * Returns the size of the coordinate sequence
     *
     * @return the number of coordinates
     */
    pub fn size(&self) -> usize {
        return self.coords.len();
    }

    /**
     * Checks {@link #getDimension()} and {@link #getMeasures()} to determine if {@link #getZ(int)}
     * is supported.
     *
     * @return true if {@link #getZ(int)} is supported.
     */
    pub fn has_z(&self) -> bool {
        return (self.get_dimension() - self.get_measures()) > 2;
    }

    /**
     * Tests whether the coordinates in the sequence have measures associated with them. Returns true
     * if {@link #getMeasures()} {@code > 0}. See {@link #getMeasures()} to determine the number of measures
     * present.
     *
     * @return true if {@link #getM(int)} is supported.
     *
     * @see #getMeasures()
     * @see #getM(int)
     */
    pub fn has_m(&self) -> bool {
        return self.get_measures() > 0;
    }

    /**
    * Returns ordinate Z of the specified coordinate if available.
    *
    @param index  the coordinate index in the sequence
    * @return the value of the Z ordinate in the index'th coordinate, or Double.NaN if not defined.
    */
    pub fn get_z(&self, index: usize) -> f64 {
        if self.has_z() {
            return self.get_ordinate(index, 2);
        } else {
            return f64::NAN;
        }
    }

    /**
     * Returns ordinate M of the specified coordinate if available.
     *
     * @param index  the coordinate index in the sequence
     * @return the value of the M ordinate in the index'th coordinate, or Double.NaN if not defined.
     */
    pub fn get_m(&self, index: usize) -> f64 {
        if self.has_m() {
            let m_index = self.get_dimension() - self.get_measures();
            return self.get_ordinate(index, m_index);
        } else {
            return f64::NAN;
        }
    }

    /**
     * @see CoordinateSequence#getX(int)
     */
    pub fn get_x(&self, index: usize) -> f64 {
        return self.get_ordinate(index, 0);
    }

    /**
     * @see CoordinateSequence#getY(int)
     */
    pub fn get_y(&self, index: usize) -> f64 {
        return self.get_ordinate(index, 1);
    }

    /**
     * @see CoordinateSequence#getOrdinate(int, int)
     */
    pub fn get_ordinate(&self, index: usize, ordinate_index: i32) -> f64 {
        match ordinate_index {
            PackedCoordinateSequence::X => return self.coords[index].x,
            PackedCoordinateSequence::Y => return self.coords[index].y,
            _ => {
                let ord = self.coords[index].get_ordinate(ordinate_index as i32);
                match ord {
                    Some(ord) => return ord,
                    None => return f64::NAN,
                }
            }
        }
    }

    /**
     * Sets the first ordinate of a coordinate in this sequence.
     *
     * @param index  the coordinate index
     * @param value  the new ordinate value
     */
    pub fn set_x(&mut self, index: usize, value: f64) {
        self.set_ordinate(index, 0, value);
    }

    /**
     * Sets the second ordinate of a coordinate in this sequence.
     *
     * @param index  the coordinate index
     * @param value  the new ordinate value
     */
    pub fn set_y(&mut self, index: usize, value: f64) {
        self.set_ordinate(index, 1, value);
    }

    /**
     * Returns a Coordinate representation of the specified coordinate, by always
     * building a new Coordinate object
     *
     * @param index  the coordinate index
     * @return  the {@link Coordinate} at the given index
     */
    pub fn get_coordinate_internal(&self, index: usize) -> Option<Coordinate> {
        return Some(self.coords[index]);
    }

    /**
     * Sets the ordinate of a coordinate in this sequence.
     * <br>
     * Warning: for performance reasons the ordinate index is not checked
     * - if it is over dimensions you may not get an exception but a meaningless value.
     *
     * @param index
     *          the coordinate index
     * @param ordinate
     *          the ordinate index in the coordinate, 0 based, smaller than the
     *          number of dimensions
     * @param value
     *          the new ordinate value
     */
    pub fn set_ordinate(&mut self, index: usize, ordinate_index: i32, value: f64) {
        match ordinate_index {
            PackedCoordinateSequence::X => self.coords[index].x = value,
            PackedCoordinateSequence::Y => self.coords[index].y = value,
            _ => self.coords[index].set_ordinate(ordinate_index, value),
        }
    }
}
