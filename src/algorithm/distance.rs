use crate::{
    geom::{coordinate::Coordinate, envelope::Envelope},
    util::math_util::MathUtil,
};

/**
 * Functions to compute distance between basic geometric structures.
 *
 * @author Martin Davis
 *
 */

pub struct Distance {}

impl Distance {
    /**
     * Computes the distance from a line segment AB to a line segment CD
     *
     * Note: NON-ROBUST!
     *
     * @param A
     *          a point of one line
     * @param B
     *          the second point of (must be different to A)
     * @param C
     *          one point of the line
     * @param D
     *          another point of the line (must be different to A)
     */
    pub fn segment_to_segment(
        a: &Coordinate,
        b: &Coordinate,
        c: &Coordinate,
        d: &Coordinate,
    ) -> f64 {
        // check for zero-length segments
        if a.equals_2d(b) {
            return Distance::point_to_segment(a, c, d);
        }
        if c.equals_2d(d) {
            return Distance::point_to_segment(d, a, b);
        }

        // AB and CD are line segments
        /*
         * from comp.graphics.algo
         *
         * Solving the above for r and s yields
         *
         *     (Ay-Cy)(Dx-Cx)-(Ax-Cx)(Dy-Cy)
         * r = ----------------------------- (eqn 1)
         *     (Bx-Ax)(Dy-Cy)-(By-Ay)(Dx-Cx)
         *
         *     (Ay-Cy)(Bx-Ax)-(Ax-Cx)(By-Ay)
         * s = ----------------------------- (eqn 2)
         *     (Bx-Ax)(Dy-Cy)-(By-Ay)(Dx-Cx)
         *
         * Let P be the position vector of the
         * intersection point, then
         *   P=A+r(B-A) or
         *   Px=Ax+r(Bx-Ax)
         *   Py=Ay+r(By-Ay)
         * By examining the values of r & s, you can also determine some other limiting
         * conditions:
         *   If 0<=r<=1 & 0<=s<=1, intersection exists
         *      r<0 or r>1 or s<0 or s>1 line segments do not intersect
         *   If the denominator in eqn 1 is zero, AB & CD are parallel
         *   If the numerator in eqn 1 is also zero, AB & CD are collinear.
         */

        let mut no_intersection = false;
        if !Envelope::intersects_4(a, b, c, d) {
            no_intersection = true;
        } else {
            let denom = (b.x - a.x) * (d.y - c.y) - (b.y - a.y) * (d.x - c.x);

            if denom == 0. {
                no_intersection = true;
            } else {
                let r_num = (a.y - c.y) * (d.x - c.x) - (a.x - c.x) * (d.y - c.y);
                let s_num = (a.y - c.y) * (b.x - a.x) - (a.x - c.x) * (b.y - a.y);

                let s = s_num / denom;
                let r = r_num / denom;

                if (r < 0.) || (r > 1.) || (s < 0.) || (s > 1.) {
                    no_intersection = true;
                }
            }
        }
        if no_intersection {
            return MathUtil::min(
                Distance::point_to_segment(a, c, d),
                Distance::point_to_segment(b, c, d),
                Distance::point_to_segment(c, a, b),
                Distance::point_to_segment(d, a, b),
            );
        }
        // segments intersect
        return 0.0;
    }

    /**
     * Computes the distance from a point to a sequence of line segments.
     *
     * @param p
     *          a point
     * @param line
     *          a sequence of contiguous line segments defined by their vertices
     * @return the minimum distance between the point and the line segments
     */
    pub fn point_to_segment_string(p: &Coordinate, line: &Vec<Coordinate>) -> f64 {
        if line.len() == 0 {
            return f64::NAN;
        }

        // this handles the case of length = 1
        let mut min_distance = p.distance(&line[0]);
        for i in 0..(line.len() - 1) {
            let dist = Distance::point_to_segment(p, &line[i], &line[i + 1]);
            if dist < min_distance {
                min_distance = dist;
            }
        }
        return min_distance;
    }

    /**
     * Computes the distance from a point p to a line segment AB
     *
     * Note: NON-ROBUST!
     *
     * @param p
     *          the point to compute the distance for
     * @param A
     *          one point of the line
     * @param B
     *          another point of the line (must be different to A)
     * @return the distance from p to line segment AB
     */
    pub fn point_to_segment(p: &Coordinate, a: &Coordinate, b: &Coordinate) -> f64 {
        // if start = end, then just compute distance to one of the endpoints
        if a.x == b.x && a.y == b.y {
            return p.distance(a);
        }

        // otherwise use comp.graphics.algorithms Frequently Asked Questions method
        /*
         * (1) r = AC dot AB
         *         ---------
         *         ||AB||^2
         *
         * r has the following meaning:
         *   r=0 P = A
         *   r=1 P = B
         *   r<0 P is on the backward extension of AB
         *   r>1 P is on the forward extension of AB
         *   0<r<1 P is interior to AB
         */

        let len2 = (b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y);
        let r = ((p.x - a.x) * (b.x - a.x) + (p.y - a.y) * (b.y - a.y)) / len2;

        if r <= 0.0 {
            return p.distance(a);
        }
        if r >= 1.0 {
            return p.distance(b);
        }

        /*
         * (2) s = (Ay-Cy)(Bx-Ax)-(Ax-Cx)(By-Ay)
         *         -----------------------------
         *                    L^2
         *
         * Then the distance from C to P = |s|*L.
         *
         * This is the same calculation as {@link #distancePointLinePerpendicular}.
         * Unrolled here for performance.
         */
        let s = ((a.y - p.y) * (b.x - a.x) - (a.x - p.x) * (b.y - a.y)) / len2;
        return f64::abs(s) * f64::sqrt(len2);
    }

    /**
     * Computes the perpendicular distance from a point p to the (infinite) line
     * containing the points AB
     *
     * @param p
     *          the point to compute the distance for
     * @param A
     *          one point of the line
     * @param B
     *          another point of the line (must be different to A)
     * @return the distance from p to line AB
     */
    pub fn point_to_line_perpendicular(p: &Coordinate, a: &Coordinate, b: &Coordinate) -> f64 {
        // use comp.graphics.algorithms Frequently Asked Questions method
        /*
         * (2) s = (Ay-Cy)(Bx-Ax)-(Ax-Cx)(By-Ay)
         *         -----------------------------
         *                    L^2
         *
         * Then the distance from C to P = |s|*L.
         */
        let len2 = (b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y);
        let s = ((a.y - p.y) * (b.x - a.x) - (a.x - p.x) * (b.y - a.y)) / len2;

        return f64::abs(s) * f64::sqrt(len2);
    }

    pub fn point_to_line_perpendicular_signed(
        p: &Coordinate,
        a: &Coordinate,
        b: &Coordinate,
    ) -> f64 {
        // use comp.graphics.algorithms Frequently Asked Questions method
        /*
         * (2) s = (Ay-Cy)(Bx-Ax)-(Ax-Cx)(By-Ay)
         *         -----------------------------
         *                    L^2
         *
         * Then the distance from C to P = |s|*L.
         */
        let len2 = (b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y);
        let s = ((a.y - p.y) * (b.x - a.x) - (a.x - p.x) * (b.y - a.y)) / len2;

        return s * f64::sqrt(len2);
    }
}
