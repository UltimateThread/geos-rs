use crate::core::geom::{coordinate::Coordinate, envelope::Envelope, precision_model::PrecisionModel};

use super::{distance::Distance, intersection::Intersection, orientation::Orientation};

/**
 * A robust version of {@link LineIntersector}.
 *
 * @version 1.7
 */

pub struct RobustLineIntersector {
    result: i32,
    input_lines: [[Coordinate; 2]; 2],
    int_pt: [Coordinate; 2],
    // The indexes of the endpoints of the intersection lines, in order along
    // the corresponding line
    int_line_index: [[i32; 2]; 2],
    is_proper: bool,
    pa: Coordinate,
    pb: Coordinate,
    // If makePrecise is true, computed intersection coordinates will be made precise
    // using Coordinate#makePrecise
    precision_model: Option<PrecisionModel>,
}

impl RobustLineIntersector {
    /**
     * Indicates that line segments do not intersect
     */
    pub const NO_INTERSECTION: i32 = 0;

    /**
     * Indicates that line segments intersect in a single point
     */
    pub const POINT_INTERSECTION: i32 = 1;

    /**
     * Indicates that line segments intersect in a line segment
     */
    pub const COLLINEAR_INTERSECTION: i32 = 2;

    pub fn default() -> Self {
        let input_lines = [[Coordinate::default(); 2]; 2];
        let int_line_index = [[0; 2]; 2];
        let coords = [Coordinate::default(); 2];
        // alias the intersection points for ease of reference
        let pa = coords[0];
        let pb = coords[1];

        let new = Self {
            result: 0,
            input_lines,
            int_pt: coords,
            int_line_index,
            pa,
            pb,
            is_proper: false,
            precision_model: None,
        };

        new
    }

    /**
     * Tests whether the input geometries intersect.
     *
     * @return true if the input geometries intersect
     */
    pub fn has_intersection(&self) -> bool {
        return self.result != RobustLineIntersector::NO_INTERSECTION;
    }

    /**
     * Returns the intIndex'th intersection point
     *
     * @param intIndex is 0 or 1
     *
     * @return the intIndex'th intersection point
     */
    pub fn get_intersection(&self, int_index: usize) -> Coordinate {
        return self.int_pt[int_index];
    }

    pub fn compute_intersection_3(&mut self, p: &Coordinate, p1: &Coordinate, p2: &Coordinate) {
        self.is_proper = false;
        // do between check first, since it is faster than the orientation test
        if Envelope::intersects_3(p1, p2, p) {
            if (Orientation::index(p1, p2, p) == 0) && (Orientation::index(p2, p1, p) == 0) {
                self.is_proper = true;
                if p.equals_2d(p1) || p.equals_2d(p2) {
                    self.is_proper = false;
                }
                self.result = RobustLineIntersector::POINT_INTERSECTION;
                return;
            }
        }
        self.result = RobustLineIntersector::NO_INTERSECTION;
    }

    /**
     * Computes the intersection of the lines p1-p2 and p3-p4.
     * This function computes both the boolean value of the hasIntersection test
     * and the (approximate) value of the intersection point itself (if there is one).
     */
    pub fn compute_intersection_4(
        &mut self,
        p1: &Coordinate,
        p2: &Coordinate,
        p3: &Coordinate,
        p4: &Coordinate,
    ) {
        self.input_lines[0][0] = Coordinate::from_coordinate(p1);
        self.input_lines[0][1] = Coordinate::from_coordinate(p2);
        self.input_lines[1][0] = Coordinate::from_coordinate(p3);
        self.input_lines[1][1] = Coordinate::from_coordinate(p4);
        self.result = self.compute_intersect(p1, p2, p3, p4);
    }

    pub fn compute_intersect(
        &mut self,
        p1: &Coordinate,
        p2: &Coordinate,
        q1: &Coordinate,
        q2: &Coordinate,
    ) -> i32 {
        self.is_proper = false;

        // first try a fast test to see if the envelopes of the lines intersect
        if !Envelope::intersects_4(p1, p2, q1, q2) {
            return RobustLineIntersector::NO_INTERSECTION;
        }

        // for each endpoint, compute which side of the other segment it lies
        // if both endpoints lie on the same side of the other segment,
        // the segments do not intersect
        let pq1 = Orientation::index(p1, p2, q1);
        let pq2 = Orientation::index(p1, p2, q2);

        if (pq1 > 0 && pq2 > 0) || (pq1 < 0 && pq2 < 0) {
            return RobustLineIntersector::NO_INTERSECTION;
        }

        let qp1 = Orientation::index(q1, q2, p1);
        let qp2 = Orientation::index(q1, q2, p2);

        if (qp1 > 0 && qp2 > 0) || (qp1 < 0 && qp2 < 0) {
            return RobustLineIntersector::NO_INTERSECTION;
        }
        // Intersection is collinear if each endpoint lies on the other line.
        let collinear = pq1 == 0 && pq2 == 0 && qp1 == 0 && qp2 == 0;
        if collinear {
            return self.compute_collinear_intersection(p1, p2, q1, q2);
        }

        // At this point we know that there is a single intersection point
        // (since the lines are not collinear).

        //  Check if the intersection is an endpoint. If it is, copy the endpoint as
        //  the intersection point. Copying the point rather than computing it
        //  ensures the point has the exact value, which is important for
        //  robustness. It is sufficient to simply check for an endpoint which is on
        //  the other line, since at this point we know that the inputLines must
        //  intersect.
        let mut p: Coordinate = Coordinate::default();
        let mut z = f64::NAN;
        if pq1 == 0 || pq2 == 0 || qp1 == 0 || qp2 == 0 {
            self.is_proper = false;

            // Check for two equal endpoints.
            // This is done explicitly rather than by the orientation tests
            // below in order to improve robustness.
            //
            // [An example where the orientation tests fail to be consistent is
            // the following (where the true intersection is at the shared endpoint
            // POINT (19.850257749638203 46.29709338043669)
            //
            // LINESTRING ( 19.850257749638203 46.29709338043669, 20.31970698357233 46.76654261437082 )
            // and
            // LINESTRING ( -48.51001596420236 -22.063180333403878, 19.850257749638203 46.29709338043669 )
            //
            // which used to produce the INCORRECT result: (20.31970698357233, 46.76654261437082, NaN)
            if p1.equals_2d(q1) {
                p = Coordinate::from_coordinate(p1);
                z = self.get_z(p1, q1);
            } else if p1.equals_2d(q2) {
                p = Coordinate::from_coordinate(p1);
                z = self.get_z(p1, q2);
            } else if p2.equals_2d(q1) {
                p = Coordinate::from_coordinate(p2);
                z = self.get_z(p2, q1);
            } else if p2.equals_2d(q2) {
                p = Coordinate::from_coordinate(p2);
                z = self.get_z(p2, q2);
            } else if pq1 == 0 {
                // Now check to see if any endpoint lies on the interior of the other segment.
                p = Coordinate::from_coordinate(q1);
                z = self.get_z_or_interpolate(q1, p1, p2);
            } else if pq2 == 0 {
                p = Coordinate::from_coordinate(q2);
                z = self.get_z_or_interpolate(q2, p1, p2);
            } else if qp1 == 0 {
                p = Coordinate::from_coordinate(p1);
                z = self.get_z_or_interpolate(p1, q1, q2);
            } else if qp2 == 0 {
                p = Coordinate::from_coordinate(p2);
                z = self.get_z_or_interpolate(p2, q1, q2);
            }
        } else {
            self.is_proper = true;
            p = self.intersection(p1, p2, q1, q2);
            z = self.z_interpolate_5(&p, p1, p2, q1, q2);
        }
        self.int_pt[0] = self.copy_with_z(&p, z);
        return RobustLineIntersector::POINT_INTERSECTION;
    }

    pub fn compute_collinear_intersection(
        &mut self,
        p1: &Coordinate,
        p2: &Coordinate,
        q1: &Coordinate,
        q2: &Coordinate,
    ) -> i32 {
        let q1_in_p = Envelope::intersects_3(p1, p2, q1);
        let q2_in_p = Envelope::intersects_3(p1, p2, q2);
        let p1_in_q = Envelope::intersects_3(q1, q2, p1);
        let p2_in_q = Envelope::intersects_3(q1, q2, p2);

        if q1_in_p && q2_in_p {
            self.int_pt[0] = self.copy_with_z_interpolate(q1, p1, p2);
            self.int_pt[1] = self.copy_with_z_interpolate(q2, p1, p2);
            return RobustLineIntersector::COLLINEAR_INTERSECTION;
        }
        if p1_in_q && p2_in_q {
            self.int_pt[0] = self.copy_with_z_interpolate(p1, q1, q2);
            self.int_pt[1] = self.copy_with_z_interpolate(p2, q1, q2);
            return RobustLineIntersector::COLLINEAR_INTERSECTION;
        }
        if q1_in_p && p1_in_q {
            // if pts are equal Z is chosen arbitrarily
            self.int_pt[0] = self.copy_with_z_interpolate(q1, p1, p2);
            self.int_pt[1] = self.copy_with_z_interpolate(p1, q1, q2);
            if q1.equals_2d(p1) && !q2_in_p && !p2_in_q {
                return RobustLineIntersector::POINT_INTERSECTION;
            } else {
                return RobustLineIntersector::COLLINEAR_INTERSECTION;
            }
        }
        if q1_in_p && p2_in_q {
            // if pts are equal Z is chosen arbitrarily
            self.int_pt[0] = self.copy_with_z_interpolate(q1, p1, p2);
            self.int_pt[1] = self.copy_with_z_interpolate(p2, q1, q2);
            if q1.equals_2d(p2) && !q2_in_p && !p1_in_q {
                return RobustLineIntersector::POINT_INTERSECTION;
            } else {
                return RobustLineIntersector::COLLINEAR_INTERSECTION;
            }
        }
        if q2_in_p && p1_in_q {
            // if pts are equal Z is chosen arbitrarily
            self.int_pt[0] = self.copy_with_z_interpolate(q2, p1, p2);
            self.int_pt[1] = self.copy_with_z_interpolate(p1, q1, q2);
            if q2.equals_2d(p1) && !q1_in_p && !p2_in_q {
                return RobustLineIntersector::POINT_INTERSECTION;
            } else {
                return RobustLineIntersector::COLLINEAR_INTERSECTION;
            }
        }
        if q2_in_p && p2_in_q {
            // if pts are equal Z is chosen arbitrarily
            self.int_pt[0] = self.copy_with_z_interpolate(q2, p1, p2);
            self.int_pt[1] = self.copy_with_z_interpolate(p2, q1, q2);
            if q2.equals_2d(p2) && !q1_in_p && !p1_in_q {
                return RobustLineIntersector::POINT_INTERSECTION;
            } else {
                return RobustLineIntersector::COLLINEAR_INTERSECTION;
            }
        }
        return RobustLineIntersector::NO_INTERSECTION;
    }

    pub fn copy_with_z_interpolate(
        &mut self,
        p: &Coordinate,
        p1: &Coordinate,
        p2: &Coordinate,
    ) -> Coordinate {
        let z = self.get_z_or_interpolate(p, p1, p2);
        return self.copy_with_z(p, z);
    }

    pub fn copy_with_z(&mut self, p: &Coordinate, z: f64) -> Coordinate {
        let mut p_copy = Coordinate::from_coordinate(p);
        if !f64::is_nan(z) {
            p_copy.set_z(z);
        }
        return p_copy;
    }

    /**
     * This method computes the actual value of the intersection point.
     * It is rounded to the precision model if being used.
     */
    pub fn intersection(
        &mut self,
        p1: &Coordinate,
        p2: &Coordinate,
        q1: &Coordinate,
        q2: &Coordinate,
    ) -> Coordinate {
        let mut int_pt = self.intersection_safe(p1, p2, q1, q2);

        if !self.is_in_segment_envelopes(&int_pt) {
            // compute a safer result
            // copy the coordinate, since it may be rounded later
            int_pt = Coordinate::from_coordinate(&self.nearest_endpoint(p1, p2, q1, q2));
        }
        if self.precision_model.is_some() {
            self.precision_model
                .unwrap()
                .make_precise_coordinate(&mut int_pt);
        }
        return int_pt;
    }

    /**
     * Computes a segment intersection.
     * Round-off error can cause the raw computation to fail,
     * (usually due to the segments being approximately parallel).
     * If this happens, a reasonable approximation is computed instead.
     *
     * @param p1 a segment endpoint
     * @param p2 a segment endpoint
     * @param q1 a segment endpoint
     * @param q2 a segment endpoint
     * @return the computed intersection point
     */
    pub fn intersection_safe(
        &mut self,
        p1: &Coordinate,
        p2: &Coordinate,
        q1: &Coordinate,
        q2: &Coordinate,
    ) -> Coordinate {
        let mut int_pt = Intersection::intersection(p1, p2, q1, q2);
        if int_pt.is_none() {
            int_pt = Some(self.nearest_endpoint(p1, p2, q1, q2));
        }
        return int_pt.unwrap();
    }

    /**
     * Tests whether a point lies in the envelopes of both input segments.
     * A correctly computed intersection point should return <code>true</code>
     * for this test.
     * Since this test is for debugging purposes only, no attempt is
     * made to optimize the envelope test.
     *
     * @return <code>true</code> if the input point lies within both input segment envelopes
     */
    pub fn is_in_segment_envelopes(&mut self, int_pt: &Coordinate) -> bool {
        let env0 = Envelope::new_coordinates(&self.input_lines[0][0], &self.input_lines[0][1]);
        let env1 = Envelope::new_coordinates(&self.input_lines[1][0], &self.input_lines[1][1]);
        return env0.contains_coordinate(int_pt) && env1.contains_coordinate(int_pt);
    }

    /**
     * Finds the endpoint of the segments P and Q which
     * is closest to the other segment.
     * This is a reasonable surrogate for the true
     * intersection points in ill-conditioned cases
     * (e.g. where two segments are nearly coincident,
     * or where the endpoint of one segment lies almost on the other segment).
     * <p>
     * This replaces the older CentralEndpoint heuristic,
     * which chose the wrong endpoint in some cases
     * where the segments had very distinct slopes
     * and one endpoint lay almost on the other segment.
     *
     * @param p1 an endpoint of segment P
     * @param p2 an endpoint of segment P
     * @param q1 an endpoint of segment Q
     * @param q2 an endpoint of segment Q
     * @return the nearest endpoint to the other segment
     */
    pub fn nearest_endpoint(
        &mut self,
        p1: &Coordinate,
        p2: &Coordinate,
        q1: &Coordinate,
        q2: &Coordinate,
    ) -> Coordinate {
        let mut nearest_pt = p1;
        let mut min_dist = Distance::point_to_segment(p1, q1, q2);

        let mut dist = Distance::point_to_segment(p2, q1, q2);
        if dist < min_dist {
            min_dist = dist;
            nearest_pt = p2;
        }
        dist = Distance::point_to_segment(q1, p1, p2);
        if dist < min_dist {
            min_dist = dist;
            nearest_pt = q1;
        }
        dist = Distance::point_to_segment(q2, p1, p2);
        if dist < min_dist {
            nearest_pt = q2;
        }
        return Coordinate::from_coordinate(nearest_pt);
    }

    /**
     * Gets the Z value of the first argument if present,
     * otherwise the value of the second argument.
     *
     * @param p a coordinate, possibly with Z
     * @param q a coordinate, possibly with Z
     * @return the Z value if present
     */
    pub fn get_z(&mut self, p: &Coordinate, q: &Coordinate) -> f64 {
        let mut z = p.get_z();
        if f64::is_nan(z) {
            z = q.get_z(); // may be NaN
        }
        return z;
    }

    /**
     * Gets the Z value of a coordinate if present, or
     * interpolates it from the segment it lies on.
     * If the segment Z values are not fully populate
     * NaN is returned.
     *
     * @param p a coordinate, possibly with Z
     * @param p1 a segment endpoint, possibly with Z
     * @param p2 a segment endpoint, possibly with Z
     * @return the extracted or interpolated Z value (may be NaN)
     */
    pub fn get_z_or_interpolate(
        &mut self,
        p: &Coordinate,
        p1: &Coordinate,
        p2: &Coordinate,
    ) -> f64 {
        let z = p.get_z();
        if !f64::is_nan(z) {
            return z;
        }
        return self.z_interpolate_3(p, p1, p2); // may be NaN
    }

    /**
     * Interpolates a Z value for a point along
     * a line segment between two points.
     * The Z value of the interpolation point (if any) is ignored.
     * If either segment point is missing Z,
     * returns NaN.
     *
     * @param p a coordinate
     * @param p1 a segment endpoint, possibly with Z
     * @param p2 a segment endpoint, possibly with Z
     * @return the interpolated Z value (may be NaN)
     */
    pub fn z_interpolate_3(&mut self, p: &Coordinate, p1: &Coordinate, p2: &Coordinate) -> f64 {
        let p1z = p1.get_z();
        let p2z = p2.get_z();
        if f64::is_nan(p1z) {
            return p2z; // may be NaN
        }
        if f64::is_nan(p2z) {
            return p1z; // may be NaN
        }
        if p.equals_2d(p1) {
            return p1z; // not NaN
        }
        if p.equals_2d(p2) {
            return p2z; // not NaN
        }
        let dz = p2z - p1z;
        if dz == 0.0 {
            return p1z;
        }
        // interpolate Z from distance of p along p1-p2
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        // seg has non-zero length since p1 < p < p2
        let seglen = dx * dx + dy * dy;
        let xoff = p.x - p1.x;
        let yoff = p.y - p1.y;
        let plen = xoff * xoff + yoff * yoff;
        let frac = f64::sqrt(plen / seglen);
        let zoff = dz * frac;
        let z_interpolated = p1z + zoff;
        return z_interpolated;
    }

    /**
     * Interpolates a Z value for a point along
     * two line segments and computes their average.
     * The Z value of the interpolation point (if any) is ignored.
     * If one segment point is missing Z that segment is ignored
     * if both segments are missing Z, returns NaN.
     *
     * @param p a coordinate
     * @param p1 a segment endpoint, possibly with Z
     * @param p2 a segment endpoint, possibly with Z
     * @param q1 a segment endpoint, possibly with Z
     * @param q2 a segment endpoint, possibly with Z
     * @return the averaged interpolated Z value (may be NaN)
     */
    pub fn z_interpolate_5(
        &mut self,
        p: &Coordinate,
        p1: &Coordinate,
        p2: &Coordinate,
        q1: &Coordinate,
        q2: &Coordinate,
    ) -> f64 {
        let zp = self.z_interpolate_3(p, p1, p2);
        let zq = self.z_interpolate_3(p, q1, q2);
        if f64::is_nan(zp) {
            return zq; // may be NaN
        }
        if f64::is_nan(zq) {
            return zp; // may be NaN
        }
        // both Zs have values, so average them
        return (zp + zq) / 2.0;
    }
}
