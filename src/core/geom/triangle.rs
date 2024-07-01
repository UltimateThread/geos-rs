use crate::core::{
    algorithm::{angle::Angle, hcoordinate::HCoordinate, orientation::Orientation},
    math::dd::DD,
};

use super::coordinate::Coordinate;

/**
 * Represents a planar triangle, and provides methods for calculating various
 * properties of triangles.
 *
 * @version 1.7
 */

pub struct Triangle {
    /**
     * The coordinates of the vertices of the triangle
     */
    p0: Coordinate,
    p1: Coordinate,
    p2: Coordinate,
}

impl Triangle {
    /**
     * Creates a new triangle with the given vertices.
     *
     * @param p0
     *          a vertex
     * @param p1
     *          a vertex
     * @param p2
     *          a vertex
     */
    pub fn new(p0: &Coordinate, p1: &Coordinate, p2: &Coordinate) -> Self {
        Self {
            p0: Coordinate::from_coordinate(p0),
            p1: Coordinate::from_coordinate(p1),
            p2: Coordinate::from_coordinate(p2),
        }
    }

    /**
     * Tests whether a triangle is acute. A triangle is acute if all interior
     * angles are acute. This is a strict test - right triangles will return
     * <tt>false</tt>. A triangle which is not acute is either right or obtuse.
     * <p>
     * Note: this implementation is not robust for angles very close to 90
     * degrees.
     *
     * @param a a vertex of the triangle
     * @param b a vertex of the triangle
     * @param c a vertex of the triangle
     * @return true if the triangle is acute
     */
    pub fn is_acute_coordinates(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> bool {
        if !Angle::is_acute(a, b, c) {
            return false;
        }
        if !Angle::is_acute(b, c, a) {
            return false;
        }
        if !Angle::is_acute(c, a, b) {
            return false;
        }
        return true;
    }

    /**
     * Tests whether a triangle is oriented counter-clockwise.
     *
     * @param a a vertex of the triangle
     * @param b a vertex of the triangle
     * @param c a vertex of the triangle
     * @return true if the triangle orientation is counter-clockwise
     */
    pub fn is_ccw_coordinates(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> bool {
        return Orientation::COUNTERCLOCKWISE == Orientation::index(a, b, c);
    }

    /**
     * Tests whether a triangle intersects a point.
     *
     * @param a a vertex of the triangle
     * @param b a vertex of the triangle
     * @param c a vertex of the triangle
     * @param p the point to test
     * @return true if the triangle intersects the point
     */
    pub fn intersects_coordinates(
        a: &Coordinate,
        b: &Coordinate,
        c: &Coordinate,
        p: &Coordinate,
    ) -> bool {
        let mut exterior_index = Orientation::COUNTERCLOCKWISE;
        if Triangle::is_ccw_coordinates(a, b, c) {
            exterior_index = Orientation::CLOCKWISE;
        }
        if exterior_index == Orientation::index(a, b, p) {
            return false;
        }
        if exterior_index == Orientation::index(b, c, p) {
            return false;
        }
        if exterior_index == Orientation::index(c, a, p) {
            return false;
        }
        return true;
    }

    /**
     * Computes the line which is the perpendicular bisector of the line segment
     * a-b.
     *
     * @param a
     *          a point
     * @param b
     *          another point
     * @return the perpendicular bisector, as an HCoordinate
     */
    pub fn perpendicular_bisector_coordinates(a: &Coordinate, b: &Coordinate) -> HCoordinate {
        // returns the perpendicular bisector of the line segment ab
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let l1 = HCoordinate::new_with_xyw(a.x + dx / 2.0, a.y + dy / 2.0, 1.0);
        let l2 = HCoordinate::new_with_xyw(a.x - dy + dx / 2.0, a.y + dx + dy / 2.0, 1.0);
        return HCoordinate::new_with_hcoordinates(&l1, &l2);
    }

    /**
     * Computes the radius of the circumcircle of a triangle.
     * <p>
     * Formula is as per https://math.stackexchange.com/a/3610959
     *
     * @param a a vertex of the triangle
     * @param b a vertex of the triangle
     * @param c a vertex of the triangle
     * @return the circumradius of the triangle
     */
    pub fn circumradius_coordinates(ca: &Coordinate, cb: &Coordinate, cc: &Coordinate) -> f64 {
        let a = ca.distance(cb);
        let b = cb.distance(cc);
        let c = cc.distance(ca);
        let area = Triangle::area_coordinates(ca, cb, cc);
        if area == 0.0 {
            return f64::INFINITY;
        }
        return (a * b * c) / (4. * area);
    }

    /**
     * Computes the circumcentre of a triangle. The circumcentre is the centre of
     * the circumcircle, the smallest circle which encloses the triangle. It is
     * also the common intersection point of the perpendicular bisectors of the
     * sides of the triangle, and is the only point which has equal distance to
     * all three vertices of the triangle.
     *
     * @param a
     *          a vertex of the triangle
     * @param b
     *          a vertex of the triangle
     * @param c
     *          a vertex of the triangle
     * @return the circumcentre of the triangle
     */
    /*
     * // original non-robust algorithm public static Coordinate
     * circumcentre(Coordinate a, Coordinate b, Coordinate c) { // compute the
     * perpendicular bisector of chord ab HCoordinate cab =
     * perpendicularBisector(a, b); // compute the perpendicular bisector of chord
     * bc HCoordinate cbc = perpendicularBisector(b, c); // compute the
     * intersection of the bisectors (circle radii) HCoordinate hcc = new
     * HCoordinate(cab, cbc); Coordinate cc = null; try { cc = new
     * Coordinate(hcc.getX(), hcc.getY()); } catch (NotRepresentableException ex)
     * { // MD - not sure what we can do to prevent this (robustness problem) //
     * Idea - can we condition which edges we choose? throw new
     * IllegalStateException(ex.getMessage()); }
     *
     * //System.out.println("Acc = " + a.distance(cc) + ", Bcc = " +
     * b.distance(cc) + ", Ccc = " + c.distance(cc) );
     *
     * return cc; }
     */

    /**
     * Computes the circumcentre of a triangle. The circumcentre is the centre of
     * the circumcircle, the smallest circle which encloses the triangle. It is
     * also the common intersection point of the perpendicular bisectors of the
     * sides of the triangle, and is the only point which has equal distance to
     * all three vertices of the triangle.
     * <p>
     * The circumcentre does not necessarily lie within the triangle. For example,
     * the circumcentre of an obtuse isosceles triangle lies outside the triangle.
     * <p>
     * This method uses an algorithm due to J.R.Shewchuk which uses normalization
     * to the origin to improve the accuracy of computation. (See <i>Lecture Notes
     * on Geometric Robustness</i>, Jonathan Richard Shewchuk, 1999).
     *
     * @param a
     *          a vertex of the triangle
     * @param b
     *          a vertex of the triangle
     * @param c
     *          a vertex of the triangle
     * @return the circumcentre of the triangle
     */
    pub fn circumcentre_coordinates(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> Coordinate {
        let cx = c.x;
        let cy = c.y;
        let ax = a.x - cx;
        let ay = a.y - cy;
        let bx = b.x - cx;
        let by = b.y - cy;

        let denom = 2. * Triangle::det_f64(ax, ay, bx, by);
        let numx = Triangle::det_f64(ay, ax * ax + ay * ay, by, bx * bx + by * by);
        let numy = Triangle::det_f64(ax, ax * ax + ay * ay, bx, bx * bx + by * by);

        let ccx = cx - numx / denom;
        let ccy = cy + numy / denom;

        return Coordinate::new_xy(ccx, ccy);
    }

    /**
     * Computes the circumcentre of a triangle. The circumcentre is the centre of
     * the circumcircle, the smallest circle which encloses the triangle. It is
     * also the common intersection point of the perpendicular bisectors of the
     * sides of the triangle, and is the only point which has equal distance to
     * all three vertices of the triangle.
     * <p>
     * The circumcentre does not necessarily lie within the triangle. For example,
     * the circumcentre of an obtuse isosceles triangle lies outside the triangle.
     * <p>
     * This method uses {@link DD} extended-precision arithmetic to
     * provide more accurate results than {@link #circumcentre(Coordinate, Coordinate, Coordinate)}
     *
     * @param a
     *          a vertex of the triangle
     * @param b
     *          a vertex of the triangle
     * @param c
     *          a vertex of the triangle
     * @return the circumcentre of the triangle
     */
    pub fn circumcentre_dd_coordinates(
        a: &Coordinate,
        b: &Coordinate,
        c: &Coordinate,
    ) -> Coordinate {
        let ax = DD::value_of_f64(a.x).subtract_f64(c.x);
        let ay = DD::value_of_f64(a.y).subtract_f64(c.y);
        let bx = DD::value_of_f64(b.x).subtract_f64(c.x);
        let by = DD::value_of_f64(b.y).subtract_f64(c.y);

        let denom = DD::determinant_xy_dd(&ax, &ay, &bx, &by).multiply_f64(2.);
        let asqr = ax.sqr().add_dd(&ay.sqr());
        let bsqr = bx.sqr().add_dd(&by.sqr());
        let numx = DD::determinant_xy_dd(&ay, &asqr, &by, &bsqr);
        let numy = DD::determinant_xy_dd(&ax, &asqr, &bx, &bsqr);

        let ccx = DD::value_of_f64(c.x)
            .subtract_dd(&numx.divide_dd(&denom))
            .double_value();
        let ccy = DD::value_of_f64(c.y)
            .add_dd(&numy.divide_dd(&denom))
            .double_value();

        return Coordinate::new_xy(ccx, ccy);
    }

    /**
     * Computes the determinant of a 2x2 matrix. Uses standard double-precision
     * arithmetic, so is susceptible to round-off error.
     *
     * @param m00
     *          the [0,0] entry of the matrix
     * @param m01
     *          the [0,1] entry of the matrix
     * @param m10
     *          the [1,0] entry of the matrix
     * @param m11
     *          the [1,1] entry of the matrix
     * @return the determinant
     */
    pub fn det_f64(m00: f64, m01: f64, m10: f64, m11: f64) -> f64 {
        return m00 * m11 - m01 * m10;
    }

    /**
     * Computes the incentre of a triangle. The <i>inCentre</i> of a triangle is
     * the point which is equidistant from the sides of the triangle. It is also
     * the point at which the bisectors of the triangle's angles meet. It is the
     * centre of the triangle's <i>incircle</i>, which is the unique circle that
     * is tangent to each of the triangle's three sides.
     * <p>
     * The incentre always lies within the triangle.
     *
     * @param a
     *          a vertex of the triangle
     * @param b
     *          a vertex of the triangle
     * @param c
     *          a vertex of the triangle
     * @return the point which is the incentre of the triangle
     */
    pub fn in_centre_coordinates(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> Coordinate {
        // the lengths of the sides, labelled by their opposite vertex
        let len0 = b.distance(c);
        let len1 = a.distance(c);
        let len2 = a.distance(b);
        let circum = len0 + len1 + len2;

        let in_centre_x = (len0 * a.x + len1 * b.x + len2 * c.x) / circum;
        let in_centre_y = (len0 * a.y + len1 * b.y + len2 * c.y) / circum;
        return Coordinate::new_xy(in_centre_x, in_centre_y);
    }

    /**
     * Computes the centroid (centre of mass) of a triangle. This is also the
     * point at which the triangle's three medians intersect (a triangle median is
     * the segment from a vertex of the triangle to the midpoint of the opposite
     * side). The centroid divides each median in a ratio of 2:1.
     * <p>
     * The centroid always lies within the triangle.
     *
     *
     * @param a
     *          a vertex of the triangle
     * @param b
     *          a vertex of the triangle
     * @param c
     *          a vertex of the triangle
     * @return the centroid of the triangle
     */
    pub fn centroid_coordinates(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> Coordinate {
        let x = (a.x + b.x + c.x) / 3.;
        let y = (a.y + b.y + c.y) / 3.;
        return Coordinate::new_xy(x, y);
    }

    /**
     * Compute the length of the perimeter of a triangle
     *
     * @param a a vertex of the triangle
     * @param b a vertex of the triangle
     * @param c a vertex of the triangle
     * @return the length of the triangle perimeter
     */
    pub fn length_coordinates(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> f64 {
        return a.distance(b) + b.distance(c) + c.distance(a);
    }

    /**
     * Computes the length of the longest side of a triangle
     *
     * @param a
     *          a vertex of the triangle
     * @param b
     *          a vertex of the triangle
     * @param c
     *          a vertex of the triangle
     * @return the length of the longest side of the triangle
     */
    pub fn longest_side_length_coordinates(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> f64 {
        let len_ab = a.distance(b);
        let len_bc = b.distance(c);
        let len_ca = c.distance(a);
        let mut max_len = len_ab;
        if len_bc > max_len {
            max_len = len_bc;
        }
        if len_ca > max_len {
            max_len = len_ca;
        }
        return max_len;
    }

    /**
     * Computes the point at which the bisector of the angle ABC cuts the segment
     * AC.
     *
     * @param a
     *          a vertex of the triangle
     * @param b
     *          a vertex of the triangle
     * @param c
     *          a vertex of the triangle
     * @return the angle bisector cut point
     */
    pub fn angle_bisector_coordinates(
        a: &Coordinate,
        b: &Coordinate,
        c: &Coordinate,
    ) -> Coordinate {
        // Uses the fact that the lengths of the parts of the split segment are
        // proportional to the lengths of the adjacent triangle sides
        let len0 = b.distance(a);
        let len2 = b.distance(c);
        let frac = len0 / (len0 + len2);
        let dx = c.x - a.x;
        let dy = c.y - a.y;

        let split_pt = Coordinate::new_xy(a.x + frac * dx, a.y + frac * dy);
        return split_pt;
    }

    /**
     * Computes the 2D area of a triangle. The area value is always non-negative.
     *
     * @param a
     *          a vertex of the triangle
     * @param b
     *          a vertex of the triangle
     * @param c
     *          a vertex of the triangle
     * @return the area of the triangle
     *
     * @see #signedArea(Coordinate, Coordinate, Coordinate)
     */
    pub fn area_coordinates(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> f64 {
        return f64::abs(((c.x - a.x) * (b.y - a.y) - (b.x - a.x) * (c.y - a.y)) / 2.);
    }

    /**
     * Computes the signed 2D area of a triangle. The area value is positive if
     * the triangle is oriented CW, and negative if it is oriented CCW.
     * <p>
     * The signed area value can be used to determine point orientation, but the
     * implementation in this method is susceptible to round-off errors. Use
     * {@link Orientation#index(Coordinate, Coordinate, Coordinate)}
     * for robust orientation calculation.
     *
     * @param a
     *          a vertex of the triangle
     * @param b
     *          a vertex of the triangle
     * @param c
     *          a vertex of the triangle
     * @return the signed 2D area of the triangle
     *
     * @see Orientation#index(Coordinate, Coordinate, Coordinate)
     */
    pub fn signed_area_coordinates(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> f64 {
        // Uses the formula 1/2 * | u x v | where u,v are the side vectors of the
        // triangle x is the vector cross-product For 2D vectors, this formula
        // simplifies to the expression below
        return ((c.x - a.x) * (b.y - a.y) - (b.x - a.x) * (c.y - a.y)) / 2.;
    }

    /**
     * Computes the 3D area of a triangle. The value computed is always
     * non-negative.
     *
     * @param a
     *          a vertex of the triangle
     * @param b
     *          a vertex of the triangle
     * @param c
     *          a vertex of the triangle
     * @return the 3D area of the triangle
     */
    pub fn area_3d_coordinates(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> f64 {
        // Uses the formula 1/2 * | u x v | where u,v are the side vectors of the
        // triangle x is the vector cross-product
        // side vectors u and v
        let ux = b.x - a.x;
        let uy = b.y - a.y;
        let uz = b.get_z() - a.get_z();

        let vx = c.x - a.x;
        let vy = c.y - a.y;
        let vz = c.get_z() - a.get_z();

        // cross-product = u x v
        let crossx = uy * vz - uz * vy;
        let crossy = uz * vx - ux * vz;
        let crossz = ux * vy - uy * vx;

        // tri area = 1/2 * | u x v |
        let abs_sq = crossx * crossx + crossy * crossy + crossz * crossz;
        let area_3d = f64::sqrt(abs_sq) / 2.;

        return area_3d;
    }

    /**
     * Computes the Z-value (elevation) of an XY point on a three-dimensional
     * plane defined by a triangle whose vertices have Z-values. The defining
     * triangle must not be degenerate (in other words, the triangle must enclose
     * a non-zero area), and must not be parallel to the Z-axis.
     * <p>
     * This method can be used to interpolate the Z-value of a point inside a
     * triangle (for example, of a TIN facet with elevations on the vertices).
     *
     * @param p
     *          the point to compute the Z-value of
     * @param v0
     *          a vertex of a triangle, with a Z ordinate
     * @param v1
     *          a vertex of a triangle, with a Z ordinate
     * @param v2
     *          a vertex of a triangle, with a Z ordinate
     * @return the computed Z-value (elevation) of the point
     */
    pub fn interpolate_z_coordinates(
        p: &Coordinate,
        v0: &Coordinate,
        v1: &Coordinate,
        v2: &Coordinate,
    ) -> f64 {
        let x0 = v0.x;
        let y0 = v0.y;
        let a = v1.x - x0;
        let b = v2.x - x0;
        let c = v1.y - y0;
        let d = v2.y - y0;
        let det = a * d - b * c;
        let dx = p.x - x0;
        let dy = p.y - y0;
        let t = (d * dx - b * dy) / det;
        let u = (-c * dx + a * dy) / det;
        let z = v0.get_z() + t * (v1.get_z() - v0.get_z()) + u * (v2.get_z() - v0.get_z());
        return z;
    }

    /**
     * Computes the incentre of this triangle. The <i>incentre</i> of a triangle
     * is the point which is equidistant from the sides of the triangle. It is
     * also the point at which the bisectors of the triangle's angles meet. It is
     * the centre of the triangle's <i>incircle</i>, which is the unique circle
     * that is tangent to each of the triangle's three sides.
     *
     * @return the point which is the inCentre of this triangle
     */
    pub fn in_centre(&self) -> Coordinate {
        return Triangle::in_centre_coordinates(&self.p0, &self.p1, &self.p2);
    }

    /**
     * Tests whether this triangle is acute. A triangle is acute if all interior
     * angles are acute. This is a strict test - right triangles will return
     * <tt>false</tt>. A triangle which is not acute is either right or obtuse.
     * <p>
     * Note: this implementation is not robust for angles very close to 90
     * degrees.
     *
     * @return true if this triangle is acute
     */
    pub fn is_acute(&self) -> bool {
        return Triangle::is_acute_coordinates(&self.p0, &self.p1, &self.p2);
    }

    /**
     * Tests whether this triangle is oriented counter-clockwise.
     *
     * @return true if the triangle orientation is counter-clockwise
     */
    pub fn is_ccw(&self) -> bool {
        return Triangle::is_ccw_coordinates(&self.p0, &self.p1, &self.p2);
    }

    /**
     * Computes the circumcentre of this triangle. The circumcentre is the centre
     * of the circumcircle, the smallest circle which passes through all the triangle vertices.
     * It is also the common intersection point of the perpendicular bisectors of the
     * sides of the triangle, and is the only point which has equal distance to
     * all three vertices of the triangle.
     * <p>
     * The circumcentre does not necessarily lie within the triangle.
     * <p>
     * This method uses an algorithm due to J.R.Shewchuk which uses normalization
     * to the origin to improve the accuracy of computation. (See <i>Lecture Notes
     * on Geometric Robustness</i>, Jonathan Richard Shewchuk, 1999).
     *
     * @return the circumcentre of this triangle
     */
    pub fn circumcentre(&self) -> Coordinate {
        return Triangle::circumcentre_coordinates(&self.p0, &self.p1, &self.p2);
    }

    /**
     * Computes the radius of the circumcircle of a triangle.
     *
     * @return the triangle circumradius
     */
    pub fn circumradius(&self) -> f64 {
        return Triangle::circumradius_coordinates(&self.p0, &self.p1, &self.p2);
    }

    /**
     * Computes the centroid (centre of mass) of this triangle. This is also the
     * point at which the triangle's three medians intersect (a triangle median is
     * the segment from a vertex of the triangle to the midpoint of the opposite
     * side). The centroid divides each median in a ratio of 2:1.
     * <p>
     * The centroid always lies within the triangle.
     *
     * @return the centroid of this triangle
     */
    pub fn centroid(&self) -> Coordinate {
        return Triangle::centroid_coordinates(&self.p0, &self.p1, &self.p2);
    }

    /**
     * Computes the length of the perimeter of this triangle.
     *
     * @return the length of the perimeter
     */
    pub fn length(&self) -> f64 {
        return Triangle::length_coordinates(&self.p0, &self.p1, &self.p2);
    }

    /**
     * Computes the length of the longest side of this triangle
     *
     * @return the length of the longest side of this triangle
     */
    pub fn longest_side_length(&self) -> f64 {
        return Triangle::longest_side_length_coordinates(&self.p0, &self.p1, &self.p2);
    }

    /**
     * Computes the 2D area of this triangle. The area value is always
     * non-negative.
     *
     * @return the area of this triangle
     *
     * @see #signedArea()
     */
    pub fn area(&self) -> f64 {
        return Triangle::area_coordinates(&self.p0, &self.p1, &self.p2);
    }

    /**
     * Computes the signed 2D area of this triangle. The area value is positive if
     * the triangle is oriented CW, and negative if it is oriented CCW.
     * <p>
     * The signed area value can be used to determine point orientation, but the
     * implementation in this method is susceptible to round-off errors. Use
     * {@link Orientation#index(Coordinate, Coordinate, Coordinate)}
     * for robust orientation calculation.
     *
     * @return the signed 2D area of this triangle
     *
     * @see Orientation#index(Coordinate, Coordinate, Coordinate)
     */
    pub fn signed_area(&self) -> f64 {
        return Triangle::signed_area_coordinates(&self.p0, &self.p1, &self.p2);
    }

    /**
     * Computes the 3D area of this triangle. The value computed is always
     * non-negative.
     *
     * @return the 3D area of this triangle
     */
    pub fn area_3d(&self) -> f64 {
        return Triangle::area_3d_coordinates(&self.p0, &self.p1, &self.p2);
    }

    /**
     * Computes the Z-value (elevation) of an XY point on a three-dimensional
     * plane defined by this triangle (whose vertices must have Z-values). This
     * triangle must not be degenerate (in other words, the triangle must enclose
     * a non-zero area), and must not be parallel to the Z-axis.
     * <p>
     * This method can be used to interpolate the Z-value of a point inside this
     * triangle (for example, of a TIN facet with elevations on the vertices).
     *
     * @param p
     *          the point to compute the Z-value of
     * @return the computed Z-value (elevation) of the point
     */
    pub fn interpolate_z(&self, p: &Coordinate) -> f64 {
        return Triangle::interpolate_z_coordinates(p, &self.p0, &self.p1, &self.p2);
    }
}
