use crate::core::algorithm::{
    distance::Distance, intersection::Intersection, orientation::Orientation,
    robust_line_intersector::RobustLineIntersector,
};

use super::coordinate::Coordinate;

#[derive(Clone, Copy)]
pub struct LineSegment {
    pub p0: Coordinate,
    pub p1: Coordinate,
}

impl LineSegment {
    pub fn default() -> Self {
        let p0 = Coordinate::default();
        let p1 = Coordinate::default();
        LineSegment::new_from_coordinates(&p0, &p1)
    }

    pub fn new_from_coordinates(p0: &Coordinate, p1: &Coordinate) -> Self {
        Self {
            p0: Coordinate::from_coordinate(p0),
            p1: Coordinate::from_coordinate(p1),
        }
    }

    pub fn new_from_xy(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        let p0 = Coordinate::new_xy(x0, y0);
        let p1 = Coordinate::new_xy(x1, y1);
        LineSegment::new_from_coordinates(&p0, &p1)
    }

    pub fn new_from_line_segment(ls: &LineSegment) -> Self {
        LineSegment::new_from_coordinates(&ls.p0, &ls.p1)
    }

    pub fn get_coordinate(&self, i: usize) -> Coordinate {
        if i == 0 {
            return self.p0;
        }
        return self.p1;
    }

    pub fn set_coordinates_from_line_segment(&mut self, ls: LineSegment) {
        self.set_coordinates(&ls.p0, &ls.p1);
    }

    pub fn set_coordinates(&mut self, p0: &Coordinate, p1: &Coordinate) {
        self.p0.x = p0.x;
        self.p0.y = p0.y;
        self.p1.x = p1.x;
        self.p1.y = p1.y;
    }

    /**
     * Gets the minimum X ordinate.
     * @return the minimum X ordinate
     */
    pub fn min_x(&self) -> f64 {
        return f64::min(self.p0.x, self.p1.x);
    }

    /**
     * Gets the maximum X ordinate.
     * @return the maximum X ordinate
     */
    pub fn max_x(&self) -> f64 {
        return f64::max(self.p0.x, self.p1.x);
    }

    /**
     * Gets the minimum Y ordinate.
     * @return the minimum Y ordinate
     */
    pub fn min_y(&self) -> f64 {
        return f64::min(self.p0.y, self.p1.y);
    }

    /**
     * Gets the maximum Y ordinate.
     * @return the maximum Y ordinate
     */
    pub fn max_y(&self) -> f64 {
        return f64::max(self.p0.y, self.p1.y);
    }

    /**
     * Computes the length of the line segment.
     * @return the length of the line segment
     */
    pub fn get_length(&self) -> f64 {
        return self.p0.distance(&self.p1);
    }

    /**
     * Tests whether the segment is horizontal.
     *
     * @return <code>true</code> if the segment is horizontal
     */
    pub fn is_horizontal(&self) -> bool {
        return self.p0.y == self.p1.y;
    }

    /**
     * Tests whether the segment is vertical.
     *
     * @return <code>true</code> if the segment is vertical
     */
    pub fn is_vertical(&self) -> bool {
        return self.p0.x == self.p1.x;
    }

    /**
     * Determines the orientation of a LineSegment relative to this segment.
     * The concept of orientation is specified as follows:
     * Given two line segments A and L,
     * <ul>
     * <li>A is to the left of a segment L if A lies wholly in the
     * closed half-plane lying to the left of L
     * <li>A is to the right of a segment L if A lies wholly in the
     * closed half-plane lying to the right of L
     * <li>otherwise, A has indeterminate orientation relative to L. This
     * happens if A is collinear with L or if A crosses the line determined by L.
     * </ul>
     *
     * @param seg the LineSegment to compare
     *
     * @return 1 if <code>seg</code> is to the left of this segment
     * @return -1 if <code>seg</code> is to the right of this segment
     * @return 0 if <code>seg</code> is collinear to or crosses this segment
     */
    pub fn orientation_index_line_segment(&self, seg: &LineSegment) -> i32 {
        let orient0 = Orientation::index(&self.p0, &self.p1, &seg.p0);
        let orient1 = Orientation::index(&self.p0, &self.p1, &seg.p1);
        // this handles the case where the points are L or collinear
        if orient0 >= 0 && orient1 >= 0 {
            return i32::max(orient0, orient1);
        }
        // this handles the case where the points are R or collinear
        if orient0 <= 0 && orient1 <= 0 {
            return i32::min(orient0, orient1);
        }
        // points lie on opposite sides ==> indeterminate orientation
        return 0;
    }

    /**
     * Determines the orientation index of a {@link Coordinate} relative to this segment.
     * The orientation index is as defined in {@link Orientation#index(Coordinate, Coordinate, Coordinate)}.
     *
     * @param p the coordinate to compare
     *
     * @return 1 (LEFT) if <code>p</code> is to the left of this segment
     * @return -1 (RIGHT) if <code>p</code> is to the right of this segment
     * @return 0 (COLLINEAR) if <code>p</code> is collinear with this segment
     *
     * @see Orientation#index(Coordinate, Coordinate, Coordinate)
     */
    pub fn orientation_index_coordinate(&self, p: &Coordinate) -> i32 {
        return Orientation::index(&self.p0, &self.p1, p);
    }

    /**
     * Reverses the direction of the line segment.
     */
    pub fn reverse(&mut self) {
        let temp = self.p0;
        self.p0 = self.p1;
        self.p1 = temp;
    }

    /**
     * Puts the line segment into a normalized form.
     * This is useful for using line segments in maps and indexes when
     * topological equality rather than exact equality is desired.
     * A segment in normalized form has the first point smaller
     * than the second (according to the standard ordering on {@link Coordinate}).
     */
    pub fn normalize(&mut self) {
        if self.p1.compare_to(&self.p0) < 0 {
            self.reverse();
        }
    }

    /**
     * Computes the angle that the vector defined by this segment
     * makes with the X-axis.
     * The angle will be in the range [ -PI, PI ] radians.
     *
     * @return the angle this segment makes with the X-axis (in radians)
     */
    pub fn angle(&self) -> f64 {
        return f64::atan2(self.p1.y - self.p0.y, self.p1.x - self.p0.x);
    }

    /**
     * Computes the midpoint of the segment
     *
     * @return the midpoint of the segment
     */
    pub fn mid_point(&self) -> Coordinate {
        return LineSegment::mid_point_coordinates(&self.p0, &self.p1);
    }

    /**
     * Computes the midpoint of a segment
     *
     * @return the midpoint of the segment
     */
    pub fn mid_point_coordinates(p0: &Coordinate, p1: &Coordinate) -> Coordinate {
        return Coordinate::new_xy((p0.x + p1.x) / 2., (p0.y + p1.y) / 2.);
    }

    /**
     * Computes the distance between this line segment and another segment.
     *
     * @return the distance to the other segment
     */
    pub fn distance_line_segment(&self, ls: &LineSegment) -> f64 {
        return Distance::segment_to_segment(&self.p0, &self.p1, &ls.p0, &ls.p1);
    }

    /**
     * Computes the distance between this line segment and a given point.
     *
     * @return the distance from this segment to the given point
     */
    pub fn distance_coordinate(&self, p: &Coordinate) -> f64 {
        return Distance::point_to_segment(p, &self.p0, &self.p1);
    }

    /**
     * Computes the perpendicular distance between the (infinite) line defined
     * by this line segment and a point.
     * If the segment has zero length this returns the distance between
     * the segment and the point.
     *
     * @param p the point to compute the distance to
     * @return the perpendicular distance between the line and point
     */
    pub fn distance_perpendicular(&self, p: &Coordinate) -> f64 {
        if self.p0.equals_2d(&self.p1) {
            return self.p0.distance(p);
        }
        return Distance::point_to_line_perpendicular(p, &self.p0, &self.p1);
    }

    /**
     * Computes the oriented perpendicular distance between the (infinite) line
     * defined by this line segment and a point.
     * The oriented distance is positive if the point on the left of the line,
     * and negative if it is on the right.
     * If the segment has zero length this returns the distance between
     * the segment and the point.
     *
     * @param p the point to compute the distance to
     * @return the oriented perpendicular distance between the line and point
     */
    pub fn distance_perpendicular_oriented(&self, p: &Coordinate) -> f64 {
        if self.p0.equals_2d(&self.p1) {
            return self.p0.distance(p);
        }
        let dist = self.distance_perpendicular(p);
        if self.orientation_index_coordinate(p) < 0 {
            return -dist;
        }
        return dist;
    }

    /**
     * Computes the {@link Coordinate} that lies a given
     * fraction along the line defined by this segment.
     * A fraction of <code>0.0</code> returns the start point of the segment;
     * a fraction of <code>1.0</code> returns the end point of the segment.
     * If the fraction is &lt; 0.0 or &gt; 1.0 the point returned
     * will lie before the start or beyond the end of the segment.
     *
     * @param segmentLengthFraction the fraction of the segment length along the line
     * @return the point at that distance
     */
    pub fn point_along(&self, segment_length_fraction: f64) -> Coordinate {
        let mut coord = Coordinate::default();
        coord.x = self.p0.x + segment_length_fraction * (self.p1.x - self.p0.x);
        coord.y = self.p0.y + segment_length_fraction * (self.p1.y - self.p0.y);
        return coord;
    }

    /**
     * Computes the {@link Coordinate} that lies a given
     * fraction along the line defined by this segment and offset from
     * the segment by a given distance.
     * A fraction of <code>0.0</code> offsets from the start point of the segment;
     * a fraction of <code>1.0</code> offsets from the end point of the segment.
     * The computed point is offset to the left of the line if the offset distance is
     * positive, to the right if negative.
     *
     * @param segmentLengthFraction the fraction of the segment length along the line
     * @param offsetDistance the distance the point is offset from the segment
     *    (positive is to the left, negative is to the right)
     * @return the point at that distance and offset
     *
     * @throws IllegalStateException if the segment has zero length
     */
    pub fn point_along_offset(
        &self,
        segment_length_fraction: f64,
        offset_distance: f64,
    ) -> Coordinate {
        // the point on the segment line
        let segx = self.p0.x + segment_length_fraction * (self.p1.x - self.p0.x);
        let segy = self.p0.y + segment_length_fraction * (self.p1.y - self.p0.y);

        let dx = self.p1.x - self.p0.x;
        let dy = self.p1.y - self.p0.y;
        let len = f64::hypot(dx, dy);
        let mut ux = 0.0;
        let mut uy = 0.0;
        if offset_distance != 0.0 {
            // if (len <= 0.0)
            //   throw new IllegalStateException("Cannot compute offset from zero-length line segment");

            // u is the vector that is the length of the offset, in the direction of the segment
            ux = offset_distance * dx / len;
            uy = offset_distance * dy / len;
        }

        // the offset point is the seg point plus the offset vector rotated 90 degrees CCW
        let offsetx = segx - uy;
        let offsety = segy + ux;

        let mut coord = Coordinate::default();
        coord.set_x(offsetx);
        coord.set_y(offsety);
        return coord;
    }

    /**
     * Computes the Projection Factor for the projection of the point p
     * onto this LineSegment.  The Projection Factor is the constant r
     * by which the vector for this segment must be multiplied to
     * equal the vector for the projection of <tt>p</tt> on the line
     * defined by this segment.
     * <p>
     * The projection factor will lie in the range <tt>(-inf, +inf)</tt>,
     * or be <code>NaN</code> if the line segment has zero length..
     *
     * @param p the point to compute the factor for
     * @return the projection factor for the point
     */
    pub fn projection_factor(&self, p: &Coordinate) -> f64 {
        if p.equals_2d(&self.p0) {
            return 0.0;
        }
        if p.equals_2d(&self.p1) {
            return 1.0;
        }
        // Otherwise, use comp.graphics.algorithms Frequently Asked Questions method
        /*     	      AC dot AB
                   r = ---------
                         ||AB||^2
                r has the following meaning:
                r=0 P = A
                r=1 P = B
                r<0 P is on the backward extension of AB
                r>1 P is on the forward extension of AB
                0<r<1 P is interior to AB
        */
        let dx = self.p1.x - self.p0.x;
        let dy = self.p1.y - self.p0.y;
        let len = dx * dx + dy * dy;

        // handle zero-length segments
        if len <= 0.0 {
            return f64::NAN;
        }

        let r = ((p.x - self.p0.x) * dx + (p.y - self.p0.y) * dy) / len;
        return r;
    }

    /**
     * Computes the fraction of distance (in <tt>[0.0, 1.0]</tt>)
     * that the projection of a point occurs along this line segment.
     * If the point is beyond either ends of the line segment,
     * the closest fractional value (<tt>0.0</tt> or <tt>1.0</tt>) is returned.
     * <p>
     * Essentially, this is the {@link #projectionFactor} clamped to
     * the range <tt>[0.0, 1.0]</tt>.
     * If the segment has zero length, 1.0 is returned.
     *  
     * @param inputPt the point
     * @return the fraction along the line segment the projection of the point occurs
     */
    pub fn segment_fraction(&self, input_pt: &Coordinate) -> f64 {
        let mut seg_frac = self.projection_factor(input_pt);
        if seg_frac < 0.0 {
            seg_frac = 0.0;
        } else if seg_frac > 1.0 || f64::is_nan(seg_frac) {
            seg_frac = 1.0;
        }
        return seg_frac;
    }

    /**
     * Compute the projection of a point onto the line determined
     * by this line segment.
     * <p>
     * Note that the projected point
     * may lie outside the line segment.  If this is the case,
     * the projection factor will lie outside the range [0.0, 1.0].
     */
    pub fn project_coordinate(&self, p: &Coordinate) -> Coordinate {
        if p.equals_2d(&self.p0) || p.equals_2d(&self.p1) {
            return Coordinate::from_coordinate(&p);
        }

        let r = self.projection_factor(p);
        return self.project_coordinate_with_projection_factor(p, r);
    }

    pub fn project_coordinate_with_projection_factor(
        &self,
        p: &Coordinate,
        projection_factor: f64,
    ) -> Coordinate {
        let mut coord = Coordinate::from_coordinate(&p);
        coord.x = self.p0.x + projection_factor * (self.p1.x - self.p0.x);
        coord.y = self.p0.y + projection_factor * (self.p1.y - self.p0.y);
        return coord;
    }

    /**
     * Project a line segment onto this line segment and return the resulting
     * line segment.  The returned line segment will be a subset of
     * the target line line segment.  This subset may be null, if
     * the segments are oriented in such a way that there is no projection.
     * <p>
     * Note that the returned line may have zero length (i.e. the same endpoints).
     * This can happen for instance if the lines are perpendicular to one another.
     *
     * @param seg the line segment to project
     * @return the projected line segment, or <code>null</code> if there is no overlap
     */
    pub fn project_line_segment(&self, seg: &LineSegment) -> Option<LineSegment> {
        let pf0 = self.projection_factor(&seg.p0);
        let pf1 = self.projection_factor(&seg.p1);
        // check if segment projects at all
        if pf0 >= 1.0 && pf1 >= 1.0 {
            return None;
        }
        if pf0 <= 0.0 && pf1 <= 0.0 {
            return None;
        }

        let mut newp0 = self.project_coordinate_with_projection_factor(&seg.p0, pf0);
        if pf0 < 0.0 {
            newp0 = self.p0;
        }
        if pf0 > 1.0 {
            newp0 = self.p1;
        }

        let mut newp1 = self.project_coordinate_with_projection_factor(&seg.p1, pf1);
        if pf1 > 1.0 {
            newp1 = self.p1;
        }
        if pf1 < 0.0 {
            newp1 = self.p0;
        }

        return Some(LineSegment::new_from_coordinates(&newp0, &newp1));
    }

    /**
     * Computes the {@link LineSegment} that is offset from
     * the segment by a given distance.
     * The computed segment is offset to the left of the line if the offset distance is
     * positive, to the right if negative.
     *
     * @param offsetDistance the distance the point is offset from the segment
     *    (positive is to the left, negative is to the right)
     * @return a line segment offset by the specified distance
     *
     * @throws IllegalStateException if the segment has zero length
     */
    pub fn offset(&self, offset_distance: f64) -> LineSegment {
        let offset0 = self.point_along_offset(0., offset_distance);
        let offset1 = self.point_along_offset(1., offset_distance);
        return LineSegment::new_from_coordinates(&offset0, &offset1);
    }

    /**
     * Computes the reflection of a point in the line defined
     * by this line segment.
     *
     * @param p the point to reflect
     * @return the reflected point
     */
    pub fn reflect(&self, p: &Coordinate) -> Coordinate {
        // general line equation
        let a = self.p1.get_y() - self.p0.get_y();
        let b = self.p0.get_x() - self.p1.get_x();
        let c = self.p0.get_y() * (self.p1.get_x() - self.p0.get_x())
            - self.p0.get_x() * (self.p1.get_y() - self.p0.get_y());

        // compute reflected point
        let a2_plus_b2 = a * a + b * b;
        let a2_sub_b2 = a * a - b * b;

        let x = p.get_x();
        let y = p.get_y();
        let rx = (-a2_sub_b2 * x - 2. * a * b * y - 2. * a * c) / a2_plus_b2;
        let ry = (a2_sub_b2 * y - 2. * a * b * x - 2. * b * c) / a2_plus_b2;

        let mut coord = Coordinate::from_coordinate(&p);
        coord.set_x(rx);
        coord.set_y(ry);
        return coord;
    }

    /**
     * Computes the closest point on this line segment to another point.
     * @param p the point to find the closest point to
     * @return a Coordinate which is the closest point on the line segment to the point p
     */
    pub fn closest_point(&self, p: &Coordinate) -> Coordinate {
        let factor = self.projection_factor(p);
        if factor > 0. && factor < 1. {
            return self.project_coordinate_with_projection_factor(p, factor);
        }
        let dist0 = self.p0.distance(p);
        let dist1 = self.p1.distance(p);
        if dist0 < dist1 {
            return self.p0;
        }
        return self.p1;
    }

    /**
     * Computes the closest points on two line segments.
     *
     * @param line the segment to find the closest point to
     * @return a pair of Coordinates which are the closest points on the line segments
     */
    pub fn closest_points(&self, line: &LineSegment) -> Vec<Coordinate> {
        // test for intersection
        let int_pt = self.intersection(line);
        if int_pt.is_some() {
            return vec![Coordinate::new_xy(
                int_pt.unwrap().get_x(),
                int_pt.unwrap().get_y(),
            )];
        }

        //  if no intersection closest pair contains at least one endpoint.
        //  Test each endpoint in turn.
        let mut closest_pt: Vec<Coordinate> = vec![Coordinate::default(); 2];
        let mut min_distance;
        let mut dist;

        let close00 = self.closest_point(&line.p0);
        min_distance = close00.distance(&line.p0);
        closest_pt[0] = close00;
        closest_pt[1] = line.p0;

        let close01 = self.closest_point(&line.p1);
        dist = close01.distance(&line.p1);
        if dist < min_distance {
            min_distance = dist;
            closest_pt[0] = close01;
            closest_pt[1] = line.p1;
        }

        let close10 = line.closest_point(&self.p0);
        dist = close10.distance(&self.p0);
        if dist < min_distance {
            min_distance = dist;
            closest_pt[0] = self.p0;
            closest_pt[1] = close10;
        }

        let close11 = line.closest_point(&self.p1);
        dist = close11.distance(&self.p1);
        if dist < min_distance {
            closest_pt[0] = self.p1;
            closest_pt[1] = close11;
        }

        return closest_pt;
    }

    /**
     * Computes an intersection point between two line segments, if there is one.
     * There may be 0, 1 or many intersection points between two segments.
     * If there are 0, null is returned. If there is 1 or more,
     * exactly one of them is returned
     * (chosen at the discretion of the algorithm).  
     * If more information is required about the details of the intersection,
     * the {@link RobustLineIntersector} class should be used.
     *
     * @param line a line segment
     * @return an intersection point, or <code>null</code> if there is none
     *
     * @see RobustLineIntersector
     */
    pub fn intersection(&self, line: &LineSegment) -> Option<Coordinate> {
        let mut li = RobustLineIntersector::default();
        li.compute_intersection_4(&self.p0, &self.p1, &line.p0, &line.p1);
        if li.has_intersection() {
            return Some(li.get_intersection(0));
        }
        return None;
    }

    /**
     * Computes the intersection point of the lines of infinite extent defined
     * by two line segments (if there is one).
     * There may be 0, 1 or an infinite number of intersection points
     * between two lines.
     * If there is a unique intersection point, it is returned.
     * Otherwise, <tt>null</tt> is returned.
     * If more information is required about the details of the intersection,
     * the {@link RobustLineIntersector} class should be used.
     *
     * @param line a line segment defining an straight line with infinite extent
     * @return an intersection point,
     * or <code>null</code> if there is no point of intersection
     * or an infinite number of intersection points
     *
     * @see RobustLineIntersector
     */
    pub fn line_intersection(&self, line: LineSegment) -> Option<Coordinate> {
        return Intersection::intersection(&self.p0, &self.p1, &line.p0, &line.p1);
    }

    /**
     * Creates a LineString with the same coordinates as this segment
     *
     * @param geomFactory the geometry factory to use
     * @return a LineString with the same geometry as this segment
     */
    // TODO: implement GeometryFactory::createLineString
    // pub fn to_line_string(&self) -> LineString {
    //     let coords: Vec<Coordinate> = vec![
    //         Coordinate::from_coordinate(&self.p0),
    //         Coordinate::from_coordinate(&self.p1),
    //     ];
    //     return GeometryFactory::createLineString(&coords);
    // }

    /**
     *  Returns <code>true</code> if <code>other</code> has the same values for
     *  its points.
     *
     *@param  o  a <code>LineSegment</code> with which to do the comparison.
     *@return        <code>true</code> if <code>other</code> is a <code>LineSegment</code>
     *      with the same values for the x and y ordinates.
     */
    pub fn equals(&self, other: &LineSegment) -> bool {
        return self.p0.equals_2d(&other.p0) && self.p1.equals_2d(&other.p1);
    }

    /**
     *  Compares this object with the specified object for order.
     *  Uses the standard lexicographic ordering for the points in the LineSegment.
     *
     *@param  o  the <code>LineSegment</code> with which this <code>LineSegment</code>
     *      is being compared
     *@return    a negative integer, zero, or a positive integer as this <code>LineSegment</code>
     *      is less than, equal to, or greater than the specified <code>LineSegment</code>
     */
    pub fn compare_to(&self, other: &LineSegment) -> i32 {
        let comp0 = self.p0.compare_to(&other.p0);
        if comp0 != 0 {
            return comp0;
        }
        return self.p1.compare_to(&other.p1);
    }

    /**
     *  Returns <code>true</code> if <code>other</code> is
     *  topologically equal to this LineSegment (e.g. irrespective
     *  of orientation).
     *
     *@param  other  a <code>LineSegment</code> with which to do the comparison.
     *@return        <code>true</code> if <code>other</code> is a <code>LineSegment</code>
     *      with the same values for the x and y ordinates.
     */
    pub fn equals_topo(&self, other: &LineSegment) -> bool {
        return self.p0.equals_2d(&other.p0) && self.p1.equals_2d(&other.p1)
            || self.p0.equals_2d(&other.p1) && self.p1.equals_2d(&other.p0);
    }

    // public String toString()
    // {
    //   return WKTConstants.LINESTRING + " (" +
    //       p0.x + " " + p0.y
    //       + ", " +
    //       p1.x + " " + p1.y + ")";
    // }
}
