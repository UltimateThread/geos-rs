use crate::core::geom::{
    coordinate::Coordinate, envelope::Envelope,
    implementation::coordinate_array_sequence::CoordinateArraySequence, location::Location,
};

use super::{orientation::Orientation, ray_crossing_counter::RayCrossingCounter};

/**
 * Functions for locating points within basic geometric
 * structures such as line segments, lines and rings.
 *
 * @author Martin Davis
 *
 */

pub struct PointLocation {}

impl PointLocation {
    /**
     * Tests whether a point lies on a line segment.
     *
     * @param p the point to test
     * @param p0 a point of the line segment
     * @param p1 a point of the line segment
     * @return true if the point lies on the line segment
     */
    pub fn is_on_segment(p: &Coordinate, p0: &Coordinate, p1: &Coordinate) -> bool {
        //-- test envelope first since it's faster
        if !Envelope::intersects_3(p0, p1, p) {
            return false;
        }
        //-- handle zero-length segments
        if p.equals_2d(p0) {
            return true;
        }
        let is_on_line = Orientation::COLLINEAR == Orientation::index(p0, p1, p);
        return is_on_line;
    }

    /**
     * Tests whether a point lies on the line defined by a list of
     * coordinates.
     *
     * @param p the point to test
     * @param line the line coordinates
     * @return true if the point is a vertex of the line or lies in the interior
     *         of a line segment in the line
     */
    pub fn is_on_line_coordinates(p: &Coordinate, line: &Vec<Coordinate>) -> bool {
        for i in 1..line.len() {
            let p0 = line[i - 1];
            let p1 = line[i];
            if PointLocation::is_on_segment(p, &p0, &p1) {
                return true;
            }
        }
        return false;
    }

    /**
     * Tests whether a point lies on the line defined by a
     * {@link CoordinateSequence}.
     *
     * @param p the point to test
     * @param line the line coordinates
     * @return true if the point is a vertex of the line or lies in the interior
     *         of a line segment in the line
     */
    pub fn is_on_line_coordinate_array_sequence(
        p: &Coordinate,
        line: &CoordinateArraySequence,
    ) -> bool {
        let mut p0 = Coordinate::default();
        let mut p1 = Coordinate::default();
        let n = line.size();
        for i in 1..n {
            line.get_coordinate_index_coordinate(i - 1, &mut p0);
            line.get_coordinate_index_coordinate(i, &mut p1);
            if PointLocation::is_on_segment(p, &p0, &p1) {
                return true;
            }
        }
        return false;
    }

    /**
     * Tests whether a point lies inside or on a ring. The ring may be oriented in
     * either direction. A point lying exactly on the ring boundary is considered
     * to be inside the ring.
     * <p>
     * This method does <i>not</i> first check the point against the envelope of
     * the ring.
     *
     * @param p
     *          point to check for ring inclusion
     * @param ring
     *          an array of coordinates representing the ring (which must have
     *          first point identical to last point)
     * @return true if p is inside ring
     *
     * @see PointLocation#locateInRing(Coordinate, Coordinate[])
     */
    pub fn is_in_ring(p: &Coordinate, ring: &Vec<Coordinate>) -> bool {
        return PointLocation::locate_in_ring(p, ring) != Location::EXTERIOR;
    }

    /**
     * Determines whether a point lies in the interior, on the boundary, or in the
     * exterior of a ring. The ring may be oriented in either direction.
     * <p>
     * This method does <i>not</i> first check the point against the envelope of
     * the ring.
     *
     * @param p
     *          point to check for ring inclusion
     * @param ring
     *          an array of coordinates representing the ring (which must have
     *          first point identical to last point)
     * @return the {@link Location} of p relative to the ring
     */
    pub fn locate_in_ring(p: &Coordinate, ring: &Vec<Coordinate>) -> i32 {
        return RayCrossingCounter::locate_point_in_ring_vec(p, ring);
    }
}
