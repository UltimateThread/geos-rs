use crate::core::algorithm::{area::Area, centroid::Centroid, orientation::Orientation};

use super::{
    coordinate::Coordinate, coordinate_array_sequences::CoordinateArraySequences,
    coordinate_sequence_comparator::CoordinateSequenceComparator, envelope::Envelope,
    geometry::Geometry, geometry_factory::GeometryFactory, linear_ring::LinearRing, point::Point,
    precision_model::PrecisionModel,
};

/**
 * Represents a polygon with linear edges, which may include holes.
 * The outer boundary (shell)
 * and inner boundaries (holes) of the polygon are represented by {@link LinearRing}s.
 * The boundary rings of the polygon may have any orientation.
 * Polygons are closed, simple geometries by definition.
 * <p>
 * The polygon model conforms to the assertions specified in the
 * <A HREF="http://www.opengis.org/techno/specs.htm">OpenGIS Simple Features
 * Specification for SQL</A>.
 * <p>
 * A <code>Polygon</code> is topologically valid if and only if:
 * <ul>
 * <li>the coordinates which define it are valid coordinates
 * <li>the linear rings for the shell and holes are valid
 * (i.e. are closed and do not self-intersect)
 * <li>holes touch the shell or another hole at at most one point
 * (which implies that the rings of the shell and holes must not cross)
 * <li>the interior of the polygon is connected,
 * or equivalently no sequence of touching holes
 * makes the interior of the polygon disconnected
 * (i.e. effectively split the polygon into two pieces).
 * </ul>
 *
 *@version 1.7
 */
#[derive(Clone)]
pub struct Polygon {
    /**
     *  The exterior boundary,
     * or <code>null</code> if this <code>Polygon</code>
     *  is empty.
     */
    shell: LinearRing,

    /**
     * The interior boundaries, if any.
     * This instance var is never null.
     * If there are no holes, the array is of zero length.
     */
    holes: Vec<LinearRing>,

    precision_model: Option<PrecisionModel>,
    envelope: Option<Envelope>,
}

impl Polygon {
    /**
     *  Constructs a <code>Polygon</code> with the given exterior boundary.
     *
     *@param  shell           the outer boundary of the new <code>Polygon</code>,
     *      or <code>null</code> or an empty <code>LinearRing</code> if the empty
     *      geometry is to be created.
     *@param  precisionModel  the specification of the grid of allowable points
     *      for this <code>Polygon</code>
     *@param  SRID            the ID of the Spatial Reference System used by this
     *      <code>Polygon</code>
     * @deprecated Use GeometryFactory instead
     */
    pub fn new_with_linear_ring_precision_model(
        shell: &LinearRing,
        precision_model: PrecisionModel,
    ) -> Self {
        let coords: Vec<LinearRing> = vec![];
        Polygon::new_with_linear_ring_vec_precision_model(shell, &coords, precision_model)
    }

    pub fn new_with_linear_ring(shell: &LinearRing) -> Self {
        let coords: Vec<LinearRing> = vec![];
        Polygon::new_with_linear_ring_vec(shell, &coords)
    }

    /**
     *  Constructs a <code>Polygon</code> with the given exterior boundary and
     *  interior boundaries.
     *
     *@param  shell           the outer boundary of the new <code>Polygon</code>,
     *      or <code>null</code> or an empty <code>LinearRing</code> if the empty
     *      geometry is to be created.
     *@param  holes           the inner boundaries of the new <code>Polygon</code>
     *      , or <code>null</code> or empty <code>LinearRing</code>s if the empty
     *      geometry is to be created.
     *@param  precisionModel  the specification of the grid of allowable points
     *      for this <code>Polygon</code>
     *@param  SRID            the ID of the Spatial Reference System used by this
     *      <code>Polygon</code>
     * @deprecated Use GeometryFactory instead
     */
    pub fn new_with_linear_ring_vec_precision_model(
        shell: &LinearRing,
        holes: &Vec<LinearRing>,
        precision_model: PrecisionModel,
    ) -> Self {
        Self {
            shell: shell.copy(),
            holes: holes.to_vec(),
            precision_model: Some(precision_model),
            envelope: None,
        }
    }

    /**
     *  Constructs a <code>Polygon</code> with the given exterior boundary and
     *  interior boundaries.
     *
     *@param  shell           the outer boundary of the new <code>Polygon</code>,
     *      or <code>null</code> or an empty <code>LinearRing</code> if the empty
     *      geometry is to be created.
     *@param  holes           the inner boundaries of the new <code>Polygon</code>
     *      , or <code>null</code> or empty <code>LinearRing</code>s if the empty
     *      geometry is to be created.
     */
    pub fn new_with_linear_ring_vec(shell: &LinearRing, holes: &Vec<LinearRing>) -> Self {
        Self {
            shell: shell.copy(),
            holes: holes.to_vec(),
            precision_model: None,
            envelope: None,
        }
    }

    pub fn get_coordinate(&self) -> Option<Coordinate> {
        return self.shell.copy().get_coordinate();
    }

    pub fn get_coordinates(&self) -> Vec<Coordinate> {
        if self.is_empty() {
            return vec![];
        }
        let mut coordinates: Vec<Coordinate> = vec![Coordinate::default(); self.get_num_points()];
        let mut k: i32 = -1;
        let shell_coordinates = self.shell.get_coordinates();
        for x in 0..shell_coordinates.len() {
            k += 1;
            coordinates[k as usize] = shell_coordinates[x];
        }
        for i in 0..self.holes.len() {
            let child_coordinates = self.holes[i].get_coordinates();
            for j in 0..child_coordinates.len() {
                k += 1;
                coordinates[k as usize] = child_coordinates[j];
            }
        }
        return coordinates;
    }

    pub fn get_num_points(&self) -> usize {
        let mut num_points = self.shell.get_num_points();
        for i in 0..self.holes.len() {
            num_points += self.holes[i].get_num_points();
        }
        return num_points;
    }

    pub fn get_dimension(&self) -> i32 {
        return 2;
    }

    pub fn get_boundary_dimension(&self) -> i32 {
        return 1;
    }

    pub fn is_empty(&self) -> bool {
        return self.shell.is_empty();
    }

    pub fn is_rectangle(&self) -> bool {
        if self.get_num_interior_ring() != 0 {
            return false;
        }
        if self.shell.get_num_points() != 5 {
            return false;
        }

        let seq = self.shell.get_coordinate_array_sequence();

        // check vertices have correct values
        let env = self.copy_internal().get_envelope_internal();
        for i in 0..5 {
            let x = seq.get_x(i);
            if !(x == env.get_min_x() || x == env.get_max_x()) {
                return false;
            }
            let y = seq.get_y(i);
            if !(y == env.get_min_y() || y == env.get_max_y()) {
                return false;
            }
        }

        // check vertices are in right order
        let mut prev_x = seq.get_x(0);
        let mut prev_y = seq.get_y(0);
        for i in 1..=4 {
            let x = seq.get_x(i);
            let y = seq.get_y(i);
            let x_changed = x != prev_x;
            let y_changed = y != prev_y;
            if x_changed == y_changed {
                return false;
            }
            prev_x = x;
            prev_y = y;
        }
        return true;
    }

    pub fn get_exterior_ring(&self) -> LinearRing {
        return self.shell.copy();
    }

    pub fn get_num_interior_ring(&self) -> usize {
        return self.holes.len();
    }

    pub fn get_interior_ring_n(&self, n: usize) -> LinearRing {
        return self.holes[n].copy();
    }

    pub fn get_geometry_type(&self) -> String {
        return Geometry::TYPENAME_POLYGON.to_owned();
    }

    /**
     *  Returns the area of this <code>Polygon</code>
     *
     *@return the area of the polygon
     */
    pub fn get_area(&self) -> f64 {
        let mut area = 0.0;
        area += Area::of_ring_coordinate_sequence(&self.shell.get_coordinate_array_sequence());
        for i in 0..self.holes.len() {
            area -=
                Area::of_ring_coordinate_sequence(&self.holes[i].get_coordinate_array_sequence());
        }
        return area;
    }

    /**
     *  Returns the perimeter of this <code>Polygon</code>
     *
     *@return the perimeter of the polygon
     */
    pub fn get_length(&self) -> f64 {
        let mut len = 0.0;
        len += self.shell.get_length();
        for i in 0..self.holes.len() {
            len += self.holes[i].get_length();
        }
        return len;
    }

    /**
     * Computes the boundary of this geometry
     *
     * @return a lineal geometry (which may be empty)
     * @see Geometry#getBoundary
     */
    // TODO: Implement ME!
    //   public Geometry getBoundary() {
    //     if (isEmpty()) {
    //       return getFactory().createMultiLineString();
    //     }
    //     LinearRing[] rings = new LinearRing[holes.length + 1];
    //     rings[0] = shell;
    //     for (int i = 0; i < holes.length; i++) {
    //       rings[i + 1] = holes[i];
    //     }
    //     // create LineString or MultiLineString as appropriate
    //     if (rings.length <= 1)
    //       return getFactory().createLinearRing(rings[0].getCoordinateSequence());
    //     return getFactory().createMultiLineString(rings);
    //   }

    pub fn compute_envelope_internal(&mut self) -> Envelope {
        return self.shell.get_envelope_internal();
    }

    pub fn equals_exact(&self, other: &Polygon, tolerance: f64) -> bool {
        let this_shell = self.shell.copy();
        let other_polygon_shell = other.shell.copy();
        if !this_shell.equals_exact(&other_polygon_shell, tolerance) {
            return false;
        }
        if self.holes.len() != other.holes.len() {
            return false;
        }
        for i in 0..self.holes.len() {
            if !self.holes[i].equals_exact(&other.holes[i], tolerance) {
                return false;
            }
        }
        return true;
    }

    // TODO: Implement ME!
    //   public void apply(CoordinateFilter filter) {
    // 	    shell.apply(filter);
    // 	    for (int i = 0; i < holes.length; i++) {
    // 	      holes[i].apply(filter);
    // 	    }
    // 	  }

    //   public void apply(CoordinateSequenceFilter filter)
    //   {
    // 	    shell.apply(filter);
    //       if (! filter.isDone()) {
    //         for (int i = 0; i < holes.length; i++) {
    //           holes[i].apply(filter);
    //           if (filter.isDone())
    //             break;
    //         }
    //       }
    //       if (filter.isGeometryChanged())
    //         geometryChanged();
    // 	  }

    //   public void apply(GeometryFilter filter) {
    //     filter.filter(this);
    //   }

    //   public void apply(GeometryComponentFilter filter) {
    //     filter.filter(this);
    //     shell.apply(filter);
    //     for (int i = 0; i < holes.length; i++) {
    //       holes[i].apply(filter);
    //     }
    //   }

    pub fn copy(&self) -> Polygon {
        self.copy_internal()
    }

    fn copy_internal(&self) -> Polygon {
        let shell_copy = self.shell.copy();
        let mut hole_copies = vec![];
        for i in 0..self.holes.len() {
            hole_copies.push(self.holes[i].copy());
        }
        return Polygon::new_with_linear_ring_vec(&shell_copy, &hole_copies);
    }

    // TODO: Implement ME!
    // pub fn convexHull(&self) -> LinearRing {
    //     return self.getExteriorRing().convexHull();
    // }

    pub fn self_normalize(&mut self) {
        self.shell = self.normalized(&self.shell, true);
        for i in 0..self.holes.len() {
            self.holes[i] = self.normalized(&self.holes[i], false);
        }
        // Arrays.sort(holes);
    }

    pub fn compare_to(&self, poly: &Polygon) -> i32 {
        let this_shell = self.shell.copy();
        let other_shell = poly.shell.copy();
        let shell_comp = this_shell.compare_to_same_class(&other_shell);
        if shell_comp != 0 {
            return shell_comp;
        }

        let n_hole1 = self.get_num_interior_ring();
        let n_hole2 = poly.get_num_interior_ring();
        let mut i = 0;
        while i < n_hole1 && i < n_hole2 {
            let this_hole = self.get_interior_ring_n(i);
            let other_hole = poly.get_interior_ring_n(i);
            let hole_comp = this_hole.compare_to_same_class(&other_hole);
            if hole_comp != 0 {
                return hole_comp;
            }
            i += 1;
        }
        if i < n_hole1 {
            return 1;
        }
        if i < n_hole2 {
            return -1;
        }
        return 0;
    }

    pub fn compare_to_with_comparator(
        &self,
        poly: &Polygon,
        comp: &CoordinateSequenceComparator,
    ) -> i32 {
        let this_shell = self.shell.copy();
        let other_shell = poly.shell.copy();
        let shell_comp = this_shell.compare_to_same_class_with_comparator(&other_shell, comp);
        if shell_comp != 0 {
            return shell_comp;
        }

        let n_hole1 = self.get_num_interior_ring();
        let n_hole2 = poly.get_num_interior_ring();
        let mut i = 0;
        while i < n_hole1 && i < n_hole2 {
            let this_hole = self.get_interior_ring_n(i);
            let other_hole = poly.get_interior_ring_n(i);
            let hole_comp = this_hole.compare_to_same_class_with_comparator(&other_hole, comp);
            if hole_comp != 0 {
                return hole_comp;
            }
            i += 1;
        }
        if i < n_hole1 {
            return 1;
        }
        if i < n_hole2 {
            return -1;
        }
        return 0;
    }

    pub fn get_type_code() -> i32 {
        return Geometry::TYPECODE_POLYGON;
    }

    pub fn normalized(&self, ring: &LinearRing, clockwise: bool) -> LinearRing {
        let res = ring.copy();
        Polygon::normalize(&res, clockwise);
        return res;
    }

    pub fn normalize(ring: &LinearRing, clockwise: bool) {
        if ring.is_empty() {
            return;
        }

        let mut seq = ring.get_coordinate_array_sequence();
        let min_coordinate_index = CoordinateArraySequences::min_coordinate_index_sequence_from_to(
            &seq,
            0,
            seq.size() - 2,
        );
        CoordinateArraySequences::scroll_coordinate_index_ensure_ring(
            &mut seq,
            min_coordinate_index,
            true,
        );
        if Orientation::is_ccw_coordinate_array_sequence(&seq) == clockwise {
            CoordinateArraySequences::reverse(&mut seq);
        }
    }

    /**
     * Computes a new geometry which has all component coordinate sequences
     * in reverse order (opposite orientation) to this one.
     *
     * @return a reversed geometry
     */
    pub fn reverse(&self) -> Polygon {
        let mut res = self.reverse_internal();
        if self.envelope.is_some() {
            res.envelope = Some(self.envelope.unwrap().copy());
        }

        return res;
    }

    fn reverse_internal(&self) -> Polygon {
        let shell = self.get_exterior_ring().reverse();
        let mut holes: Vec<LinearRing> = vec![];
        for i in 0..self.get_num_interior_ring() {
            holes[i] = self.get_interior_ring_n(i).reverse();
        }

        return GeometryFactory::create_polygon_with_linear_ring_vec(&shell, &holes);
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

    /**
     * Computes the centroid of this <code>Geometry</code>.
     * The centroid
     * is equal to the centroid of the set of component Geometries of highest
     * dimension (since the lower-dimension geometries contribute zero
     * "weight" to the centroid).
     * <p>
     * The centroid of an empty geometry is <code>POINT EMPTY</code>.
     *
     * @return a {@link Point} which is the centroid of this Geometry
     */
    pub fn get_centroid(&self) -> Point {
        if self.is_empty() {
            return Point::default();
        }
        if let Some(mut cent_pt) = Centroid::get_centroid_from_polygon(self) {
            return self.create_point_from_internal_coord(&mut cent_pt);
        }
        return Point::default();
    }

    fn create_point_from_internal_coord(&self, coord: &mut Coordinate) -> Point {
        if let Some(mut precision_model) = self.precision_model {
            precision_model.make_precise_coordinate(coord);
        }
        return GeometryFactory::create_point_from_coordinate(coord);
    }
}
