use super::{
    coordinate::Coordinate, dimension::Dimension, envelope::Envelope, geometry::Geometry, point::Point, precision_model::PrecisionModel
};

/**
 * Models a collection of {@link Point}s.
 * <p>
 * Any collection of Points is a valid MultiPoint.
 *
 *@version 1.7
 */

pub struct MultiPoint {
    points: Vec<Point>,
    precision_model: Option<PrecisionModel>,
    envelope: Option<Envelope>,
}

impl MultiPoint {
    /**
     *  Constructs a <code>MultiPoint</code>.
     *
     *@param  points          the <code>Point</code>s for this <code>MultiPoint</code>
     *      , or <code>null</code> or an empty array to create the empty geometry.
     *      Elements may be empty <code>Point</code>s, but not <code>null</code>s.
     *@param  precisionModel  the specification of the grid of allowable points
     *      for this <code>MultiPoint</code>
     *@param  SRID            the ID of the Spatial Reference System used by this
     *      <code>MultiPoint</code>
     * @deprecated Use GeometryFactory instead
     */
    pub fn new_with_points_precision_model(
        points: &Vec<Point>,
        precision_model: PrecisionModel,
    ) -> Self {
        Self {
            points: points.to_vec(),
            precision_model: Some(precision_model),
            envelope: None,
        }
    }

    /**
     *@param  points          the <code>Point</code>s for this <code>MultiPoint</code>
     *      , or <code>null</code> or an empty array to create the empty geometry.
     *      Elements may be empty <code>Point</code>s, but not <code>null</code>s.
     */
    pub fn new_with_points(points: &Vec<Point>) -> Self {
        Self {
            points: points.to_vec(),
            precision_model: None,
            envelope: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.points.len() == 0;
    }

    pub fn get_num_points(&self) -> usize {
        return self.points.len();
    }

    pub fn get_precision_model(&self) -> Option<PrecisionModel> {
        return self.precision_model;
    }

    pub fn get_dimension(&self) -> i32 {
        return 0;
    }

    pub fn has_dimension(&self, dim: i32) -> bool {
        return dim == 0;
    }

    pub fn get_boundary_dimension(&self) -> i32 {
        return Dimension::FALSE;
    }

    pub fn get_geometry_type(&self) -> String {
        return Geometry::TYPENAME_MULTIPOINT.to_owned();
    }

    //   /**
    //    * Gets the boundary of this geometry.
    //    * Zero-dimensional geometries have no boundary by definition,
    //    * so an empty GeometryCollection is returned.
    //    *
    //    * @return an empty GeometryCollection
    //    * @see Geometry#getBoundary
    //    */
    // TODO: Implement ME!
    //   public Geometry getBoundary() {
    //     return getFactory().createGeometryCollection();
    //   }

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

    fn compute_envelope_internal(&mut self) -> Envelope {
        let mut envelope = Envelope::default();
        for i in 0..self.points.len() {
            envelope.expand_to_include_envelope(&self.points[i].get_envelope_internal());
        }
        return envelope;
    }

    pub fn reverse(&self) -> MultiPoint {
        let mut res = self.reverse_internal();
        if self.envelope.is_some() {
            res.envelope = Some(self.envelope.unwrap().copy());
        }

        return res;
    }

    fn reverse_internal(&self) -> MultiPoint {
        let mut points: Vec<Point> = vec![];
        for i in 0..self.points.len() {
            points[i] = self.points[i].copy();
        }
        return MultiPoint::new_with_points(&points);
    }

    /**
     * Tests whether this geometry is
     * topologically equal to the argument geometry.
     * <p>
     * This method is included for backward compatibility reasons.
     * It has been superseded by the {@link #equalsTopo(Geometry)} method,
     * which has been named to clearly denote its functionality.
     * <p>
     * This method should NOT be confused with the method
     * {@link #equals(Object)}, which implements
     * an exact equality comparison.
     *
     *@param  g  the <code>Geometry</code> with which to compare this <code>Geometry</code>
     *@return true if the two <code>Geometry</code>s are topologically equal
     *
     *@see #equalsTopo(Geometry)
     */
    // pub fn equals(&mut self, multipoint: &mut MultiPoint) -> bool {
    //     return self.equals_topo(multipoint);
    // }

    /**
     * Tests whether this geometry is topologically equal to the argument geometry
     * as defined by the SFS <code>equals</code> predicate.
     * <p>
     * The SFS <code>equals</code> predicate has the following equivalent definitions:
     * <ul>
     * <li>The two geometries have at least one point in common,
     * and no point of either geometry lies in the exterior of the other geometry.
     * <li>The DE-9IM Intersection Matrix for the two geometries matches
     * the pattern <code>T*F**FFF*</code>
     * <pre>
     * T*F
     * **F
     * FF*
     * </pre>
     * </ul>
     * <b>Note</b> that this method computes <b>topologically equality</b>.
     * For structural equality, see {@link #equalsExact(Geometry)}.
     *
     *@param g the <code>Geometry</code> with which to compare this <code>Geometry</code>
     *@return <code>true</code> if the two <code>Geometry</code>s are topologically equal
     *
     *@see #equalsExact(Geometry)
     */
    // pub fn equals_topo(&mut self, multipoint: &mut MultiPoint) -> bool {
    //     // short-circuit test
    //     if !self
    //         .get_envelope_internal()
    //         .equals(&multipoint.get_envelope_internal())
    //     {
    //         return false;
    //     }
    //     return self.relate(multipoint).is_equals(self.get_dimension(), multipoint.get_dimension());
    // }

    // /**
    //  *  Returns the DE-9IM {@link IntersectionMatrix} for the two <code>Geometry</code>s.
    //  *
    //  *@param  g  the <code>Geometry</code> with which to compare this <code>Geometry</code>
    //  *@return        an {@link IntersectionMatrix} describing the intersections of the interiors,
    //  *      boundaries and exteriors of the two <code>Geometry</code>s
    //  */
    // pub fn relate(&self, multipoint: &MultiPoint) -> IntersectionMatrix {
    //     return RelateOpMultiPoint::relate_multipoints(self, multipoint);
    // }

    pub fn equals_exact(&self, other: MultiPoint, tolerance: f64) -> bool {
        if self.points.len() != other.points.len() {
            return false;
        }
        for i in 0..self.points.len() {
            if !(self.points[i]).equals_exact(&other.points[i], tolerance) {
                return false;
            }
        }
        return true;
    }

    pub fn get_point_at_index(&self, index: usize) -> Option<Point> {
        if self.points.len() == 0 || self.points.len() < index - 1 {
            return None;
        }
        return Some(self.points[index].copy());
    }

    /**
     *  Returns the <code>Coordinate</code> at the given position.
     *
     *@param  n  the index of the <code>Coordinate</code> to retrieve, beginning
     *      at 0
     *@return    the <code>n</code>th <code>Coordinate</code>
     */
    pub fn get_coordinate(&self, n: usize) -> Option<Coordinate> {
        return self.points[n].get_coordinate();
    }

    pub fn copy(&self) -> MultiPoint {
        let mut points: Vec<Point> = vec![];
        for i in 0..self.points.len() {
            points[i] = self.points[i].copy();
        }
        return MultiPoint::new_with_points(&points);
    }

    pub fn get_type_code() -> i32 {
        return Geometry::TYPECODE_MULTIPOINT;
    }
}
