use crate::core::algorithm::length::Length;

use super::{
    coordinate::Coordinate, coordinate_array_sequences::CoordinateArraySequences,
    coordinate_sequence_comparator::CoordinateSequenceComparator, coordinates::Coordinates,
    dimension::Dimension, envelope::Envelope, geometry::Geometry,
    geometry_factory::GeometryFactory,
    implementation::coordinate_array_sequence::CoordinateArraySequence,
    precision_model::PrecisionModel,
};

/**
 * Models an OGC SFS <code>LinearRing</code>.
 * A <code>LinearRing</code> is a {@link LineString} which is both closed and simple.
 * In other words,
 * the first and last coordinate in the ring must be equal,
 * and the ring must not self-intersect.
 * Either orientation of the ring is allowed.
 * <p>
 * A ring must have either 0 or 3 or more points.
 * The first and last points must be equal (in 2D).
 * If these conditions are not met, the constructors throw
 * an {@link IllegalArgumentException}.
 * A ring with 3 points is invalid, because it is collapsed
 * and thus has a self-intersection.  It is allowed to be constructed
 * so that it can be represented, and repaired if needed.
 *
 * @version 1.7
 */

#[derive(Clone)]
pub struct LinearRing {
    points: CoordinateArraySequence,
    precision_model: Option<PrecisionModel>,
    envelope: Option<Envelope>,
}

impl LinearRing {
    /**
     * This method is ONLY used to avoid deprecation warnings.
     * @param points
     * @param factory
     * @throws IllegalArgumentException if the ring is not closed, or has too few points
     */
    pub fn new_with_coordinates(points: &Vec<Coordinate>) -> Self {
        LinearRing::new_with_coordinate_array_sequence(
            &CoordinateArraySequence::new_with_coordinates(points),
        )
    }

    /**
     * Constructs a <code>LinearRing</code> with the vertices
     * specified by the given {@link CoordinateSequence}.
     *
     *@param  points  a sequence points forming a closed and simple linestring, or
     *      <code>null</code> to create the empty geometry.
     *
     * @throws IllegalArgumentException if the ring is not closed, or has too few points
     *
     */
    pub fn new_with_coordinate_array_sequence(points: &CoordinateArraySequence) -> Self {
        Self {
            points: CoordinateArraySequence::new_from_coordinate_array_sequence(points),
            precision_model: None,
            envelope: None,
        }
    }

    pub fn get_coordinate(&self) -> Option<Coordinate> {
        if self.is_empty() {
            return None;
        }
        return Some(self.points.get_coordinate_index(0));
    }

    pub fn get_coordinates(&self) -> Vec<Coordinate> {
        return self.points.to_coordinate_array();
    }

    pub fn get_coordinate_array_sequence(&self) -> CoordinateArraySequence {
        return self.points.copy();
    }

    /**
     * Gets an {@link Envelope} containing
     * the minimum and maximum x and y values in this <code>Geometry</code>.
     * If the geometry is empty, an empty <code>Envelope</code>
     * is returned.
     * <p>
     * The returned object is a copy of the one maintained internally,
     * to avoid aliasing issues.
     * For best performance, clients which access this
     * envelope frequently should cache the return value.
     *
     *@return the envelope of this <code>Geometry</code>.
     *@return an empty Envelope if this Geometry is empty
     */
    pub fn get_envelope_internal(&mut self) -> Envelope {
        if self.envelope.is_none() {
            self.envelope = Some(self.compute_envelope_internal());
        }
        return Envelope::new_envelope(&self.envelope.unwrap());
    }

    pub fn compute_envelope_internal(&self) -> Envelope {
        if self.is_empty() {
            return Envelope::default();
        }
        let mut envelope = Envelope::default();
        self.points.expand_envelope(&mut envelope);
        return envelope;
    }

    /**
     *  Returns the length of this <code>LineString</code>
     *
     *@return the length of the linestring
     */
    pub fn get_length(&self) -> f64 {
        return Length::of_line(&self.points);
    }

    /**
     * Returns <code>Dimension.FALSE</code>, since by definition LinearRings do
     * not have a boundary.
     *
     * @return Dimension.FALSE
     */
    pub fn get_boundary_dimension(&self) -> i32 {
        return Dimension::FALSE;
    }

    pub fn is_empty(&self) -> bool {
        return self.points.size() == 0;
    }

    /**
     * Tests whether this ring is closed.
     * Empty rings are closed by definition.
     *
     * @return true if this ring is closed
     */
    pub fn is_closed(&self) -> bool {
        if self.is_empty() {
            // empty LinearRings are closed by definition
            return true;
        }
        return self
            .get_coordinate_at_index(0)
            .equals_2d(&self.get_coordinate_at_index(self.get_num_points() - 1));
    }

    pub fn get_coordinate_at_index(&self, index: usize) -> Coordinate {
        return self.points.get_coordinate_index(index);
    }

    pub fn get_num_points(&self) -> usize {
        return self.points.size();
    }

    pub fn get_geometry_type(&self) -> String {
        return Geometry::TYPENAME_LINEARRING.to_owned();
    }

    pub fn get_type_code(&self) -> i32 {
        return Geometry::TYPECODE_LINEARRING;
    }

    pub fn copy(&self) -> LinearRing {
        self.copy_internal()
    }

    fn copy_internal(&self) -> LinearRing {
        return LinearRing::new_with_coordinate_array_sequence(&self.points.copy());
    }

    pub fn reverse(&self) -> LinearRing {
        let mut res = self.reverse_internal();
        if self.envelope.is_some() {
            res.envelope = Some(self.envelope.unwrap().copy());
        }

        return res;
    }

    pub fn reverse_internal(&self) -> LinearRing {
        let mut seq = self.points.copy();
        CoordinateArraySequences::reverse(&mut seq);
        return GeometryFactory::create_linear_ring_with_coordinate_array_sequence(&seq);
    }

    pub fn equals_exact(&self, other: &LinearRing, tolerance: f64) -> bool {
        if self.points.size() != other.points.size() {
            return false;
        }
        for i in 0..self.points.size() {
            if !Coordinates::equal(
                &self.points.get_coordinate_index(i),
                &other.points.get_coordinate_index(i),
                tolerance,
            ) {
                return false;
            }
        }
        return true;
    }

    pub fn compare_to_same_class(&self, other: &LinearRing) -> i32 {
        // MD - optimized implementation
        let mut i = 0;
        let mut j = 0;
        while i < self.points.size() && j < other.points.size() {
            let comparison = self
                .points
                .get_coordinate_index(i)
                .compare_to(&other.points.get_coordinate_index(j));
            if comparison != 0 {
                return comparison;
            }
            i += 1;
            j += 1;
        }
        if i < self.points.size() {
            return 1;
        }
        if j < other.points.size() {
            return -1;
        }
        return 0;
    }

    pub fn compare_to_same_class_with_comparator(
        &self,
        other: &LinearRing,
        comp: &CoordinateSequenceComparator,
    ) -> i32 {
        return comp.compare_coordinate_array_sequence(&self.points, &other.points);
    }
}
