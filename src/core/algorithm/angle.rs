use crate::core::geom::coordinate::Coordinate;

use super::orientation::Orientation;

/**
 * Utility functions for working with angles.
 * Unless otherwise noted, methods in this class express angles in radians.
 */

pub struct Angle {}

impl Angle {
    /**
     * The value of 2*Pi
     */
    pub const PI_TIMES_2: f64 = 2.0 * std::f64::consts::PI;
    /**
     * The value of Pi/2
     */
    pub const PI_OVER_2: f64 = std::f64::consts::PI / 2.0;
    /**
     * The value of Pi/4
     */
    pub const PI_OVER_4: f64 = std::f64::consts::PI / 4.0;

    /** Constant representing counterclockwise orientation */
    pub const COUNTERCLOCKWISE: i32 = Orientation::COUNTERCLOCKWISE;

    /** Constant representing clockwise orientation */
    pub const CLOCKWISE: i32 = Orientation::CLOCKWISE;

    /** Constant representing no orientation */
    pub const NONE: i32 = Orientation::COLLINEAR;

    /**
     * Converts from radians to degrees.
     * @param radians an angle in radians
     * @return the angle in degrees
     */
    pub fn to_degrees(radians: f64) -> f64 {
        return (radians * 180.) / (std::f64::consts::PI);
    }

    /**
     * Converts from degrees to radians.
     *
     * @param angleDegrees an angle in degrees
     * @return the angle in radians
     */
    pub fn to_radians(angle_degrees: f64) -> f64 {
        return (angle_degrees * std::f64::consts::PI) / 180.0;
    }

    /**
     * Returns the angle of the vector from p0 to p1,
     * relative to the positive X-axis.
     * The angle is normalized to be in the range [ -Pi, Pi ].
     *
     * @param p0 the initial point of the vector
     * @param p1 the terminal point of the vector
     * @return the normalized angle (in radians) that p0-p1 makes with the positive x-axis.
     */
    pub fn angle_coordinates(p0: &Coordinate, p1: &Coordinate) -> f64 {
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        return f64::atan2(dy, dx);
    }

    /**
     * Returns the angle of the vector from (0,0) to p,
     * relative to the positive X-axis.
     * The angle is normalized to be in the range ( -Pi, Pi ].
     *
     * @param p the terminal point of the vector
     * @return the normalized angle (in radians) that p makes with the positive x-axis.
     */
    pub fn angle_coordinate(p: &Coordinate) -> f64 {
        return f64::atan2(p.y, p.x);
    }

    /**
     * Tests whether the angle between p0-p1-p2 is acute.
     * An angle is acute if it is less than 90 degrees.
     * <p>
     * Note: this implementation is not precise (deterministic) for angles very close to 90 degrees.
     *
     * @param p0 an endpoint of the angle
     * @param p1 the base of the angle
     * @param p2 the other endpoint of the angle
     * @return true if the angle is acute
     */
    pub fn is_acute(p0: &Coordinate, p1: &Coordinate, p2: &Coordinate) -> bool {
        // relies on fact that A dot B is positive if A ang B is acute
        let dx0 = p0.x - p1.x;
        let dy0 = p0.y - p1.y;
        let dx1 = p2.x - p1.x;
        let dy1 = p2.y - p1.y;
        let dotprod = dx0 * dx1 + dy0 * dy1;
        return dotprod > 0.;
    }

    /**
     * Tests whether the angle between p0-p1-p2 is obtuse.
     * An angle is obtuse if it is greater than 90 degrees.
     * <p>
     * Note: this implementation is not precise (deterministic) for angles very close to 90 degrees.
     *
     * @param p0 an endpoint of the angle
     * @param p1 the base of the angle
     * @param p2 the other endpoint of the angle
     * @return true if the angle is obtuse
     */
    pub fn is_obtuse(p0: &Coordinate, p1: &Coordinate, p2: &Coordinate) -> bool {
        // relies on fact that A dot B is negative if A ang B is obtuse
        let dx0 = p0.x - p1.x;
        let dy0 = p0.y - p1.y;
        let dx1 = p2.x - p1.x;
        let dy1 = p2.y - p1.y;
        let dotprod = dx0 * dx1 + dy0 * dy1;
        return dotprod < 0.;
    }

    /**
     * Returns the unoriented smallest angle between two vectors.
     * The computed angle will be in the range [0, Pi).
     *
     * @param tip1 the tip of one vector
     * @param tail the tail of each vector
     * @param tip2 the tip of the other vector
     * @return the angle between tail-tip1 and tail-tip2
     */
    pub fn angle_between(tip1: &Coordinate, tail: &Coordinate, tip2: &Coordinate) -> f64 {
        let a1 = Angle::angle_coordinates(tail, tip1);
        let a2 = Angle::angle_coordinates(tail, tip2);

        return Angle::diff(a1, a2);
    }

    /**
     * Returns the oriented smallest angle between two vectors.
     * The computed angle will be in the range (-Pi, Pi].
     * A positive result corresponds to a counterclockwise
     * (CCW) rotation
     * from v1 to v2;
     * a negative result corresponds to a clockwise (CW) rotation;
     * a zero result corresponds to no rotation.
     *
     * @param tip1 the tip of v1
     * @param tail the tail of each vector
     * @param tip2 the tip of v2
     * @return the angle between v1 and v2, relative to v1
     */
    pub fn angle_between_oriented(tip1: &Coordinate, tail: &Coordinate, tip2: &Coordinate) -> f64 {
        let a1 = Angle::angle_coordinates(tail, tip1);
        let a2 = Angle::angle_coordinates(tail, tip2);
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

    /**
     * Computes the angle of the unoriented bisector
     * of the smallest angle between two vectors.
     * The computed angle will be in the range (-Pi, Pi].
     *
     * @param tip1 the tip of v1
     * @param tail the tail of each vector
     * @param tip2 the tip of v2
     * @return the angle of the bisector between v1 and v2
     */
    pub fn bisector(tip1: &Coordinate, tail: &Coordinate, tip2: &Coordinate) -> f64 {
        let ang_del = Angle::angle_between_oriented(tip1, tail, tip2);
        let ang_bi = Angle::angle_coordinates(tail, tip1) + ang_del / 2.;
        return Angle::normalize(ang_bi);
    }

    /**
     * Computes the interior angle between two segments of a ring. The ring is
     * assumed to be oriented in a clockwise direction. The computed angle will be
     * in the range [0, 2Pi]
     *
     * @param p0
     *          a point of the ring
     * @param p1
     *          the next point of the ring
     * @param p2
     *          the next point of the ring
     * @return the interior angle based at {@code p1}
     */
    pub fn interior_angle(p0: &Coordinate, p1: &Coordinate, p2: &Coordinate) -> f64 {
        let angle_prev = Angle::angle_coordinates(p1, p0);
        let angle_next = Angle::angle_coordinates(p1, p2);
        return Angle::normalize_positive(angle_next - angle_prev);
    }

    /**
     * Returns whether an angle must turn clockwise or counterclockwise
     * to overlap another angle.
     *
     * @param ang1 an angle (in radians)
     * @param ang2 an angle (in radians)
     * @return whether a1 must turn CLOCKWISE, COUNTERCLOCKWISE or NONE to
     * overlap a2.
     */
    pub fn get_turn(ang1: f64, ang2: f64) -> i32 {
        let crossproduct = f64::sin(ang2 - ang1);

        if crossproduct > 0. {
            return Angle::COUNTERCLOCKWISE;
        }
        if crossproduct < 0. {
            return Angle::CLOCKWISE;
        }
        return Angle::NONE;
    }

    /**
     * Computes the normalized value of an angle, which is the
     * equivalent angle in the range ( -Pi, Pi ].
     *
     * @param angle the angle to normalize
     * @return an equivalent angle in the range (-Pi, Pi]
     */
    pub fn normalize(angle: f64) -> f64 {
        let mut normalized_angle = angle;
        while normalized_angle > std::f64::consts::PI {
            normalized_angle -= Angle::PI_TIMES_2;
        }
        while normalized_angle <= -std::f64::consts::PI {
            normalized_angle += Angle::PI_TIMES_2;
        }
        return normalized_angle;
    }

    /**
     * Computes the normalized positive value of an angle, which is the
     * equivalent angle in the range [ 0, 2*Pi ).
     * E.g.:
     * <ul>
     * <li>normalizePositive(0.0) = 0.0
     * <li>normalizePositive(-PI) = PI
     * <li>normalizePositive(-2PI) = 0.0
     * <li>normalizePositive(-3PI) = PI
     * <li>normalizePositive(-4PI) = 0
     * <li>normalizePositive(PI) = PI
     * <li>normalizePositive(2PI) = 0.0
     * <li>normalizePositive(3PI) = PI
     * <li>normalizePositive(4PI) = 0.0
     * </ul>
     *
     * @param angle the angle to normalize, in radians
     * @return an equivalent positive angle
     */
    pub fn normalize_positive(angle: f64) -> f64 {
        let mut normalized_pos_angle = angle;
        if normalized_pos_angle < 0.0 {
            while normalized_pos_angle < 0.0 {
                normalized_pos_angle += Angle::PI_TIMES_2;
            }
            // in case round-off error bumps the value over
            if normalized_pos_angle >= Angle::PI_TIMES_2 {
                normalized_pos_angle = 0.0;
            }
        } else {
            while normalized_pos_angle >= Angle::PI_TIMES_2 {
                normalized_pos_angle -= Angle::PI_TIMES_2;
            }
            // in case round-off error bumps the value under
            if normalized_pos_angle < 0.0 {
                normalized_pos_angle = 0.0;
            }
        }
        return normalized_pos_angle;
    }

    /**
     * Computes the unoriented smallest difference between two angles.
     * The angles are assumed to be normalized to the range [-Pi, Pi].
     * The result will be in the range [0, Pi].
     *
     * @param ang1 the angle of one vector (in [-Pi, Pi] )
     * @param ang2 the angle of the other vector (in range [-Pi, Pi] )
     * @return the angle (in radians) between the two vectors (in range [0, Pi] )
     */
    pub fn diff(ang1: f64, ang2: f64) -> f64 {
        let mut del_angle: f64;

        if ang1 < ang2 {
            del_angle = ang2 - ang1;
        } else {
            del_angle = ang1 - ang2;
        }

        if del_angle > std::f64::consts::PI {
            del_angle = Angle::PI_TIMES_2 - del_angle;
        }

        return del_angle;
    }

    /**
     * Computes sin of an angle, snapping near-zero values to zero.
     *
     * @param ang the input angle (in radians)
     * @return the result of the trigonometric function
     */
    pub fn sin_snap(ang: f64) -> f64 {
        let res = f64::sin(ang);
        if f64::abs(res) < 5e-16 {
            return 0.0;
        }
        return res;
    }

    /**
     * Computes cos of an angle, snapping near-zero values to zero.
     *
     * @param ang the input angle (in radians)
     * @return the result of the trigonometric function
     */
    pub fn cos_snap(ang: f64) -> f64 {
        let res = f64::cos(ang);
        if f64::abs(res) < 5e-16 {
            return 0.0;
        }
        return res;
    }

    /**
     * Projects a point by a given angle and distance.
     *
     * @param p the point to project
     * @param angle the angle at which to project
     * @param dist the distance to project
     * @return the projected point
     */
    pub fn project(p: &Coordinate, angle: f64, dist: f64) -> Coordinate {
        let x = p.get_x() + dist * Angle::cos_snap(angle);
        let y = p.get_y() + dist * Angle::sin_snap(angle);
        return Coordinate::new_xy(x, y);
    }
}
