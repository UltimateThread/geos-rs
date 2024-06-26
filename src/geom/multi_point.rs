use super::{
    coordinate::Coordinate, dimension::Dimension, envelope::Envelope, geometry::Geometry,
    point::Point, precision_model::PrecisionModel,
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

    pub fn get_dimension() -> i32 {
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
