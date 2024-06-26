use crate::core::geom::{coordinate::Coordinate, envelope::Envelope};


/**
 * Packed coordinate sequence implementation based on doubles
 */

#[derive(Clone)]
pub struct PackedCoordinateSequenceDouble {
    /**
     * The dimensions of the coordinates held in the packed array
     */
    dimension: i32,

    /**
     * The number of measures of the coordinates held in the packed array.
     */
    measures: i32,

    /**
     * The packed coordinate array
     */
    coords: Vec<f64>,
}

impl PackedCoordinateSequenceDouble {
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
     * Builds a new packed coordinate sequence
     *
     * @param coords  an array of <code>double</code> values that contains the ordinate values of the sequence
     * @param dimension the total number of ordinates that make up a {@link Coordinate} in this sequence.
     * @param measures the number of measure-ordinates each {@link Coordinate} in this sequence has.
     */
    pub fn new_f64_with_coordinates_dimension_measures(
        coords: &Vec<f64>,
        dimension: i32,
        measures: i32,
    ) -> Self {
        let mut new = Self {
            dimension,
            measures,
            coords: vec![],
        };

        new.coords = vec![f64::NAN; coords.len()];
        for i in 0..coords.len() {
            new.coords[i] = coords[i];
        }

        new
    }

    /**
     * Builds a new packed coordinate sequence out of a coordinate array
     *
     * @param coordinates an array of {@link Coordinate}s
     * @param dimension the total number of ordinates that make up a {@link Coordinate} in this sequence.
     */
    pub fn new_coordinates_with_dimension(coordinates: &Vec<Coordinate>, dimension: i32) -> Self {
        let mut new = Self {
            dimension,
            measures: i32::max(0, dimension - 3),
            coords: vec![],
        };

        for i in 0..coordinates.len() {
            let offset = i * dimension as usize;
            new.coords[offset] = coordinates[i].x;
            new.coords[offset + 1] = coordinates[i].y;
            if dimension >= 3 {
                new.coords[offset + 2] = coordinates[i].get_ordinate(2).unwrap();
                // Z or M
            }
            if dimension >= 4 {
                new.coords[offset + 3] = coordinates[i].get_ordinate(3).unwrap();
                // M
            }
        }

        new
    }

    /**
     * Builds a new packed coordinate sequence out of a coordinate array
     *
     * @param coordinates an array of {@link Coordinate}s
     * @param dimension the total number of ordinates that make up a {@link Coordinate} in this sequence.
     * @param measures the number of measure-ordinates each {@link Coordinate} in this sequence has.
     */
    pub fn new_coordinates_with_dimension_measures(
        coordinates: &Vec<Coordinate>,
        dimension: i32,
        measures: i32,
    ) -> Self {
        let mut new = Self {
            dimension,
            measures,
            coords: vec![],
        };

        for i in 0..coordinates.len() {
            let offset = i * dimension as usize;
            new.coords[offset] = coordinates[i].x;
            new.coords[offset + 1] = coordinates[i].y;
            if dimension >= 3 {
                new.coords[offset + 2] = coordinates[i].get_ordinate(2).unwrap();
                // Z or M
            }
            if dimension >= 4 {
                new.coords[offset + 3] = coordinates[i].get_ordinate(3).unwrap();
                // M
            }
        }

        new
    }

    /**
     * Builds a new packed coordinate sequence out of a coordinate array
     *
     * @param coordinates an array of {@link Coordinate}s
     */
    pub fn new_with_coordinates(coordinates: &Vec<Coordinate>) -> Self {
        let mut new = Self {
            dimension: 3,
            measures: 0,
            coords: vec![],
        };

        for i in 0..coordinates.len() {
            let offset = i * new.dimension as usize;
            new.coords[offset] = coordinates[i].x;
            new.coords[offset + 1] = coordinates[i].y;
            if new.dimension >= 3 {
                new.coords[offset + 2] = coordinates[i].get_ordinate(2).unwrap();
                // Z or M
            }
            if new.dimension >= 4 {
                new.coords[offset + 3] = coordinates[i].get_ordinate(3).unwrap();
                // M
            }
        }

        new
    }

    /**
     * Builds a new empty packed coordinate sequence of a given size and dimension
     *
     * @param size the number of coordinates in this sequence
     * @param dimension the total number of ordinates that make up a {@link Coordinate} in this sequence.
     * @param measures the number of measure-ordinates each {@link Coordinate} in this sequence has.
     */
    pub fn new_with_size_dimension_measures(size: usize, dimension: i32, measures: i32) -> Self {
        Self {
            dimension,
            measures,
            coords: vec![f64::NAN; size * dimension as usize],
        }
    }

    /**
     * @see PackedCoordinateSequence#getCoordinate(int)
     */
    pub fn get_coordinate_internal(&self, i: usize) -> Coordinate {
        let x = self.coords[i * self.dimension as usize];
        let y = self.coords[i * self.dimension as usize + 1];
        if self.dimension == 2 && self.measures == 0 {
            return Coordinate::new_xy(x, y);
        } else if self.dimension == 3 && self.measures == 0 {
            let z = self.coords[i * self.dimension as usize + 2];
            return Coordinate::new_xyz(x, y, z);
        } else if self.dimension == 3 && self.measures == 1 {
            let m = self.coords[i * self.dimension as usize + 2];
            return Coordinate::new_xym(x, y, m);
        } else if self.dimension == 4 {
            let z = self.coords[i * self.dimension as usize + 2];
            let m = self.coords[i * self.dimension as usize + 3];
            return Coordinate::new_xyzm(x, y, z, m);
        }
        return Coordinate::new_xy(x, y);
    }

    /**
     * Gets the underlying array containing the coordinate values.
     *
     * @return the array of coordinate values
     */
    pub fn get_raw_coordinates(&self) -> Vec<f64> {
        return self.coords.to_vec();
    }

    /**
     * @see CoordinateSequence#getCoordinate(int)
     */
    pub fn get_coordinate_by_index(&self, i: usize) -> Coordinate {
        return self.get_coordinate_internal(i);
    }

    /**
     * @see CoordinateSequence#size()
     */
    pub fn size(&self) -> usize {
        return self.coords.len() / self.dimension as usize;
    }

    /**
     * @see PackedCoordinateSequence#size()
     */
    pub fn copy(&self) -> PackedCoordinateSequenceDouble {
        let clone = self.coords.to_vec();

        PackedCoordinateSequenceDouble::new_f64_with_coordinates_dimension_measures(
            &clone,
            self.dimension,
            self.measures,
        )
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
     * @see PackedCoordinateSequence#getOrdinate(int, int)
     *      Beware, for performance reasons the ordinate index is not checked, if
     *      it's over dimensions you may not get an exception but a meaningless
     *      value.
     */
    pub fn get_ordinate(&self, index: usize, ordinate: i32) -> f64 {
        return self.coords[index * self.dimension as usize + ordinate as usize];
    }

    /**
     * @see PackedCoordinateSequence#setOrdinate(int, int, double)
     */
    pub fn set_ordinate(&mut self, index: usize, ordinate: i32, value: f64) {
        self.coords[index * self.dimension as usize + ordinate as usize] = value;
    }

    /**
     * @see CoordinateSequence#toCoordinateArray()
     */
    pub fn to_coordinate_array(&self) -> Vec<Coordinate> {
        let mut coords = vec![Coordinate::default(); self.size()];
        for i in 0..coords.len() {
            coords[i] = self.get_coordinate_internal(i);
        }

        return coords;
    }

    /**
     * @see CoordinateSequence#expandEnvelope(Envelope)
     */
    pub fn expand_envelope(&self, env: &Envelope) -> Envelope {
        let mut copy = *env;
        let mut i = 0;
        while i < self.coords.len() {
            if i + 1 < self.coords.len() {
                copy.expand_to_include_xy(self.coords[i], self.coords[i + 1]);
            }

            i += self.dimension as usize;
        }
        return copy;
    }
}
