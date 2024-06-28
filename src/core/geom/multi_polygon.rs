use super::{
    envelope::Envelope, geometry::Geometry, polygon::Polygon, precision_model::PrecisionModel,
};

/**
 * Models a collection of {@link Polygon}s.
 * <p>
 * As per the OGC SFS specification,
 * the Polygons in a MultiPolygon may not overlap,
 * and may only touch at single points.
 * This allows the topological point-set semantics
 * to be well-defined.
 *
 *
 *@version 1.7
 */

#[derive(Clone)]
pub struct MultiPolygon {
    polygons: Vec<Polygon>,
    precision_model: Option<PrecisionModel>,
    envelope: Option<Envelope>,
}

impl MultiPolygon {
    /**
     *  Constructs a <code>MultiPolygon</code>.
     *
     *@param  polygons        the <code>Polygon</code>s for this <code>MultiPolygon</code>
     *      , or <code>null</code> or an empty array to create the empty geometry.
     *      Elements may be empty <code>Polygon</code>s, but not <code>null</code>
     *      s. The polygons must conform to the assertions specified in the <A
     *      HREF="http://www.opengis.org/techno/specs.htm">OpenGIS Simple Features
     *      Specification for SQL</A> .
     *@param  precisionModel  the specification of the grid of allowable points
     *      for this <code>MultiPolygon</code>
     *@param  SRID            the ID of the Spatial Reference System used by this
     *      <code>MultiPolygon</code>
     * @deprecated Use GeometryFactory instead
     */
    pub fn new_with_polygons_precision_model(
        polygons: &Vec<Polygon>,
        precision_model: PrecisionModel,
    ) -> Self {
        Self {
            polygons: polygons.to_vec(),
            precision_model: Some(precision_model),
            envelope: None,
        }
    }

    /**
     * @param polygons
     *            the <code>Polygon</code>s for this <code>MultiPolygon</code>,
     *            or <code>null</code> or an empty array to create the empty
     *            geometry. Elements may be empty <code>Polygon</code>s, but
     *            not <code>null</code>s. The polygons must conform to the
     *            assertions specified in the <A
     *            HREF="http://www.opengis.org/techno/specs.htm">OpenGIS Simple
     *            Features Specification for SQL</A>.
     */
    pub fn new_with_polygons(polygons: &Vec<Polygon>) -> Self {
        Self {
            polygons: polygons.to_vec(),
            precision_model: None,
            envelope: None,
        }
    }

    pub fn get_dimension(&self) -> i32 {
        return 2;
    }

    pub fn has_dimension(&self, dim: i32) -> bool {
        return dim == 2;
    }

    pub fn get_boundary_dimension(&self) -> i32 {
        return 1;
    }

    pub fn get_geometry_type(&self) -> String {
        return Geometry::TYPENAME_MULTIPOLYGON.to_owned();
    }

    /*
      public boolean isSimple() {
        return true;
      }
    */

    //   /**
    //    * Computes the boundary of this geometry
    //    *
    //    * @return a lineal geometry (which may be empty)
    //    * @see Geometry#getBoundary
    //    */
    // TODO: Implement ME!
    //   pub fn getBoundary(&self) -> MultiLineString {
    //     if self.isEmpty() {
    //       return GeometryFactory::createMultiLineString();
    //     }
    //     // ArrayList allRings = new ArrayList();
    //     for i in 0..self.polygons.len() {
    //       let polygon = self.polygons[i];
    //       let rings = polygon.get_boundary();
    //       for j in 0..rings.get_num_geometries() {
    //         allRings.add(rings.getGeometryN(j));
    //       }
    //     }
    //     LineString[] allRingsArray = new LineString[allRings.size()];
    //     return getFactory().createMultiLineString((LineString[]) allRings.toArray(allRingsArray));
    //   }

    pub fn equals_exact(&self, other: MultiPolygon, tolerance: f64) -> bool {
        if self.polygons.len() != other.polygons.len() {
            return false;
        }
        for i in 0..self.polygons.len() {
            if !(self.polygons[i]).equals_exact(&other.polygons[i], tolerance) {
                return false;
            }
        }
        return true;
    }

    /**
     * Creates a {@link MultiPolygon} with
     * every component reversed.
     * The order of the components in the collection are not reversed.
     *
     * @return a MultiPolygon in the reverse order
     */
    pub fn reverse(&self) -> MultiPolygon {
        let mut res = self.reverse_internal();
        if self.envelope.is_some() {
            res.envelope = Some(self.envelope.unwrap().copy());
        }

        return res;
    }

    fn reverse_internal(&self) -> MultiPolygon {
        let mut polygons: Vec<Polygon> = vec![];
        for i in 0..self.polygons.len() {
            polygons[i] = self.polygons[i].reverse();
        }
        return MultiPolygon::new_with_polygons(&polygons);
    }

    pub fn copy(&self) -> MultiPolygon {
        self.copy_internal()
    }

    fn copy_internal(&self) -> MultiPolygon {
        let mut polygons: Vec<Polygon> = vec![];
        for i in 0..self.polygons.len() {
            polygons[i] = self.polygons[i].copy();
        }
        return MultiPolygon::new_with_polygons(&polygons);
    }

    pub fn get_type_code(&self) -> i32 {
        return Geometry::TYPECODE_MULTIPOLYGON;
    }
}
