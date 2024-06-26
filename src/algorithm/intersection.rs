use crate::geom::coordinate::Coordinate;

use super::{cg_algorithms_dd::CGAlgorithmsDD, distance::Distance, orientation::Orientation};

/**
 * Functions to compute intersection points between lines and line segments.
 * <p>
 * In general it is not possible to compute
 * the intersection point of two lines exactly, due to numerical roundoff.
 * This is particularly true when the lines are nearly parallel.
 * These routines uses numerical conditioning on the input values
 * to ensure that the computed value is very close to the correct value.
 * <p>
 * The Z-ordinate is ignored, and not populated.
 *
 * @author Martin Davis
 *
 */

pub struct Intersection {}

impl Intersection {
    /**
     * Computes the intersection point of two lines.
     * If the lines are parallel or collinear this case is detected
     * and <code>null</code> is returned.
     *
     * @param p1 an endpoint of line 1
     * @param p2 an endpoint of line 1
     * @param q1 an endpoint of line 2
     * @param q2 an endpoint of line 2
     * @return the intersection point between the lines, if there is one,
     * or null if the lines are parallel or collinear
     *
     * @see CGAlgorithmsDD#intersection(Coordinate, Coordinate, Coordinate, Coordinate)
     */
    pub fn intersection(
        p1: &Coordinate,
        p2: &Coordinate,
        q1: &Coordinate,
        q2: &Coordinate,
    ) -> Option<Coordinate> {
        return CGAlgorithmsDD::intersection(p1, p2, q1, q2);
        //-- this is less robust
        //return intersectionFP(p1, p2, q1, q2);
    }

    /**
     * Compute intersection of two lines, using a floating-point algorithm.
     * This is less accurate than {@link CGAlgorithmsDD#intersection(Coordinate, Coordinate, Coordinate, Coordinate)}.
     * It has caused spatial predicate failures in some cases.
     * This is kept for testing purposes.
     *
     * @param p1 an endpoint of line 1
     * @param p2 an endpoint of line 1
     * @param q1 an endpoint of line 2
     * @param q2 an endpoint of line 2
     * @return the intersection point between the lines, if there is one,
     * or null if the lines are parallel or collinear
     */
    pub fn intersection_fp(
        p1: &Coordinate,
        p2: &Coordinate,
        q1: &Coordinate,
        q2: &Coordinate,
    ) -> Option<Coordinate> {
        // compute midpoint of "kernel envelope"
        let mut min_x0 = p2.x;
        if p1.x < p2.x {
            min_x0 = p1.x;
        }
        let mut min_y0 = p2.y;
        if p1.y < p2.y {
            min_y0 = p1.y;
        }
        let mut max_x0 = p2.x;
        if p1.x > p2.x {
            max_x0 = p1.x;
        }
        let mut max_y0 = p2.y;
        if p1.y > p2.y {
            max_y0 = p1.y
        }

        let mut min_x1 = q2.x;
        if q1.x < q2.x {
            min_x1 = q1.x;
        }
        let mut min_y1 = q2.y;
        if q1.y < q2.y {
            min_y1 = q1.y;
        }
        let mut max_x1 = q2.x;
        if q1.x > q2.x {
            max_x1 = q1.x;
        }
        let mut max_y1 = q2.y;
        if q1.y > q2.y {
            max_y1 = q1.y;
        }

        let mut int_min_x = min_x1;
        if min_x0 > min_x1 {
            int_min_x = min_x0;
        }
        let mut int_max_x = max_x1;
        if max_x0 < max_x1 {
            int_max_x = max_x0;
        }
        let mut int_min_y = min_y1;
        if min_y0 > min_y1 {
            int_min_y = min_y0;
        }
        let mut int_max_y = max_y1;
        if max_y0 < max_y1 {
            int_max_y = max_y0;
        }

        let midx = (int_min_x + int_max_x) / 2.0;
        let midy = (int_min_y + int_max_y) / 2.0;

        // condition ordinate values by subtracting midpoint
        let p1x = p1.x - midx;
        let p1y = p1.y - midy;
        let p2x = p2.x - midx;
        let p2y = p2.y - midy;
        let q1x = q1.x - midx;
        let q1y = q1.y - midy;
        let q2x = q2.x - midx;
        let q2y = q2.y - midy;

        // unrolled computation using homogeneous coordinates eqn
        let px = p1y - p2y;
        let py = p2x - p1x;
        let pw = p1x * p2y - p2x * p1y;

        let qx = q1y - q2y;
        let qy = q2x - q1x;
        let qw = q1x * q2y - q2x * q1y;

        let x = py * qw - qy * pw;
        let y = qx * pw - px * qw;
        let w = px * qy - qx * py;

        let x_int = x / w;
        let y_int = y / w;

        // check for parallel lines
        if (f64::is_nan(x_int))
            || (f64::is_infinite(x_int) || f64::is_nan(y_int))
            || (f64::is_infinite(y_int))
        {
            return None;
        }
        // de-condition intersection point
        return Some(Coordinate::new_xy(x_int + midx, y_int + midy));
    }

    /**
     * Computes the intersection point of a line and a line segment (if any).
     * There will be no intersection point if:
     * <ul>
     * <li>the segment does not intersect the line
     * <li>the line or the segment are degenerate (have zero length)
     * </ul>
     * If the segment is collinear with the line the first segment endpoint is returned.
     *
     * @param line1 a point on the line
     * @param line2 a point on the line
     * @param seg1 an endpoint of the line segment
     * @param seg2 an endpoint of the line segment
     * @return the intersection point, or null if it is not possible to find an intersection
     */
    pub fn line_segment(
        line1: &Coordinate,
        line2: &Coordinate,
        seg1: &Coordinate,
        seg2: &Coordinate,
    ) -> Option<Coordinate> {
        let orient_s1 = Orientation::index(line1, line2, seg1);
        if orient_s1 == 0 {
            return Some(Coordinate::from_coordinate(seg1));
        }

        let orient_s2 = Orientation::index(line1, line2, seg2);
        if orient_s2 == 0 {
            return Some(Coordinate::from_coordinate(seg2));
        }

        // If segment lies completely on one side of the line, it does not intersect
        if (orient_s1 > 0 && orient_s2 > 0) || (orient_s1 < 0 && orient_s2 < 0) {
            return None;
        }

        // The segment intersects the line.
        // The full line-line intersection is used to compute the intersection point.
        let int_pt = Intersection::intersection(line1, line2, seg1, seg2);
        if int_pt.is_some() {
            return int_pt;
        }

        // Due to robustness failure it is possible the intersection computation will return null.
        // In this case choose the closest point
        let dist1 = Distance::point_to_line_perpendicular(seg1, line1, line2);
        let dist2 = Distance::point_to_line_perpendicular(seg2, line1, line2);
        if dist1 < dist2 {
            return Some(Coordinate::from_coordinate(seg1));
        }
        return Some(Coordinate::from_coordinate(seg2));
    }
}
