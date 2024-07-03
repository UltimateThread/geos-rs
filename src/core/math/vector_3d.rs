use crate::core::geom::coordinate::Coordinate;

/**
 * Represents a vector in 3-dimensional Cartesian space.
 *
 * @author mdavis
 *
 */

pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    /**
     * Creates a new 3D vector from a {@link Coordinate}. The coordinate should have
     * the X,Y and Z ordinates specified.
     *
     * @param v the Coordinate to copy
     */
    pub fn new_from_coordinate(v: &Coordinate) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.get_z(),
        }
    }

    /**
     * Creates a new vector with the direction and magnitude
     * of the difference between the
     * <tt>to</tt> and <tt>from</tt> {@link Coordinate}s.
     *
     * @param from the origin Coordinate
     * @param to the destination Coordinate
     */
    pub fn new_from_to_coordinates(from: &Coordinate, to: &Coordinate) -> Self {
        Self {
            x: to.x - from.x,
            y: to.y - from.y,
            z: to.get_z() - from.get_z(),
        }
    }

    /**
     * Creates a vector with the givne components.
     *
     * @param x the X component
     * @param y the Y component
     * @param z the Z component
     */
    pub fn new_from_xyz(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /**
     * Creates a new vector with given X, Y and Z components.
     *
     * @param x the X component
     * @param y the Y component
     * @param z the Z component
     * @return a new vector
     */
    pub fn create_from_xyz(x: f64, y: f64, z: f64) -> Vector3D {
        return Vector3D::new_from_xyz(x, y, z);
    }

    /**
     * Creates a vector from a 3D {@link Coordinate}.
     * The coordinate should have the
     * X,Y and Z ordinates specified.
     *
     * @param coord the Coordinate to copy
     * @return a new vector
     */
    pub fn create_from_coordinate(coord: &Coordinate) -> Self {
        return Vector3D::new_from_coordinate(coord);
    }

    /**
     * Computes the dot product of the 3D vectors AB and CD.
     *
     * @param A the start point of the first vector
     * @param B the end point of the first vector
     * @param C the start point of the second vector
     * @param D the end point of the second vector
     * @return the dot product
     */
    pub fn dot_4(ac: &Coordinate, bc: &Coordinate, cc: &Coordinate, dc: &Coordinate) -> f64 {
        let abx = bc.x - ac.x;
        let aby = bc.y - ac.y;
        let abz = bc.get_z() - ac.get_z();
        let cdx = dc.x - cc.x;
        let cdy = dc.y - cc.y;
        let cdz = dc.get_z() - cc.get_z();
        return abx * cdx + aby * cdy + abz * cdz;
    }

    /**
     * Computes the 3D dot-product of two {@link Coordinate}s.
     *
     * @param v1 the first vector
     * @param v2 the second vector
     * @return the dot product of the vectors
     */
    pub fn dot_2(v1: &Coordinate, v2: &Coordinate) -> f64 {
        return v1.x * v2.x + v1.y * v2.y + v1.get_z() * v2.get_z();
    }

    /**
     * Gets the X component of this vector.
     *
     * @return the value of the X component
     */
    pub fn get_x(&self) -> f64 {
        return self.x;
    }

    /**
     * Gets the Y component of this vector.
     *
     * @return the value of the Y component
     */
    pub fn get_y(&self) -> f64 {
        return self.y;
    }

    /**
     * Gets the Z component of this vector.
     *
     * @return the value of the Z component
     */
    pub fn get_z(&self) -> f64 {
        return self.z;
    }

    /**
     * Computes a vector which is the sum
     * of this vector and the given vector.
     *
     * @param v the vector to add
     * @return the sum of this and <code>v</code>
     */
    pub fn add(&self, v: &Vector3D) -> Vector3D {
        return Vector3D::create_from_xyz(self.x + v.x, self.y + v.y, self.z + v.z);
    }

    /**
     * Computes a vector which is the difference
     * of this vector and the given vector.
     *
     * @param v the vector to subtract
     * @return the difference of this and <code>v</code>
     */
    pub fn subtract(&self, v: &Vector3D) -> Vector3D {
        return Vector3D::create_from_xyz(self.x - v.x, self.y - v.y, self.z - v.z);
    }

    /**
     * Creates a new vector which has the same direction
     * and with length equals to the length of this vector
     * divided by the scalar value <code>d</code>.
     *
     * @param d the scalar divisor
     * @return a new vector with divided length
     */
    pub fn divide(&self, d: f64) -> Vector3D {
        return Vector3D::create_from_xyz(self.x / d, self.y / d, self.z / d);
    }

    /**
     * Computes the dot-product of two vectors
     *
     * @param v a vector
     * @return the dot product of the vectors
     */
    pub fn dot(&self, v: &Vector3D) -> f64 {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }

    /**
     * Computes the length of this vector.
     *
     * @return the length of the vector
     */
    pub fn length(&self) -> f64 {
        return f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }

    /**
     * Computes the length of a vector.
     *
     * @param v a coordinate representing a 3D vector
     * @return the length of the vector
     */
    pub fn length_of_coordinate(v: &Coordinate) -> f64 {
        return f64::sqrt(v.x * v.x + v.y * v.y + v.get_z() * v.get_z());
    }

    /**
     * Computes a vector having identical direction
     * but normalized to have length 1.
     *
     * @return a new normalized vector
     */
    pub fn normalize(&self) -> Vector3D {
        let length = self.length();
        if length > 0.0 {
            return self.divide(self.length());
        }
        return Vector3D::create_from_xyz(0.0, 0.0, 0.0);
    }

    /**
     * Computes a vector having identical direction
     * but normalized to have length 1.
     *
     * @param v a coordinate representing a 3D vector
     * @return a coordinate representing the normalized vector
     */
    pub fn normalize_coordinate(v: &Coordinate) -> Coordinate {
        let len = Vector3D::length_of_coordinate(v);
        return Coordinate::new_xyz(v.x / len, v.y / len, v.get_z() / len);
    }

    /**
     * Gets a string representation of this vector
     *
     * @return a string representing this vector
     */
    pub fn to_string(&self) -> String {
        let str = format!("[{}, {}, {}]", self.x, self.y, self.z);
        return str;
    }

    /**
     * Tests if a vector <tt>o</tt> has the same values for the components.
     *
     * @param o a <tt>Vector3D</tt> with which to do the comparison.
     * @return true if <tt>other</tt> is a <tt>Vector3D</tt> with the same values
     *         for the x and y components.
     */
    pub fn equals(&self, v: &Vector3D) -> bool {
        return self.x == v.x && self.y == v.y && self.z == v.z;
    }
}
