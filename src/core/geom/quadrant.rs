use super::coordinate::Coordinate;

/**
 * Utility functions for working with quadrants of the Euclidean plane.
 * <p>
 * Quadrants are referenced and numbered as follows:
 * <pre>
 * 1 - NW | 0 - NE
 * -------+-------
 * 2 - SW | 3 - SE
 * </pre>
 *
 * @version 1.7
 */

#[derive(Clone, Copy)]
pub struct Quadrant {}

impl Quadrant {
    pub const NE: i32 = 0;
    pub const NW: i32 = 1;
    pub const SW: i32 = 2;
    pub const SE: i32 = 3;

    /**
     * Returns the quadrant of a directed line segment (specified as x and y
     * displacements, which cannot both be 0).
     *
     * @throws IllegalArgumentException if the displacements are both 0
     */
    pub fn quadrant_xy(dx: f64, dy: f64) -> i32 {
        if dx == 0.0 && dy == 0.0 {
            return -1;
        }
        if dx >= 0.0 {
            if dy >= 0.0 {
                return Quadrant::NE;
            } else {
                return Quadrant::SE;
            }
        } else {
            if dy >= 0.0 {
                return Quadrant::NW;
            } else {
                return Quadrant::SW;
            }
        }
    }

    /**
     * Returns the quadrant of a directed line segment from p0 to p1.
     *
     * @throws IllegalArgumentException if the points are equal
     */
    pub fn quadrant_coordinates(p0: &Coordinate, p1: &Coordinate) -> i32 {
        if p1.x == p0.x && p1.y == p0.y {
            return -1;
        }

        if p1.x >= p0.x {
            if p1.y >= p0.y {
                return Quadrant::NE;
            } else {
                return Quadrant::SE;
            }
        } else {
            if p1.y >= p0.y {
                return Quadrant::NW;
            } else {
                return Quadrant::SW;
            }
        }
    }

    /**
     * Returns true if the quadrants are 1 and 3, or 2 and 4
     */
    pub fn is_opposite(quad1: i32, quad2: i32) -> bool {
        if quad1 == quad2 {
            return false;
        }
        let diff = (quad1 - quad2 + 4) % 4;
        // if quadrants are not adjacent, they are opposite
        if diff == 2 {
            return true;
        }
        return false;
    }

    /**
     * Returns the right-hand quadrant of the halfplane defined by the two quadrants,
     * or -1 if the quadrants are opposite, or the quadrant if they are identical.
     */
    pub fn common_half_plane(quad1: i32, quad2: i32) -> i32 {
        // if quadrants are the same they do not determine a unique common halfplane.
        // Simply return one of the two possibilities
        if quad1 == quad2 {
            return quad1;
        }
        let diff = (quad1 - quad2 + 4) % 4;
        // if quadrants are not adjacent, they do not share a common halfplane
        if diff == 2 {
            return -1;
        }
        //
        let mut min = quad2;
        if quad1 < quad2 {
            min = quad1;
        }
        let mut max = quad2;
        if quad1 > quad2 {
            max = quad1;
        }
        // for this one case, the righthand plane is NOT the minimum index;
        if min == 0 && max == 3 {
            return 3;
        }
        // in general, the halfplane index is the minimum of the two adjacent quadrants
        return min;
    }

    /**
     * Returns whether the given quadrant lies within the given halfplane (specified
     * by its right-hand quadrant).
     */
    pub fn is_in_half_plane(quad: i32, half_plane: i32) -> bool {
        if half_plane == Quadrant::SE {
            return quad == Quadrant::SE || quad == Quadrant::SW;
        }
        return quad == half_plane || quad == half_plane + 1;
    }

    /**
     * Returns true if the given quadrant is 0 or 1.
     */
    pub fn is_northern(quad: i32) -> bool {
        return quad == Quadrant::NE || quad == Quadrant::NW;
    }
}
