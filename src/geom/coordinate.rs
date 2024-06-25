use std::fmt;

use crate::util::number_util::NumberUtil;

#[derive(Copy, Clone)]
pub struct Coordinate {
    /**
     * The x-ordinate.
     */
    pub x: f64,

    /**
     * The y-ordinate.
     */
    pub y: f64,

    /**
     * The z-ordinate.
     */
    pub z: f64,

    /**
     * The m-ordinate.
     */
    pub m: f64,
}

impl Coordinate {
    /**
     * The value used to indicate a null or missing ordinate value.
     * In particular, used for the value of ordinates for dimensions
     * greater than the defined dimension of a coordinate.
     */
    pub const NULL_ORDINATE: f64 = f64::NAN;

    /** Standard ordinate index value for, where X is 0 */
    pub const X: i32 = 0;

    /** Standard ordinate index value for, where Y is 1 */
    pub const Y: i32 = 1;

    /**
     * Standard ordinate index value for, where Z is 2.
     *
     * <p>This constant assumes XYZM coordinate sequence definition, please check this assumption
     * using {@link CoordinateSequence#getDimension()} and {@link CoordinateSequence#getMeasures()}
     * before use.
     */
    pub const Z: i32 = 2;

    /**
     * Standard ordinate index value for, where M is 3.
     *
     * <p>This constant assumes XYZM coordinate sequence definition, please check this assumption
     * using {@link CoordinateSequence#getDimension()} and {@link CoordinateSequence#getMeasures()}
     * before use.
     */
    pub const M: i32 = 3;

    /**
     *  Constructs a <code>Coordinate</code> at (0,0,NaN).
     */
    pub fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: f64::NAN,
            m: f64::NAN,
        }
    }

    /**
     *  Constructs a <code>Coordinate</code> at (x,y,NaN).
     *
     *@param  x  the x-value
     *@param  y  the y-value
     */
    pub fn new_xy(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            z: f64::NAN,
            m: f64::NAN,
        }
    }

    /**
     *  Constructs a <code>Coordinate</code> at (x,y,z).
     *
     *@param  x  the x-ordinate
     *@param  y  the y-ordinate
     *@param  z  the z-ordinate
     */
    pub fn new_xyz(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            m: f64::NAN,
        }
    }

    /**
     *  Constructs a <code>Coordinate</code> at (x,y,z).
     *
     *@param  x  the x-ordinate
     *@param  y  the y-ordinate
     *@param  z  the z-ordinate
     */
    pub fn new_xym(x: f64, y: f64, m: f64) -> Self {
        Self {
            x,
            y,
            z: f64::NAN,
            m,
        }
    }

    pub fn new_xyzm(x: f64, y: f64, z: f64, m: f64) -> Self {
        Self {
            x,
            y,
            z,
            m,
        }
    }

    /**
     *  Constructs a <code>Coordinate</code> having the same (x,y,z) values as
     *  <code>other</code>.
     *
     *@param  c  the <code>Coordinate</code> to copy.
     */
    pub fn from_coordinate(c: &Coordinate) -> Self {
        Self {
            x: c.x,
            y: c.y,
            z: c.get_z(),
            m: c.get_m(),
        }
    }

    pub fn new_coordinatexy_default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: f64::NAN,
            m: f64::NAN,
        }
    }

    pub fn new_coordinatexym_default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: f64::NAN,
            m: 0.,
        }
    }

    pub fn new_coordinatexyzm_default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
            m: 0.,
        }
    }

    pub fn new_coordinatexy(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            z: f64::NAN,
            m: f64::NAN,
        }
    }

    pub fn new_coordinatexym(x: f64, y: f64, m: f64) -> Self {
        Self {
            x,
            y,
            z: f64::NAN,
            m,
        }
    }

    pub fn new_coordinatexyzm(x: f64, y: f64, z: f64, m: f64) -> Self {
        Self { x, y, z, m }
    }

    /**
     * Create a new Coordinate of the same type as this Coordinate, but with no values.
     *
     * @return a new Coordinate
     */
    pub fn create() -> Self {
        Coordinate::default()
    }

    /**
     *  Sets this <code>Coordinate</code>s (x,y,z) values to that of <code>other</code>.
     *
     *@param  other  the <code>Coordinate</code> to copy
     */
    pub fn set_coordinate(&mut self, other: &Coordinate) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.get_z();
    }

    /**
     *  Retrieves the value of the X ordinate.
     *  
     *  @return the value of the X ordinate
     */
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /**
     * Sets the X ordinate value.
     *
     * @param x the value to set as X
     */
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /**
     *  Retrieves the value of the Y ordinate.
     *  
     *  @return the value of the Y ordinate
     */
    pub fn get_y(&self) -> f64 {
        self.y
    }

    /**
     * Sets the Y ordinate value.
     *
     * @param y the value to set as Y
     */
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /**
     *  Retrieves the value of the Z ordinate, if present.
     *  If no Z value is present returns <tt>NaN</tt>.
     *  
     *  @return the value of the Z ordinate, or <tt>NaN</tt>
     */
    pub fn get_z(&self) -> f64 {
        self.z
    }

    /**
     * Sets the Z ordinate value.
     *
     * @param z the value to set as Z
     */
    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }

    /**
     *  Retrieves the value of the measure, if present.
     *  If no measure value is present returns <tt>NaN</tt>.
     *  
     *  @return the value of the measure, or <tt>NaN</tt>
     */
    pub fn get_m(&self) -> f64 {
        self.m
    }

    /**
     * Sets the M ordinate value.
     *
     * @param m the value to set as M
     */
    pub fn set_m(&mut self, m: f64) {
        self.m = m;
    }

    /**
     * Gets the ordinate value for the given index.
     *
     * The base implementation supports values for the index are
     * {@link #X}, {@link #Y}, and {@link #Z}.
     *
     * @param ordinateIndex the ordinate index
     * @return the value of the ordinate
     * @throws IllegalArgumentException if the index is not valid
     */
    pub fn get_ordinate(&self, ordinate_index: i32) -> Option<f64> {
        match ordinate_index {
            Coordinate::X => return Some(self.x),
            Coordinate::Y => return Some(self.y),
            Coordinate::Z => return Some(self.z),
            _ => return None,
        }
    }

    /**
     * Sets the ordinate for the given index
     * to a given value.
     *
     * The base implementation supported values for the index are
     * {@link #X}, {@link #Y}, and {@link #Z}.
     *
     * @param ordinateIndex the ordinate index
     * @param value the value to set
     * @throws IllegalArgumentException if the index is not valid
     */
    pub fn set_ordinate(&mut self, ordinate_index: i32, value: f64) {
        match ordinate_index {
            Coordinate::X => self.x = value,
            Coordinate::Y => self.y = value,
            Coordinate::Z => self.set_z(value), // delegate to subclass rather than offer direct field acces,
            _ => {}
        }
    }

    /**
     * Tests if the coordinate has valid X and Y ordinate values.
     * An ordinate value is valid iff it is finite.
     *
     * @return true if the coordinate is valid
     * @see Double#isFinite(double)
     */
    pub fn is_valid(&self) -> bool {
        if !f64::is_finite(self.x) {
            return false;
        }
        if !f64::is_finite(self.y) {
            return false;
        }
        return true;
    }
    
    pub fn is_xy(&self) -> bool {
        !f64::is_nan(self.x) && !f64::is_nan(self.y) && f64::is_nan(self.z) && f64::is_nan(self.m)
    }

    pub fn is_xym(&self) -> bool {
        !f64::is_nan(self.x) && !f64::is_nan(self.y) && f64::is_nan(self.z) && !f64::is_nan(self.m)
    }

    pub fn is_xyzm(&self) -> bool {
        !f64::is_nan(self.x) && !f64::is_nan(self.y) && !f64::is_nan(self.z) && !f64::is_nan(self.m)
    }

    /**
     *  Returns whether the planar projections of the two <code>Coordinate</code>s
     *  are equal.
     *
     *@param  other  a <code>Coordinate</code> with which to do the 2D comparison.
     *@return        <code>true</code> if the x- and y-coordinates are equal; the
     *      z-coordinates do not have to be equal.
     */
    pub fn equals_2d(&self, other: &Coordinate) -> bool {
        if self.x != other.x {
            return false;
        }
        if self.y != other.y {
            return false;
        }
        return true;
    }

    /**
     * Tests if another Coordinate has the same values for the X and Y ordinates,
     * within a specified tolerance value.
     * The Z ordinate is ignored.
     *
     *@param c a <code>Coordinate</code> with which to do the 2D comparison.
     *@param tolerance the tolerance value to use
     *@return true if <code>other</code> is a <code>Coordinate</code>
     *      with the same values for X and Y.
     */
    pub fn equals_2d_with_tolerance(&self, c: &Coordinate, tolerance: f64) -> bool {
        if !NumberUtil::equals_with_tolerance(self.x, c.x, tolerance) {
            return false;
        }
        if !NumberUtil::equals_with_tolerance(self.y, c.y, tolerance) {
            return false;
        }
        return true;
    }

    /**
     * Tests if another coordinate has the same values for the X, Y and Z ordinates.
     *
     *@param other a <code>Coordinate</code> with which to do the 3D comparison.
     *@return true if <code>other</code> is a <code>Coordinate</code>
     *      with the same values for X, Y and Z.
     */
    pub fn equals_3d(&self, other: &Coordinate) -> bool {
        (self.x == other.x)
            && (self.y == other.y)
            && ((self.get_z() == other.get_z())
                || (f64::is_nan(self.get_z()) && f64::is_nan(other.get_z())))
    }

    /**
     * Tests if another coordinate has the same value for Z, within a tolerance.
     *
     * @param c a coordinate
     * @param tolerance the tolerance value
     * @return true if the Z ordinates are within the given tolerance
     */
    pub fn equal_in_z(&self, c: &Coordinate, tolerance: f64) -> bool {
        return NumberUtil::equals_with_tolerance(self.get_z(), c.get_z(), tolerance);
    }

        /**
     * Tests if another coordinate has the same value for Z, within a tolerance.
     *
     * @param c a coordinate
     * @param tolerance the tolerance value
     * @return true if the Z ordinates are within the given tolerance
     */
    pub fn equal_in_coordinate_z(&self, c: &Coordinate, tolerance: f64) -> bool {
        return NumberUtil::equals_with_tolerance(self.get_z(), c.get_z(), tolerance);
    }

    /**
     *  Compares this {@link Coordinate} with the specified {@link Coordinate} for order.
     *  This method ignores the z value when making the comparison.
     *  Returns:
     *  <UL>
     *    <LI> -1 : this.x &lt; other.x || ((this.x == other.x) &amp;&amp; (this.y &lt; other.y))
     *    <LI> 0 : this.x == other.x &amp;&amp; this.y = other.y
     *    <LI> 1 : this.x &gt; other.x || ((this.x == other.x) &amp;&amp; (this.y &gt; other.y))
     *
     *  </UL>
     *  Note: This method assumes that ordinate values
     * are valid numbers.  NaN values are not handled correctly.
     *
     *@param  o  the <code>Coordinate</code> with which this <code>Coordinate</code>
     *      is being compared
     *@return    -1, zero, or 1 as this <code>Coordinate</code>
     *      is less than, equal to, or greater than the specified <code>Coordinate</code>
     */
    pub fn compare_to(&self, other: &Coordinate) -> i32 {
        if self.x < other.x {
            return -1;
        }
        if self.x > other.x {
            return 1;
        }
        if self.y < other.y {
            return -1;
        }
        if self.y > other.y {
            return 1;
        }
        return 0;
    }

    /**
     * Computes the 2-dimensional Euclidean distance to another location.
     * The Z-ordinate is ignored.
     *
     * @param c a point
     * @return the 2-dimensional Euclidean distance between the locations
     */
    pub fn distance(&self, c: &Coordinate) -> f64 {
        let dx = self.x - c.x;
        let dy = self.y - c.y;

        f64::hypot(dx, dy)
    }

    /**
     * Computes the 3-dimensional Euclidean distance to another location.
     *
     * @param c a coordinate
     * @return the 3-dimensional Euclidean distance between the locations
     */
    pub fn distance_3d(&self, c: &Coordinate) -> f64 {
        let dx = self.x - c.x;
        let dy = self.y - c.y;
        let dz = self.get_z() - c.get_z();
        f64::sqrt(dx * dx + dy * dy + dz * dz)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.get_z())
    }
}
