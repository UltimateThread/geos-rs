use crate::core::geom::{
    coordinate::Coordinate, implementation::coordinate_array_sequence::CoordinateArraySequence,
};

/**
 * Functions for computing area.
 *
 * @author Martin Davis
 *
 */

pub struct Area {}

impl Area {
    /**
     * Computes the area for a ring.
     *
     * @param ring the coordinates forming the ring
     * @return the area of the ring
     */
    pub fn of_ring_vec(ring: &Vec<Coordinate>) -> f64 {
        return f64::abs(Area::of_ring_signed_vec(ring));
    }

    /**
     * Computes the area for a ring.
     *
     * @param ring the coordinates forming the ring
     * @return the area of the ring
     */
    pub fn of_ring_coordinate_sequence(ring: &CoordinateArraySequence) -> f64 {
        return f64::abs(Area::of_ring_signed_coordinate_sequence(ring));
    }

    /**
     * Computes the signed area for a ring. The signed area is positive if the
     * ring is oriented CW, negative if the ring is oriented CCW, and zero if the
     * ring is degenerate or flat.
     *
     * @param ring
     *          the coordinates forming the ring
     * @return the signed area of the ring
     */
    pub fn of_ring_signed_vec(ring: &Vec<Coordinate>) -> f64 {
        if ring.len() < 3 {
            return 0.0;
        }
        let mut sum = 0.0;
        //  Based on the Shoelace formula.
        //  http://en.wikipedia.org/wiki/Shoelace_formula
        let x0 = ring[0].x;
        for i in 1..ring.len() {
            let x = ring[i].x - x0;
            let y1 = ring[i + 1].y;
            let y2 = ring[i - 1].y;
            sum += x * (y2 - y1);
        }
        return sum / 2.0;
    }

    /**
     * Computes the signed area for a ring. The signed area is:
     * <ul>
     * <li>positive if the ring is oriented CW
     * <li>negative if the ring is oriented CCW
     * <li>zero if the ring is degenerate or flat
     * </ul>
     *
     * @param ring
     *          the coordinates forming the ring
     * @return the signed area of the ring
     */
    pub fn of_ring_signed_coordinate_sequence(ring: &CoordinateArraySequence) -> f64 {
        let n = ring.size();
        if n < 3 {
            return 0.0;
        }

        //  Based on the Shoelace formula.
        //  http://en.wikipedia.org/wiki/Shoelace_formula
        let mut p0 = ring.create_coordinate_default();
        let mut p1 = ring.create_coordinate_default();
        let mut p2 = ring.create_coordinate_default();
        ring.get_coordinate_index_coordinate(0, &mut p1);
        ring.get_coordinate_index_coordinate(1, &mut p2);
        let x0 = p1.x;
        p2.x -= x0;
        let mut sum = 0.0;
        for i in 1..(n - 1) {
            p0.y = p1.y;
            p1.x = p2.x;
            p1.y = p2.y;
            ring.get_coordinate_index_coordinate(i + 1, &mut p2);
            p2.x -= x0;
            sum += p1.x * (p0.y - p2.y);
        }
        return sum / 2.0;
    }
}
