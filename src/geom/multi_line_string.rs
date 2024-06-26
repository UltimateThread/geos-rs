use super::{
    dimension::Dimension, envelope::Envelope, geometry::Geometry, line_string::LineString,
    precision_model::PrecisionModel,
};

/**
 * Models a collection of {@link LineString}s.
 * <p>
 * Any collection of LineStrings is a valid MultiLineString.
 *
 *@version 1.7
 */

pub struct MultiLineString {
    line_strings: Vec<LineString>,
    precision_model: Option<PrecisionModel>,
    envelope: Option<Envelope>,
}

impl MultiLineString {
    /**
     *  Constructs a <code>MultiLineString</code>.
     *
     *@param  lineStrings     the <code>LineString</code>s for this <code>MultiLineString</code>
     *      , or <code>null</code> or an empty array to create the empty geometry.
     *      Elements may be empty <code>LineString</code>s, but not <code>null</code>
     *      s.
     *@param  precisionModel  the specification of the grid of allowable points
     *      for this <code>MultiLineString</code>
     *@param  SRID            the ID of the Spatial Reference System used by this
     *      <code>MultiLineString</code>
     * @deprecated Use GeometryFactory instead
     */
    pub fn new_with_line_strings_precision_model(
        line_strings: &Vec<LineString>,
        precision_model: PrecisionModel,
    ) -> Self {
        Self {
            line_strings: line_strings.to_vec(),
            precision_model: Some(precision_model),
            envelope: None,
        }
    }

    /**
     * @param lineStrings
     *            the <code>LineString</code>s for this <code>MultiLineString</code>,
     *            or <code>null</code> or an empty array to create the empty
     *            geometry. Elements may be empty <code>LineString</code>s,
     *            but not <code>null</code>s.
     */
    pub fn new_with_line_strings(line_strings: &Vec<LineString>) -> Self {
        Self {
            line_strings: line_strings.to_vec(),
            precision_model: None,
            envelope: None,
        }
    }

    pub fn get_dimension(&self) -> i32 {
        return 1;
    }

    pub fn has_dimension(&self, dim: i32) -> bool {
        return dim == 1;
    }

    pub fn get_boundary_dimension(&self) -> i32 {
        if self.is_closed() {
            return Dimension::FALSE;
        }
        return 0;
    }

    pub fn get_geometry_type(&self) -> String {
        return Geometry::TYPENAME_MULTILINESTRING.to_owned();
    }

    pub fn is_empty(&self) -> bool {
        for i in 0..self.line_strings.len() {
            if !self.line_strings[i].is_empty() {
                return false;
            }
        }
        return true;
    }

    pub fn is_closed(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        for i in 0..self.line_strings.len() {
            if !self.line_strings[i].is_closed() {
                return false;
            }
        }
        return true;
    }

    //   /**
    //    * Gets the boundary of this geometry.
    //    * The boundary of a lineal geometry is always a zero-dimensional geometry (which may be empty).
    //    *
    //    * @return the boundary geometry
    //    * @see Geometry#getBoundary
    //    */
    // TODO: Implement ME!
    //   public Geometry getBoundary()
    //   {
    //     return (new BoundaryOp(this)).getBoundary();
    //   }

    /**
     * Creates a {@link MultiLineString} in the reverse
     * order to this object.
     * Both the order of the component LineStrings
     * and the order of their coordinate sequences
     * are reversed.
     *
     * @return a {@link MultiLineString} in the reverse order
     */
    pub fn reverse(&self) -> MultiLineString {
        let mut res = self.reverse_internal();
        if self.envelope.is_some() {
            res.envelope = Some(self.envelope.unwrap().copy());
        }

        return res;
    }

    fn reverse_internal(&self) -> MultiLineString {
        let mut line_strings: Vec<LineString> = vec![];
        for i in 0..line_strings.len() {
            line_strings[i] = self.line_strings[i].reverse();
        }
        return MultiLineString::new_with_line_strings(&line_strings);
    }

    pub fn copy_internal(&self) -> MultiLineString {
        let mut line_strings: Vec<LineString> = vec![];
        for i in 0..self.line_strings.len() {
            line_strings[i] = self.line_strings[i].copy();
        }
        return MultiLineString::new_with_line_strings(&line_strings);
    }

    pub fn equals_exact(&self, other: &MultiLineString, tolerance: f64) -> bool {
        if self.line_strings.len() != other.line_strings.len() {
            return false;
        }
        for i in 0..self.line_strings.len() {
            if !(self.line_strings[i]).equals_exact(&other.line_strings[i], tolerance) {
                return false;
            }
        }
        return true;
    }

    pub fn get_type_code() -> i32 {
        return Geometry::TYPECODE_MULTILINESTRING;
    }
}
