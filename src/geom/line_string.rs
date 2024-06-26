use crate::algorithm::length::Length;

use super::{
    coordinate::Coordinate, coordinate_array_sequences::CoordinateArraySequences,
    coordinate_sequence_comparator::CoordinateSequenceComparator, dimension::Dimension,
    envelope::Envelope, geometry::Geometry, geometry_factory::GeometryFactory,
    implementation::coordinate_array_sequence::CoordinateArraySequence, point::Point,
};

/**
 *  Models an OGC-style <code>LineString</code>.
 *  A LineString consists of a sequence of two or more vertices,
 *  along with all points along the linearly-interpolated curves
 *  (line segments) between each
 *  pair of consecutive vertices.
 *  Consecutive vertices may be equal.
 *  The line segments in the line may intersect each other (in other words,
 *  the linestring may "curl back" in itself and self-intersect.
 *  Linestrings with exactly two identical points are invalid.
 *  <p>
 * A linestring must have either 0 or 2 or more points.
 * If these conditions are not met, the constructors throw
 * an {@link IllegalArgumentException}
 *
 *@version 1.7
 */

pub struct LineString {
    /**
     *  The points of this <code>LineString</code>.
     */
    points: CoordinateArraySequence,

    /**
     *  The bounding box of this <code>Geometry</code>.
     */
    envelope: Option<Envelope>,
}

impl LineString {
    /**
     * The minimum number of vertices allowed in a valid non-empty linestring.
     * Empty linestrings with 0 vertices are also valid.
     */
    pub const MINIMUM_VALID_SIZE: usize = 2;

    /**
     * Constructs a <code>LineString</code> with the given points.
     *
     *@param  points the points of the linestring, or <code>null</code>
     *      to create the empty geometry.
     * @throws IllegalArgumentException if too few points are provided
     */
    pub fn new_from_coordinate_sequence(points: CoordinateArraySequence) -> Self {
        Self {
            points,
            envelope: None,
        }
    }

    pub fn init(&mut self, points: CoordinateArraySequence) {
        self.points = points;
    }

    pub fn get_coordinates(&self) -> Vec<Coordinate> {
        return self.points.to_coordinate_array();
    }

    pub fn get_coordinate_sequence(&self) -> CoordinateArraySequence {
        return CoordinateArraySequence::new_from_coordinate_array_sequence(&self.points);
    }

    pub fn get_coordinate_n(&self, n: usize) -> Coordinate {
        return self.points.get_coordinate_index(n);
    }

    pub fn get_coordinate(&self) -> Option<Coordinate> {
        if self.is_empty() {
            return None;
        }
        return Some(self.points.get_coordinate_index(0));
    }

    pub fn get_dimension(&self) -> i32 {
        return 1;
    }

    pub fn get_boundary_dimension(&self) -> i32 {
        if self.is_closed() {
            return Dimension::FALSE;
        }
        return 0;
    }

    pub fn is_empty(&self) -> bool {
        return self.points.size() == 0;
    }

    pub fn get_num_points(&self) -> usize {
        return self.points.size();
    }

    pub fn get_point_n(&self, n: usize) -> Point {
        return GeometryFactory::create_point_from_coordinate(&self.points.get_coordinate_index(n));
    }

    pub fn get_start_point(&self) -> Option<Point> {
        if self.is_empty() {
            return None;
        }
        return Some(self.get_point_n(0));
    }

    pub fn get_end_point(&self) -> Option<Point> {
        if self.is_empty() {
            return None;
        }
        return Some(self.get_point_n(self.get_num_points() - 1));
    }

    pub fn is_closed(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        return self
            .get_coordinate_n(0)
            .equals_2d(&self.get_coordinate_n(self.get_num_points() - 1));
    }

    pub fn is_ring(&self) -> bool {
        return self.is_closed();
        // TODO: Implement isSimpleOp
        // return self.isClosed() && self.isSimple();
    }

    pub fn get_geometry_type(&self) -> String {
        return Geometry::TYPENAME_LINESTRING.to_owned();
    }

    /**
     *  Returns the length of this <code>LineString</code>
     *
     *@return the length of the linestring
     */
    pub fn get_length(&self) -> f64 {
        return Length::of_line(&self.points);
    }

    //   /**
    //    * Gets the boundary of this geometry.
    //    * The boundary of a lineal geometry is always a zero-dimensional geometry (which may be empty).
    //    *
    //    * @return the boundary geometry
    //    * @see Geometry#getBoundary
    //    */
    // TODO: Implement ME!
    //   public Geometry getBoundary() {
    //     return (new BoundaryOp(this)).getBoundary();
    //   }

    /**
     * Creates a {@link LineString} whose coordinates are in the reverse
     * order of this objects
     *
     * @return a {@link LineString} with coordinates in the reverse order
     */
    pub fn reverse(&self) -> LineString {
        let mut res = self.reverse_internal();
        if self.envelope.is_some() {
            res.envelope = Some(self.envelope.unwrap().copy());
        }

        return res;
    }

    pub fn reverse_internal(&self) -> LineString {
        let mut seq = self.points.copy();
        CoordinateArraySequences::reverse(&mut seq);
        return GeometryFactory::create_line_string_coordinate_array_sequence(&seq);
    }

    /**
     *  Returns true if the given point is a vertex of this <code>LineString</code>.
     *
     *@param  pt  the <code>Coordinate</code> to check
     *@return     <code>true</code> if <code>pt</code> is one of this <code>LineString</code>
     *      's vertices
     */
    pub fn is_coordinate(&self, pt: &Coordinate) -> bool {
        for i in 0..self.points.size() {
            if self.points.get_coordinate_index(i).equals_2d(pt) {
                return true;
            }
        }
        return false;
    }

    pub fn compute_envelope_internal(&self) -> Envelope {
        if self.is_empty() {
            return Envelope::default();
        }
        let mut default = Envelope::default();
        self.points.expand_envelope(&mut default);
        return default;
    }

    pub fn equals_exact(&self, other: &LineString, tolerance: f64) -> bool {
        if self.points.size() != other.points.size() {
            return false;
        }
        for i in 0..self.points.size() {
            if !Geometry::equal(
                &self.points.get_coordinate_index(i),
                &other.points.get_coordinate_index(i),
                tolerance,
            ) {
                return false;
            }
        }
        return true;
    }

    // TODO: Implement CoordinateFilters
    //   pub fn apply_coordinate_filter(&self, filter: &CoordinateFilter) {
    //     for i in 0..self.points.size() {
    //         filter.filter(self.points.get_coordinate_index(i));
    //       }
    //   }

    //   pub fn apply_coordinate_sequence_filter(&self, filter: &CoordinateSequenceFilter) {
    //     if self.points.size() == 0 {
    //       return;
    //     }
    //     for i in 0..self.points.size() {
    //       filter.filter(self.points, i);
    //       if filter.isDone(){
    //           break;
    //         }
    //     }
    //     if filter.isGeometryChanged() {
    //         self.geometryChanged();
    //     }
    //   }

    //   pub fn applyGeometryFilter(&self, filter: &GeometryFilter) {
    //     filter.filter(self);
    //   }

    //   pub fn applyComponentFilter(&self, filter: &GeometryComponentFilter) {
    //     filter.filter(self);
    //   }

    pub fn copy_internal(&self) -> LineString {
        return LineString::new_from_coordinate_sequence(self.points.copy());
    }

    /**
     * Normalizes a LineString.  A normalized linestring
     * has the first point which is not equal to it's reflected point
     * less than the reflected point.
     */
    pub fn normalize(&mut self) {
        for i in 0..(self.points.size() / 2) {
            let j = self.points.size() - 1 - i;
            // skip equal points on both ends
            if !self
                .points
                .get_coordinate_index(i)
                .equals_2d(&self.points.get_coordinate_index(j))
            {
                if self
                    .points
                    .get_coordinate_index(i)
                    .compare_to(&self.points.get_coordinate_index(j))
                    > 0
                {
                    let mut copy = self.points.copy();
                    CoordinateArraySequences::reverse(&mut copy);
                    self.points = copy;
                }
                return;
            }
        }
    }

    pub fn compare_to(&self, line: &LineString) -> i32 {
        // MD - optimized implementation
        let mut i = 0;
        let mut j = 0;
        while i < self.points.size() && j < line.points.size() {
            let comparison = self
                .points
                .get_coordinate_index(i)
                .compare_to(&line.points.get_coordinate_index(j));
            if comparison != 0 {
                return comparison;
            }
            i += 1;
            j += 1;
        }
        if i < self.points.size() {
            return 1;
        }
        if j < line.points.size() {
            return -1;
        }
        return 0;
    }

    pub fn compare_to_with_comparator(
        &self,
        line: &LineString,
        comp: &CoordinateSequenceComparator,
    ) -> i32 {
        return comp.compare_coordinate_array_sequence(&self.points, &line.points);
    }

    pub fn get_type_code() -> i32 {
        return Geometry::TYPECODE_LINESTRING;
    }
}
