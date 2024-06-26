use std::fmt;

use crate::core::geom::{coordinate::Coordinate, coordinate_arrays::CoordinateArrays, coordinates::Coordinates, envelope::Envelope};

#[derive(Clone)]
pub struct CoordinateArraySequence {
    /**
     * The actual dimension of the coordinates in the sequence.
     * Allowable values are 2, 3 or 4.
     */
    dimension: i32,
    /**
     * The number of measures of the coordinates in the sequence.
     * Allowable values are 0 or 1.
     */
    measures: i32,

    coordinates: Vec<Coordinate>,
}

impl CoordinateArraySequence {
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
     * Constructs a sequence based on the given array
     * of {@link Coordinate}s (the
     * array is not copied).
     * The coordinate dimension defaults to 3.
     *
     * @param coordinates the coordinate array that will be referenced.
     */
    pub fn new_with_coordinates(coordinates: &Vec<Coordinate>) -> Self {
        Self {
            dimension: CoordinateArrays::dimension(&coordinates),
            measures: CoordinateArrays::measures(&coordinates),
            coordinates: coordinates.to_vec(),
        }
    }

    /**
     * Constructs a sequence based on the given array
     * of {@link Coordinate}s (the
     * array is not copied).
     *
     * @param coordinates the coordinate array that will be referenced.
     * @param dimension the dimension of the coordinates
     */
    pub fn new_with_coordinates_dimension(coordinates: Vec<Coordinate>, dimension: i32) -> Self {
        Self {
            dimension,
            measures: CoordinateArrays::measures(&coordinates),
            coordinates,
        }
    }

    /**
     * Constructs a sequence based on the given array
     * of {@link Coordinate}s (the array is not copied).
     * <p>
     * It is your responsibility to ensure the array contains Coordinates of the
     * indicated dimension and measures (See
     * {@link CoordinateArrays#enforceConsistency(Coordinate[])} ).</p>
     *
     * @param coordinates the coordinate array that will be referenced.
     * @param dimension the dimension of the coordinates
     */
    pub fn new_with_coordinates_dimension_measures(
        coordinates: Vec<Coordinate>,
        dimension: i32,
        measures: i32,
    ) -> Self {
        Self {
            dimension,
            measures,
            coordinates: coordinates,
        }
    }

    /**
     * Constructs a sequence of a given size, populated
     * with new {@link Coordinate}s.
     *
     * @param size the size of the sequence to create
     */
    pub fn new_with_size(size: usize) -> Self {
        Self {
            dimension: 3,
            measures: 0,
            coordinates: vec![Coordinate::default(); size],
        }
    }

    /**
     * Constructs a sequence of a given size, populated
     * with new {@link Coordinate}s.
     *
     * @param size the size of the sequence to create
     * @param dimension the dimension of the coordinates
     */
    pub fn new_with_size_dimension(size: usize, dimension: i32) -> Self {
        Self {
            dimension,
            measures: 0,
            coordinates: vec![Coordinates::create_dim(dimension); size],
        }
    }

    /**
     * Constructs a sequence of a given size, populated
     * with new {@link Coordinate}s.
     *
     * @param size the size of the sequence to create
     * @param dimension the dimension of the coordinates
     */
    pub fn new_with_size_dimension_measures(size: usize, dimension: i32, measures: i32) -> Self {
        Self {
            dimension,
            measures,
            coordinates: vec![
                CoordinateArraySequence::create_coordinate(dimension, measures);
                size
            ],
        }
    }

    /*
     * Creates a new sequence based on a deep copy of the given {@link CoordinateSequence}.
     * The coordinate dimension is set to equal the dimension of the input.
     *
     * @param coordSeq the coordinate sequence that will be copied.
     */
    pub fn new_from_coordinate_array_sequence(coord_seq: &CoordinateArraySequence) -> Self {
        let mut new = Self {
            dimension: coord_seq.get_dimension(),
            measures: coord_seq.get_measures(),
            coordinates: vec![Coordinate::default(); coord_seq.size()],
        };

        for i in 0..new.coordinates.len() {
            new.coordinates[i] = coord_seq.get_coordinate_copy(i);
        }

        new
    }

    /**
     * Creates a coordinate for use in this sequence.
     * <p>
     * The coordinate is created supporting the same number of {@link #getDimension()} and {@link #getMeasures()}
     * as this sequence and is suitable for use with {@link #getCoordinate(int, Coordinate)}.
     * </p>
     * @return coordinate for use with this sequence
     */
    pub fn create_coordinate(dimension: i32, measures: i32) -> Coordinate {
        return Coordinates::create_dim_measures(dimension, measures);
    }

    pub fn create_coordinate_default(&self) -> Coordinate {
        return Coordinates::create_dim_measures(self.get_dimension(), self.get_measures());
    }

    /**
     * @see org.locationtech.jts.geom.CoordinateSequence#getDimension()
     */
    pub fn get_dimension(&self) -> i32 {
        self.dimension
    }

    pub fn get_measures(&self) -> i32 {
        self.measures
    }

    /**
     * Get the Coordinate with index i.
     *
     * @param i
     *                  the index of the coordinate
     * @return the requested Coordinate instance
     */
    pub fn get_coordinate_index(&self, i: usize) -> Coordinate {
        self.coordinates[i]
    }

    /**
     * Get a copy of the Coordinate with index i.
     *
     * @param i  the index of the coordinate
     * @return a copy of the requested Coordinate
     */
    pub fn get_coordinate_copy(&self, i: usize) -> Coordinate {
        self.coordinates[i]
    }

    /**
     * @see org.locationtech.jts.geom.CoordinateSequence#getX(int)
     */
    pub fn get_coordinate_index_coordinate(&self, index: usize, coord: &mut Coordinate) {
        coord.set_coordinate(&self.coordinates[index]);
    }

    /**
     * @see org.locationtech.jts.geom.CoordinateSequence#getX(int)
     */
    pub fn get_x(&self, index: usize) -> f64 {
        return self.coordinates[index].x;
    }

    /**
     * @see org.locationtech.jts.geom.CoordinateSequence#getY(int)
     */
    pub fn get_y(&self, index: usize) -> f64 {
        return self.coordinates[index].y;
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
     * @see org.locationtech.jts.geom.CoordinateSequence#getZ(int)
     */
    pub fn get_z(&self, index: usize) -> f64 {
        if self.has_z() {
            return self.coordinates[index].get_z();
        } else {
            return f64::NAN;
        }
    }

    /**
     * @see org.locationtech.jts.geom.CoordinateSequence#getM(int)
     */
    pub fn get_m(&self, index: usize) -> f64 {
        if self.has_m() {
            return self.coordinates[index].get_m();
        } else {
            return f64::NAN;
        }
    }

    /**
     * @see org.locationtech.jts.geom.CoordinateSequence#getOrdinate(int, int)
     */
    pub fn get_ordinate(&self, index: usize, ordinate_index: i32) -> f64 {
        match ordinate_index {
            CoordinateArraySequence::X => return self.coordinates[index].x,
            CoordinateArraySequence::Y => return self.coordinates[index].y,
            _ => {
                let ord = self.coordinates[index].get_ordinate(ordinate_index as i32);
                match ord {
                    Some(ord) => return ord,
                    None => return f64::NAN,
                }
            }
        }
    }

    /**
     * Creates a deep copy of the CoordinateArraySequence
     *
     * @return The deep copy
     */
    pub fn copy(&self) -> CoordinateArraySequence {
        let mut clone_coordinates: Vec<Coordinate> =
            vec![Coordinate::default(); self.coordinates.len()];
        for i in 0..self.coordinates.len() {
            clone_coordinates[i] = self.coordinates[i];
        }
        return CoordinateArraySequence::new_with_coordinates_dimension_measures(
            clone_coordinates,
            self.dimension,
            self.measures,
        );
    }

    /**
     * Returns the size of the coordinate sequence
     *
     * @return the number of coordinates
     */
    pub fn size(&self) -> usize {
        return self.coordinates.len();
    }

    /**
     * @see org.locationtech.jts.geom.CoordinateSequence#setOrdinate(int, int, double)
     */
    pub fn set_ordinate(&mut self, index: usize, ordinate_index: i32, value: f64) {
        match ordinate_index {
            CoordinateArraySequence::X => self.coordinates[index].x = value,
            CoordinateArraySequence::Y => self.coordinates[index].y = value,
            _ => self.coordinates[index].set_ordinate(ordinate_index, value),
        }
    }

    /**
     * This method exposes the internal Array of Coordinate Objects
     *
     * @return the Coordinate[] array.
     */
    pub fn to_coordinate_array(&self) -> Vec<Coordinate> {
        return self.coordinates.to_vec();
    }

    pub fn expand_envelope(&self, env: &mut Envelope) {
        for i in 0..self.coordinates.len() {
            env.expand_to_include_coordinate(&self.coordinates[i]);
        }
    }
}

impl fmt::Display for CoordinateArraySequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.coordinates.len() > 0 {
            let mut str = "(".to_string();
            str = str + &format!("{}", self.coordinates[0]);
            for i in 1..self.coordinates.len() {
                str = str + &format!(", ");
                str = str + &format!("{}", self.coordinates[i]);
            }
            str = str + &format!(")");
            write!(f, "{}", str)
        } else {
            write!(f, "()")
        }
    }
}
