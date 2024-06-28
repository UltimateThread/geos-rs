use crate::core::geom::{
    coordinate::Coordinate, implementation::{coordinate_array_sequence::CoordinateArraySequence, packed_coordinate_sequence_double::PackedCoordinateSequenceDouble},
    location::Location,
};

use super::orientation::Orientation;

/**
 * Counts the number of segments crossed by a horizontal ray extending to the right
 * from a given point, in an incremental fashion.
 * This can be used to determine whether a point lies in a {@link Polygonal} geometry.
 * The class determines the situation where the point lies exactly on a segment.
 * When being used for Point-In-Polygon determination, this case allows short-circuiting
 * the evaluation.
 * <p>
 * This class handles polygonal geometries with any number of shells and holes.
 * The orientation of the shell and hole rings is unimportant.
 * In order to compute a correct location for a given polygonal geometry,
 * it is essential that <b>all</b> segments are counted which
 * <ul>
 * <li>touch the ray
 * <li>lie in in any ring which may contain the point
 * </ul>
 * The only exception is when the point-on-segment situation is detected, in which
 * case no further processing is required.
 * The implication of the above rule is that segments
 * which can be a priori determined to <i>not</i> touch the ray
 * (i.e. by a test of their bounding box or Y-extent)
 * do not need to be counted.  This allows for optimization by indexing.
 * <p>
 * This implementation uses the extended-precision orientation test,
 * to provide maximum robustness and consistency within
 * other algorithms.
 *
 * @author Martin Davis
 *
 */

pub struct RayCrossingCounter {
    p: Coordinate,
    crossing_count: i32,
    // true if the test point lies on an input segment
    is_point_on_segment: bool,
}

impl RayCrossingCounter {
    pub fn new_with_coordinate(p: &Coordinate) -> Self {
        Self {
            p: Coordinate::from_coordinate(p),
            crossing_count: 0,
            is_point_on_segment: false,
        }
    }

    /**
     * Determines the {@link Location} of a point in a ring.
     * This method is an exemplar of how to use this class.
     *
     * @param p the point to test
     * @param ring an array of Coordinates forming a ring
     * @return the location of the point in the ring
     */
    pub fn locate_point_in_ring_vec(p: &Coordinate, ring: &Vec<Coordinate>) -> i32 {
        let mut counter = RayCrossingCounter::new_with_coordinate(p);

        for i in 1..ring.len() {
            let p1 = ring[i];
            let p2 = ring[i - 1];
            counter.count_segment(&p1, &p2);
            if counter.is_on_segment() {
                return counter.get_location();
            }
        }
        return counter.get_location();
    }

    /**
     * Determines the {@link Location} of a point in a ring.
     *
     * @param p
     *            the point to test
     * @param ring
     *            a coordinate sequence forming a ring
     * @return the location of the point in the ring
     */
    pub fn locate_point_in_ring_coordinate_array_sequence(
        p: &Coordinate,
        ring: &CoordinateArraySequence,
    ) -> i32 {
        let mut counter = RayCrossingCounter::new_with_coordinate(p);

        let mut p1 = Coordinate::default();
        let mut p2 = Coordinate::default();
        for i in 1..ring.size() {
            //ring.getCoordinate(i, p1); // throws exception if ring contains M ordinate
            p1.x = ring.get_ordinate(i, CoordinateArraySequence::X);
            p1.y = ring.get_ordinate(i, CoordinateArraySequence::Y);
            //ring.getCoordinate(i - 1, p2); // throws exception if ring contains M ordinate
            p2.x = ring.get_ordinate(i - 1, CoordinateArraySequence::X);
            p2.y = ring.get_ordinate(i - 1, CoordinateArraySequence::Y);
            counter.count_segment(&p1, &p2);
            if counter.is_on_segment() {
                return counter.get_location();
            }
        }
        return counter.get_location();
    }

    /**
     * Determines the {@link Location} of a point in a ring.
     *
     * @param p
     *            the point to test
     * @param ring
     *            a coordinate sequence forming a ring
     * @return the location of the point in the ring
     */
    pub fn locate_point_in_ring_packed_coordinate_sequence(
        p: &Coordinate,
        ring: &PackedCoordinateSequenceDouble,
    ) -> i32 {
        let mut counter = RayCrossingCounter::new_with_coordinate(p);

        let mut p1 = Coordinate::default();
        let mut p2 = Coordinate::default();
        for i in 1..ring.size() {
            //ring.getCoordinate(i, p1); // throws exception if ring contains M ordinate
            p1.x = ring.get_ordinate(i, CoordinateArraySequence::X);
            p1.y = ring.get_ordinate(i, CoordinateArraySequence::Y);
            //ring.getCoordinate(i - 1, p2); // throws exception if ring contains M ordinate
            p2.x = ring.get_ordinate(i - 1, CoordinateArraySequence::X);
            p2.y = ring.get_ordinate(i - 1, CoordinateArraySequence::Y);
            counter.count_segment(&p1, &p2);
            if counter.is_on_segment() {
                return counter.get_location();
            }
        }
        return counter.get_location();
    }

    /**
     * Counts a segment
     *
     * @param p1 an endpoint of the segment
     * @param p2 another endpoint of the segment
     */
    pub fn count_segment(&mut self, p1: &Coordinate, p2: &Coordinate) {
        // For each segment, check if it crosses
        // a horizontal ray running from the test point in the positive x direction.

        // check if the segment is strictly to the left of the test point
        if p1.x < self.p.x && p2.x < self.p.x {
            return;
        }

        // check if the point is equal to the current ring vertex
        if self.p.x == p2.x && self.p.y == p2.y {
            self.is_point_on_segment = true;
            return;
        }
        // For horizontal segments, check if the point is on the segment.
        // Otherwise, horizontal segments are not counted.
        if p1.y == self.p.y && p2.y == self.p.y {
            let mut minx = p1.x;
            let mut maxx = p2.x;
            if minx > maxx {
                minx = p2.x;
                maxx = p1.x;
            }
            if self.p.x >= minx && self.p.x <= maxx {
                self.is_point_on_segment = true;
            }
            return;
        }
        // Evaluate all non-horizontal segments which cross a horizontal ray to the
        // right of the test pt. To avoid double-counting shared vertices, we use the
        // convention that
        // <ul>
        // <li>an upward edge includes its starting endpoint, and excludes its
        // final endpoint
        // <li>a downward edge excludes its starting endpoint, and includes its
        // final endpoint
        // </ul>
        if ((p1.y > self.p.y) && (p2.y <= self.p.y)) || ((p2.y > self.p.y) && (p1.y <= self.p.y)) {
            let mut orient = Orientation::index(p1, p2, &self.p);
            if orient == Orientation::COLLINEAR {
                self.is_point_on_segment = true;
                return;
            }
            // Re-orient the result if needed to ensure effective segment direction is upwards
            if p2.y < p1.y {
                orient = -orient;
            }
            // The upward segment crosses the ray if the test point lies to the left (CCW) of the segment.
            if orient == Orientation::LEFT {
                self.crossing_count += 1;
            }
        }
    }

    /**
     * Gets the count of crossings.
     *
     * @return the crossing count
     */
    pub fn get_count(&self) -> i32 {
        return self.crossing_count;
    }

    /**
     * Reports whether the point lies exactly on one of the supplied segments.
     * This method may be called at any time as segments are processed.
     * If the result of this method is <tt>true</tt>,
     * no further segments need be supplied, since the result
     * will never change again.
     *
     * @return true if the point lies exactly on a segment
     */
    pub fn is_on_segment(&self) -> bool {
        return self.is_point_on_segment;
    }

    /**
     * Gets the {@link Location} of the point relative to
     * the ring, polygon
     * or multipolygon from which the processed segments were provided.
     * <p>
     * This method only determines the correct location
     * if <b>all</b> relevant segments must have been processed.
     *
     * @return the Location of the point
     */
    pub fn get_location(&self) -> i32 {
        if self.is_point_on_segment {
            return Location::BOUNDARY;
        }

        // The point is in the interior of the ring if the number of X-crossings is
        // odd.
        if (self.crossing_count % 2) == 1 {
            return Location::INTERIOR;
        }
        return Location::EXTERIOR;
    }

    /**
     * Tests whether the point lies in or on
     * the ring, polygon
     * or multipolygon from which the processed segments were provided.
     * <p>
     * This method only determines the correct location
     * if <b>all</b> relevant segments must have been processed.
     *
     * @return true if the point lies in or on the supplied polygon
     */
    pub fn is_point_in_polygon(&self) -> bool {
        return self.get_location() != Location::EXTERIOR;
    }
}
