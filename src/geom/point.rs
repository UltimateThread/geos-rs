use super::{coordinate::Coordinate, coordinate_sequence_comparator::CoordinateSequenceComparator, coordinates::Coordinates, dimension::Dimension, envelope::Envelope, geometry::Geometry, implementation::{coordinate_array_sequence::CoordinateArraySequence, coordinate_array_sequence_factory::CoordinateArraySequenceFactory}};



#[derive(Clone)]
pub struct Point {
    /**
     *  The <code>Coordinate</code> wrapped by this <code>Point</code>.
     */
    coordinates: CoordinateArraySequence,

    /**
     *  The bounding box of this <code>Geometry</code>.
     */
    envelope: Option<Envelope>,
}

impl Point {
    pub fn default() -> Self {
        let coords: Vec<Coordinate> = vec![];
        Self {
            coordinates: CoordinateArraySequenceFactory::create_from_coordinates(&coords),
            envelope: None,
        }
    }

    /**
     *@param  coordinates      contains the single coordinate on which to base this <code>Point</code>
     *      , or <code>null</code> to create the empty geometry.
     */
    pub fn new_with_coordinate_seq(coordinates: &CoordinateArraySequence) -> Self {
        assert!(coordinates.size() <= 1);

        Self {
            coordinates: coordinates.copy(),
            envelope: None,
        }
    }

    pub fn get_coordinates(&self) -> Vec<Coordinate> {
        if self.is_empty() {
            return vec![];
        } else {
            return vec![self.get_coordinate().unwrap()];
        }
    }

    pub fn get_num_points(&self) -> usize {
        if self.is_empty() {
            return 0;
        } else {
            return 1;
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.coordinates.size() == 0;
    }

    pub fn is_simple(&self) -> bool {
        return true;
    }

    pub fn get_dimension() -> i32 {
        return 0;
    }

    pub fn get_boundary_dimension() -> i32 {
        return Dimension::FALSE;
    }

    pub fn get_x(&self) -> Option<f64> {
        if self.get_coordinate().is_none() {
            return None;
        }
        return Some(self.get_coordinate().unwrap().x);
    }

    pub fn get_y(&self) -> Option<f64> {
        if self.get_coordinate().is_none() {
            return None;
        }
        return Some(self.get_coordinate().unwrap().y);
    }

    pub fn get_coordinate(&self) -> Option<Coordinate> {
        if self.coordinates.size() != 0 {
            return Some(self.coordinates.get_coordinate_index(0));
        } else {
            return None;
        }
    }

    pub fn get_geometry_type(&self) -> &'static str {
        return Geometry::TYPENAME_POINT;
    }

    // /**
    //  * Gets the boundary of this geometry.
    //  * Zero-dimensional geometries have no boundary by definition,
    //  * so an empty GeometryCollection is returned.
    //  *
    //  * @return an empty GeometryCollection
    //  * @see Geometry#getBoundary
    //  */
    // pub fn get_boundary() -> Geometry {
    //     return GeometryCollection::default();
    // }

    pub fn compute_envelope_internal(&self) -> Envelope {
        if self.is_empty() {
            return Envelope::default();
        }

        let mut env = Envelope::default();
        env.expand_to_include_xy(self.coordinates.get_x(0), self.coordinates.get_y(0));
        return env;
    }

    pub fn equals_exact(&self, other: &Point, tolerance: f64) -> bool {
        if self.is_empty() && other.is_empty() {
            return true;
        }
        if self.is_empty() != other.is_empty() {
            return false;
        }

        let coordinate = self.get_coordinate();
        let other_coordinate = other.get_coordinate();

        if coordinate.is_none() && other_coordinate.is_none() {
            return true;
        }
        if coordinate.is_some() && other_coordinate.is_none() {
            return false;
        }
        if coordinate.is_none() && other_coordinate.is_some() {
            return false;
        }

        return Coordinates::equal(&other_coordinate.unwrap(), &coordinate.unwrap(), tolerance);
    }

    // pub fn apply_coordinate_filter(&self, filter: &CoordinateFilter) {
    //     if self.is_empty() {
    //         return;
    //     }
    //     filter.filter(self.get_coordinate());
    // }

    // pub fn apply_coordinate_seq_filter(&self, filter: &CoordinateSequenceFilter) {
    //     if self.is_empty() {
    //         return;
    //     }

    //     filter.filter(self.coordinates, 0);

    //     if filter.isGeometryChanged() {
    //         self.geometry_changed();
    //     }
    // }

    // /**
    //  * Notifies this geometry that its coordinates have been changed by an external
    //  * party (for example, via a {@link CoordinateFilter}).
    //  * When this method is called the geometry will flush
    //  * and/or update any derived information it has cached (such as its {@link Envelope} ).
    //  * The operation is applied to all component Geometries.
    //  */
    // pub fn geometry_changed(&self) {
    //     self.apply_geometry_component_filter(geometryChangedFilter);
    // }

    // pub fn apply_geometry_filter(&self, filter: &GeometryFilter) {
    //     filter.filter(self);
    // }

    // pub fn apply_geometry_component_filter(&self, filter: &GeometryComponentFilter) {
    //     filter.filter(self);
    // }

    pub fn copy_internal(&self) -> Point {
        return Point::new_with_coordinate_seq(&self.coordinates.copy());
    }

    pub fn reverse(&self) -> Point {
        let mut res = self.reverse_internal();
        if self.envelope.is_some() {
            res.envelope = Some(self.envelope.unwrap().copy());
        }

        return res;
    }

    pub fn reverse_internal(&self) -> Point {
        return Point::new_with_coordinate_seq(&self.coordinates.copy());
    }

    pub fn compare_to_same_class(&self, other: &Point, comp: &CoordinateSequenceComparator) -> i32 {
        return comp.compare_coordinate_array_sequence(&self.coordinates, &other.coordinates);
    }

    pub fn get_type_code() -> i32 {
        return Geometry::TYPECODE_POINT;
    }

    pub fn get_coordinate_sequence(&self) -> CoordinateArraySequence {
        return self.coordinates.copy();
    }
}
