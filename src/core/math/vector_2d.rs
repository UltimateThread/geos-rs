use crate::core::{
    algorithm::{angle::Angle, cg_algorithms_dd::CGAlgorithmsDD},
    geom::coordinate::Coordinate,
};

/**
 * A 2-dimensional mathematical vector represented by double-precision X and Y components.
 *
 * @author mbdavis
 *
 */
#[derive(Clone, Copy)]
pub struct Vector2D {
    /**
     * The X component of this vector.
     */
    x: f64,

    /**
     * The Y component of this vector.
     */
    y: f64,
}

impl Vector2D {
    pub fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn new_from_xy(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn new_from_vector_2d(v: &Vector2D) -> Self {
        Self { x: v.x, y: v.y }
    }

    pub fn new_from_coordinate_from_to(from: &Coordinate, to: &Coordinate) -> Self {
        Self {
            x: to.x - from.x,
            y: to.y - from.y,
        }
    }

    pub fn new_from_coordinate(v: &Coordinate) -> Self {
        Self { x: v.x, y: v.y }
    }

    /**
     * Creates a new vector with given X and Y components.
     *
     * @param x the x component
     * @param y the y component
     * @return a new vector
     */
    pub fn create_from_xy(x: f64, y: f64) -> Vector2D {
        return Vector2D::new_from_xy(x, y);
    }

    /**
     * Creates a new vector from an existing one.
     *
     * @param v the vector to copy
     * @return a new vector
     */
    pub fn create_from_vector_2d(v: &Vector2D) -> Vector2D {
        return Vector2D::new_from_vector_2d(v);
    }

    /**
     * Creates a vector from a {@link Coordinate}.
     *
     * @param coord the Coordinate to copy
     * @return a new vector
     */
    pub fn create_from_coodinate(coord: &Coordinate) -> Vector2D {
        return Vector2D::new_from_coordinate(coord);
    }

    /**
     * Creates a vector with the direction and magnitude
     * of the difference between the
     * <tt>to</tt> and <tt>from</tt> {@link Coordinate}s.
     *
     * @param from the origin Coordinate
     * @param to the destination Coordinate
     * @return a new vector
     */
    pub fn create_from_coordinate_from_to(from: &Coordinate, to: &Coordinate) -> Vector2D {
        return Vector2D::new_from_coordinate_from_to(from, to);
    }

    pub fn get_x(&self) -> f64 {
        return self.x;
    }

    pub fn get_y(&self) -> f64 {
        return self.y;
    }

    pub fn get_component(&self, index: usize) -> f64 {
        if index == 0 {
            return self.x;
        }
        return self.y;
    }

    pub fn add(&self, v: &Vector2D) -> Vector2D {
        return Vector2D::create_from_xy(self.x + v.x, self.y + v.y);
    }

    pub fn subtract(&self, v: &Vector2D) -> Vector2D {
        return Vector2D::create_from_xy(self.x - v.x, self.y - v.y);
    }

    /**
     * Multiplies the vector by a scalar value.
     *
     * @param d the value to multiply by
     * @return a new vector with the value v * d
     */
    pub fn multiply(&self, d: f64) -> Vector2D {
        return Vector2D::create_from_xy(self.x * d, self.y * d);
    }

    /**
     * Divides the vector by a scalar value.
     *
     * @param d the value to divide by
     * @return a new vector with the value v / d
     */
    pub fn divide(&self, d: f64) -> Vector2D {
        return Vector2D::create_from_xy(self.x / d, self.y / d);
    }

    pub fn negate(&self) -> Vector2D {
        return Vector2D::create_from_xy(-self.x, -self.y);
    }

    pub fn length(&self) -> f64 {
        return f64::hypot(self.x, self.y);
    }

    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y;
    }

    pub fn normalize(&self) -> Vector2D {
        let length = self.length();
        if length > 0.0 {
            return self.divide(length);
        }
        return Vector2D::create_from_xy(0.0, 0.0);
    }

    pub fn average(&self, v: &Vector2D) -> Vector2D {
        return self.weighted_sum(v, 0.5);
    }

    /**
     * Computes the weighted sum of this vector
     * with another vector,
     * with this vector contributing a fraction
     * of <tt>frac</tt> to the total.
     * <p>
     * In other words,
     * <pre>
     * sum = frac * this + (1 - frac) * v
     * </pre>
     *
     * @param v the vector to sum
     * @param frac the fraction of the total contributed by this vector
     * @return the weighted sum of the two vectors
     */
    pub fn weighted_sum(&self, v: &Vector2D, frac: f64) -> Vector2D {
        return Vector2D::create_from_xy(
            frac * self.x + (1.0 - frac) * v.x,
            frac * self.y + (1.0 - frac) * v.y,
        );
    }

    /**
     * Computes the distance between this vector and another one.
     * @param v a vector
     * @return the distance between the vectors
     */
    pub fn distance(&self, v: &Vector2D) -> f64 {
        let delx = v.x - self.x;
        let dely = v.y - self.y;
        return f64::hypot(delx, dely);
    }

    /**
     * Computes the dot-product of two vectors
     *
     * @param v a vector
     * @return the dot product of the vectors
     */
    pub fn dot(&self, v: &Vector2D) -> f64 {
        return self.x * v.x + self.y * v.y;
    }

    pub fn angle(&self) -> f64 {
        return f64::atan2(self.y, self.x);
    }

    pub fn angle_vector_2d(&self, v: &Vector2D) -> f64 {
        return Angle::diff(v.angle(), self.angle());
    }

    pub fn angle_to(&self, v: &Vector2D) -> f64 {
        let a1 = self.angle();
        let a2 = v.angle();
        let ang_del = a2 - a1;

        // normalize, maintaining orientation
        if ang_del <= -std::f64::consts::PI {
            return ang_del + Angle::PI_TIMES_2;
        }
        if ang_del > std::f64::consts::PI {
            return ang_del - Angle::PI_TIMES_2;
        }
        return ang_del;
    }

    pub fn rotate(&self, angle: f64) -> Vector2D {
        let cos = f64::cos(angle);
        let sin = f64::sin(angle);
        return Vector2D::create_from_xy(self.x * cos - self.y * sin, self.x * sin + self.y * cos);
    }

    /**
     * Rotates a vector by a given number of quarter-circles (i.e. multiples of 90
     * degrees or Pi/2 radians). A positive number rotates counter-clockwise, a
     * negative number rotates clockwise. Under this operation the magnitude of
     * the vector and the absolute values of the ordinates do not change, only
     * their sign and ordinate index.
     *
     * @param numQuarters
     *          the number of quarter-circles to rotate by
     * @return the rotated vector.
     */
    pub fn rotate_by_quarter_circle(&self, num_quarters: i32) -> Option<Vector2D> {
        let mut n_quad = num_quarters % 4;
        if num_quarters < 0 && n_quad != 0 {
            n_quad = n_quad + 4;
        }
        match n_quad {
            0 => return Some(Vector2D::create_from_xy(self.x, self.y)),
            1 => return Some(Vector2D::create_from_xy(-self.y, self.x)),
            2 => return Some(Vector2D::create_from_xy(-self.x, -self.y)),
            3 => return Some(Vector2D::create_from_xy(self.y, -self.x)),
            _ => return None,
        }
    }

    pub fn is_parallel(&self, v: &Vector2D) -> bool {
        return 0 == CGAlgorithmsDD::sign_of_det2x2_f64(self.x, self.y, v.x, v.y);
    }

    pub fn translate(&self, coord: &Coordinate) -> Coordinate {
        return Coordinate::new_xy(self.x + coord.x, self.y + coord.y);
    }

    pub fn to_coordinate(&self) -> Coordinate {
        return Coordinate::new_xy(self.x, self.y);
    }

    /**
     * Gets a string representation of this vector
     *
     * @return a string representing this vector
     */
    pub fn to_string(&self) -> String {
        let str = format!("[{}, {}]", self.x, self.y);
        return str;
    }

    /**
     * Tests if a vector <tt>o</tt> has the same values for the x and y
     * components.
     *
     * @param o
     *          a <tt>Vector2D</tt> with which to do the comparison.
     * @return true if <tt>other</tt> is a <tt>Vector2D</tt> with the same
     *         values for the x and y components.
     */
    pub fn equals(&self, v: &Vector2D) -> bool {
        return self.x == v.x && self.y == v.y;
    }
}
