use crate::core::geom::implementation::coordinate_array_sequence::CoordinateArraySequence;

/**
 * Functions for computing length.
 *
 * @author Martin Davis
 *
 */

pub struct Length {}

impl Length {
    /**
     * Computes the length of a linestring specified by a sequence of points.
     *
     * @param pts the points specifying the linestring
     * @return the length of the linestring
     */
    pub fn of_line(pts: &CoordinateArraySequence) -> f64 {
        // optimized for processing CoordinateSequences
        let n = pts.size();
        if n <= 1 {
            return 0.0;
        }

        let mut len = 0.0;

        let mut p = pts.create_coordinate_default();
        pts.get_coordinate_index_coordinate(0, &mut p);
        let mut x0 = p.x;
        let mut y0 = p.y;

        for i in 1..n {
            pts.get_coordinate_index_coordinate(i, &mut p);
            let x1 = p.x;
            let y1 = p.y;
            let dx = x1 - x0;
            let dy = y1 - y0;

            len += f64::hypot(dx, dy);

            x0 = x1;
            y0 = y1;
        }
        return len;
    }
}
