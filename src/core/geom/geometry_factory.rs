// use super::{coordinate::Coordinate, envelope::Envelope, implementation::{coordinate_array_sequence::CoordinateArraySequence, coordinate_array_sequence_factory::CoordinateArraySequenceFactory}, point::Point, precision_model::PrecisionModel};

use super::{
    coordinate::Coordinate, coordinate_array_sequences::CoordinateArraySequences, implementation::{
        coordinate_array_sequence::CoordinateArraySequence,
        coordinate_array_sequence_factory::CoordinateArraySequenceFactory,
    }, line_string::LineString, linear_ring::LinearRing, multi_point::MultiPoint, multi_polygon::MultiPolygon, point::Point, polygon::Polygon, precision_model::PrecisionModel
};

#[derive(Clone, Copy)]
pub struct GeometryFactory {
    srid: i32,
    precision_model: PrecisionModel,
}

impl GeometryFactory {
    // /**
    //    * Constructs a GeometryFactory that generates Geometries having the given
    //    * PrecisionModel, spatial-reference ID, and CoordinateSequence implementation.
    //    */
    //   pub fn new_with_precision_model_srid(precision_model: PrecisionModel, srid: i32) -> Self {
    //         Self {
    //             srid,
    //             precision_model,
    //         }
    // }

    // /**
    // * Constructs a GeometryFactory that generates Geometries having the given
    // * CoordinateSequence implementation, a double-precision floating PrecisionModel and a
    // * spatial-reference ID of 0.
    // */
    // pub fn default() -> Self {
    //     Self {
    //         srid: 0,
    //         precision_model: PrecisionModel::default(),
    //     }
    // }

    // /**
    // * Constructs a GeometryFactory that generates Geometries having the given
    // * {@link PrecisionModel} and the default CoordinateSequence
    // * implementation.
    // *
    // * @param precisionModel the PrecisionModel to use
    // */
    // pub fn new_with_precision_model (precision_model: PrecisionModel) -> Self {
    //     Self {
    //         srid: 0,
    //         precision_model,
    //     }
    // }

    // /**
    // * Creates a {@link Geometry} with the same extent as the given envelope.
    // * The Geometry returned is guaranteed to be valid.
    // * To provide this behaviour, the following cases occur:
    // * <p>
    // * If the <code>Envelope</code> is:
    // * <ul>
    // * <li>null : returns an empty {@link Point}
    // * <li>a point : returns a non-empty {@link Point}
    // * <li>a line : returns a two-point {@link LineString}
    // * <li>a rectangle : returns a {@link Polygon} whose points are (minx, miny),
    // *  (minx, maxy), (maxx, maxy), (maxx, miny), (minx, miny).
    // * </ul>
    // *
    // *@param  envelope the <code>Envelope</code> to convert
    // *@return an empty <code>Point</code> (for null <code>Envelope</code>s),
    // *	a <code>Point</code> (when min x = max x and min y = max y) or a
    // *      <code>Polygon</code> (in all other cases)
    // */
    // pub fn envelope_to_point(envelope: &Envelope) -> Point {
    //     // null envelope - return empty point geometry
    //     if envelope.is_null() {
    //         return GeometryFactory::createPoint();
    //     }

    //     // point?
    //     if envelope.get_min_x() == envelope.get_max_x() && envelope.get_min_y() == envelope.get_max_y() {
    //         return GeometryFactory::createPointFromCoordinate(&Coordinate::new_xy(envelope.get_min_x(), envelope.get_min_y()));
    //     } else {
    //         return GeometryFactory::createPoint();
    //     }
    // }

    // pub fn envelope_to_line_string(envelope: &Envelope) -> LineString {
    //     // vertical or horizontal line?
    //     if envelope.get_min_x() == envelope.get_max_x() || envelope.get_min_y() == envelope.get_max_y() {
    //         let coords: Vec<Coordinate> = vec![
    //             Coordinate::new_xy(envelope.get_min_x(), envelope.get_min_y()),
    //             Coordinate::new_xy(envelope.get_max_x(), envelope.get_max_y())
    //         ];
    //         return createLineString(coords);
    //     }
    // }

    // pub fn toPolygon(envelope: &Envelope) -> Polygon {
    //     let coords: Vec<Coordinate> = vec![
    //         Coordinate::new_xy(envelope.get_min_x(), envelope.get_min_y()),
    //         Coordinate::new_xy(envelope.get_min_x(), envelope.get_max_y()),
    //         Coordinate::new_xy(envelope.get_max_x(), envelope.get_max_y()),
    //         Coordinate::new_xy(envelope.get_max_x(), envelope.get_min_y()),
    //         Coordinate::new_xy(envelope.get_min_x(), envelope.get_min_y())
    //     ];

    //     // create a CW ring for the polygon
    //     return createPolygon(createLinearRing(coords), None);
    // }

    // /**
    // * Returns the PrecisionModel that Geometries created by this factory
    // * will be associated with.
    // *
    // * @return the PrecisionModel for this factory
    // */
    // pub fn getPrecisionModel(&self) -> PrecisionModel {
    //     return self.precision_model;
    // }

    /**
     * Constructs an empty {@link Point} geometry.
     *
     * @return an empty Point
     */
    pub fn create_point() -> Point {
        let coords: Vec<Coordinate> = vec![];
        let coord_seq = CoordinateArraySequenceFactory::create_from_coordinates(&coords);
        return GeometryFactory::create_point_from_coordinate_array_sequence(&coord_seq);
    }

    /**
     * Creates a Point using the given Coordinate.
     * A null Coordinate creates an empty Geometry.
     *
     * @param coordinate a Coordinate, or null
     * @return the created Point
     */
    pub fn create_point_from_coordinate(coordinate: &Coordinate) -> Point {
        let coords: Vec<Coordinate> = vec![Coordinate::from_coordinate(&coordinate)];
        return GeometryFactory::create_point_from_coordinate_array_sequence(
            &CoordinateArraySequenceFactory::create_from_coordinates(&coords),
        );
    }

    /**
     * Creates a Point using the given CoordinateSequence; a null or empty
     * CoordinateSequence will create an empty Point.
     *
     * @param coordinates a CoordinateSequence (possibly empty), or null
     * @return the created Point
     */
    pub fn create_point_from_coordinate_array_sequence(
        coordinates: &CoordinateArraySequence,
    ) -> Point {
        return Point::new_with_coordinate_seq(coordinates);
    }

    // /**
    // * Constructs an empty {@link MultiLineString} geometry.
    // *
    // * @return an empty MultiLineString
    // */
    // public MultiLineString createMultiLineString() {
    // return new MultiLineString(null, this);
    // }

    // /**
    // * Creates a MultiLineString using the given LineStrings; a null or empty
    // * array will create an empty MultiLineString.
    // *
    // * @param lineStrings LineStrings, each of which may be empty but not null
    // * @return the created MultiLineString
    // */
    // public MultiLineString createMultiLineString(LineString[] lineStrings) {
    // return new MultiLineString(lineStrings, this);
    // }

    // /**
    // * Constructs an empty {@link GeometryCollection} geometry.
    // *
    // * @return an empty GeometryCollection
    // */
    // public GeometryCollection createGeometryCollection() {
    // return new GeometryCollection(null, this);
    // }

    // /**
    // * Creates a GeometryCollection using the given Geometries; a null or empty
    // * array will create an empty GeometryCollection.
    // *
    // * @param geometries an array of Geometries, each of which may be empty but not null, or null
    // * @return the created GeometryCollection
    // */
    // public GeometryCollection createGeometryCollection(Geometry[] geometries) {
    // return new GeometryCollection(geometries, this);
    // }

    // /**
    // * Constructs an empty {@link MultiPolygon} geometry.
    // *
    // * @return an empty MultiPolygon
    // */
    // pub fn createMultiPolygon() -> MultiPolygon {
    // return new MultiPolygon(null, this);
    // }

    /**
    * Creates a MultiPolygon using the given Polygons; a null or empty array
    * will create an empty Polygon. The polygons must conform to the
    * assertions specified in the <A
    * HREF="http://www.opengis.org/techno/specs.htm">OpenGIS Simple Features
    * Specification for SQL</A>.
    *
    * @param polygons
    *            Polygons, each of which may be empty but not null
    * @return the created MultiPolygon
    */
    pub fn create_multi_polygon(polygons: &Vec<Polygon>) -> MultiPolygon {
        return MultiPolygon::new_with_polygons(polygons);
    }

    /**
     * Constructs an empty {@link LinearRing} geometry.
     *
     * @return an empty LinearRing
     */
    pub fn create_linear_ring() -> LinearRing {
        let coords: Vec<Coordinate> = vec![];
        return GeometryFactory::create_linear_ring_with_coordinate_array_sequence(
            &CoordinateArraySequenceFactory::create_from_coordinates(&coords),
        );
    }

    /**
     * Creates a {@link LinearRing} using the given {@link Coordinate}s.
     * A null or empty array creates an empty LinearRing.
     * The points must form a closed and simple linestring.
     * @param coordinates an array without null elements, or an empty array, or null
     * @return the created LinearRing
     * @throws IllegalArgumentException if the ring is not closed, or has too few points
     */
    pub fn create_linear_ring_with_coordinates(coordinates: &Vec<Coordinate>) -> LinearRing {
        return GeometryFactory::create_linear_ring_with_coordinate_array_sequence(
            &CoordinateArraySequenceFactory::create_from_coordinates(coordinates),
        );
    }

    /**
     * Creates a {@link LinearRing} using the given {@link CoordinateSequence}.
     * A null or empty array creates an empty LinearRing.
     * The points must form a closed and simple linestring.
     *
     * @param coordinates a CoordinateSequence (possibly empty), or null
     * @return the created LinearRing
     * @throws IllegalArgumentException if the ring is not closed, or has too few points
     */
    pub fn create_linear_ring_with_coordinate_array_sequence(
        coordinates: &CoordinateArraySequence,
    ) -> LinearRing {
        return LinearRing::new_with_coordinate_array_sequence(coordinates);
    }

    // /**
    // * Constructs an empty {@link MultiPoint} geometry.
    // *
    // * @return an empty MultiPoint
    // */
    // public MultiPoint createMultiPoint() {
    // return new MultiPoint(null, this);
    // }

    /**
     * Creates a {@link MultiPoint} using the given {@link Point}s.
     * A null or empty array will create an empty MultiPoint.
     *
     * @param point an array of Points (without null elements), or an empty array, or <code>null</code>
     * @return a MultiPoint object
     */
    pub fn create_multi_point_from_points(point: &Vec<Point>) -> MultiPoint {
        return MultiPoint::new_with_points(point);
    }

    /**
     * Creates a {@link MultiPoint} using the given {@link Coordinate}s.
     * A null or empty array will create an empty MultiPoint.
     *
     * @param coordinates an array (without null elements), or an empty array, or <code>null</code>
     * @return a MultiPoint object
     * @deprecated Use {@link GeometryFactory#createMultiPointFromCoords} instead
     */
    pub fn create_multi_point_with_coordinates(coordinates: &Vec<Coordinate>) -> MultiPoint {
        return GeometryFactory::create_multi_point_from_coordinate_array_sequence(
            &CoordinateArraySequenceFactory::create_from_coordinates(&coordinates),
        );
    }

    // /**
    // * Creates a {@link MultiPoint} using the given {@link Coordinate}s.
    // * A null or empty array will create an empty MultiPoint.
    // *
    // * @param coordinates an array (without null elements), or an empty array, or <code>null</code>
    // * @return a MultiPoint object
    // */
    // public MultiPoint createMultiPointFromCoords(Coordinate[] coordinates) {
    // return createMultiPoint(coordinates != null
    //          ? getCoordinateSequenceFactory().create(coordinates)
    //          : null);
    // }

    /**
     * Creates a {@link MultiPoint} using the
     * points in the given {@link CoordinateSequence}.
     * A <code>null</code> or empty CoordinateSequence creates an empty MultiPoint.
     *
     * @param coordinates a CoordinateSequence (possibly empty), or <code>null</code>
     * @return a MultiPoint geometry
     */
    pub fn create_multi_point_from_coordinate_array_sequence(
        coordinates: &CoordinateArraySequence,
    ) -> MultiPoint {
        let mut points: Vec<Point> = vec![Point::default(); coordinates.size()];
        for i in 0..coordinates.size() {
            let mut pt_seq = CoordinateArraySequenceFactory::create_with_size_dimension_measures(
                1,
                coordinates.get_dimension(),
                coordinates.get_measures(),
            );
            CoordinateArraySequences::copy(coordinates, i, &mut pt_seq, 0, 1);
            points[i] = GeometryFactory::create_point_from_coordinate_array_sequence(&pt_seq);
        }
        return GeometryFactory::create_multi_point_from_points(&points);
    }

    /**
     * Constructs a <code>Polygon</code> with the given exterior boundary and
     * interior boundaries.
     *
     * @param shell
     *            the outer boundary of the new <code>Polygon</code>, or
     *            <code>null</code> or an empty <code>LinearRing</code> if
     *            the empty geometry is to be created.
     * @param holes
     *            the inner boundaries of the new <code>Polygon</code>, or
     *            <code>null</code> or empty <code>LinearRing</code> s if
     *            the empty geometry is to be created.
     * @throws IllegalArgumentException if a ring is invalid
     */
    pub fn create_polygon_with_linear_ring_vec(
        shell: &LinearRing,
        holes: &Vec<LinearRing>,
    ) -> Polygon {
        return Polygon::new_with_linear_ring_vec(shell, holes);
    }

    /**
     * Constructs a <code>Polygon</code> with the given exterior boundary.
     *
     * @param shell
     *            the outer boundary of the new <code>Polygon</code>, or
     *            <code>null</code> or an empty <code>LinearRing</code> if
     *            the empty geometry is to be created.
     * @throws IllegalArgumentException if the boundary ring is invalid
     */
    pub fn create_polygon_with_coordinate_array_sequence(
        shell: &CoordinateArraySequence,
    ) -> Polygon {
        return GeometryFactory::create_polygon_with_linear_ring(
            &GeometryFactory::create_linear_ring_with_coordinate_array_sequence(shell),
        );
    }

    /**
     * Constructs a <code>Polygon</code> with the given exterior boundary.
     *
     * @param shell
     *            the outer boundary of the new <code>Polygon</code>, or
     *            <code>null</code> or an empty <code>LinearRing</code> if
     *            the empty geometry is to be created.
     * @throws IllegalArgumentException if the boundary ring is invalid
     */
    pub fn create_polygon_with_coordinates(shell: &Vec<Coordinate>) -> Polygon {
        return GeometryFactory::create_polygon_with_linear_ring(
            &GeometryFactory::create_linear_ring_with_coordinates(shell),
        );
    }

    /**
     * Constructs a <code>Polygon</code> with the given exterior boundary.
     *
     * @param shell
     *            the outer boundary of the new <code>Polygon</code>, or
     *            <code>null</code> or an empty <code>LinearRing</code> if
     *            the empty geometry is to be created.
     * @throws IllegalArgumentException if the boundary ring is invalid
     */
    pub fn create_polygon_with_linear_ring(shell: &LinearRing) -> Polygon {
        return Polygon::new_with_linear_ring(shell);
    }

    // /**
    // *  Build an appropriate <code>Geometry</code>, <code>MultiGeometry</code>, or
    // *  <code>GeometryCollection</code> to contain the <code>Geometry</code>s in
    // *  it.
    // * For example:<br>
    // *
    // *  <ul>
    // *    <li> If <code>geomList</code> contains a single <code>Polygon</code>,
    // *    the <code>Polygon</code> is returned.
    // *    <li> If <code>geomList</code> contains several <code>Polygon</code>s, a
    // *    <code>MultiPolygon</code> is returned.
    // *    <li> If <code>geomList</code> contains some <code>Polygon</code>s and
    // *    some <code>LineString</code>s, a <code>GeometryCollection</code> is
    // *    returned.
    // *    <li> If <code>geomList</code> is empty, an empty <code>GeometryCollection</code>
    // *    is returned
    // *  </ul>
    // *
    // * Note that this method does not "flatten" Geometries in the input, and hence if
    // * any MultiGeometries are contained in the input a GeometryCollection containing
    // * them will be returned.
    // *
    // *@param  geomList  the <code>Geometry</code>s to combine
    // *@return           a <code>Geometry</code> of the "smallest", "most
    // *      type-specific" class that can contain the elements of <code>geomList</code>
    // *      .
    // */
    // public Geometry buildGeometry(Collection geomList) {

    // /**
    // * Determine some facts about the geometries in the list
    // */
    // Class geomClass = null;
    // boolean isHeterogeneous = false;
    // boolean hasGeometryCollection = false;
    // for (Iterator i = geomList.iterator(); i.hasNext(); ) {
    // Geometry geom = (Geometry) i.next();
    // Class partClass = geom.getClass();
    // if (geomClass == null) {
    // geomClass = partClass;
    // }
    // if (partClass != geomClass) {
    // isHeterogeneous = true;
    // }
    // if (geom instanceof GeometryCollection)
    // hasGeometryCollection = true;
    // }

    // /**
    // * Now construct an appropriate geometry to return
    // */
    // // for the empty geometry, return an empty GeometryCollection
    // if (geomClass == null) {
    // return createGeometryCollection();
    // }
    // if (isHeterogeneous || hasGeometryCollection) {
    // return createGeometryCollection(toGeometryArray(geomList));
    // }
    // // at this point we know the collection is hetereogenous.
    // // Determine the type of the result from the first Geometry in the list
    // // this should always return a geometry, since otherwise an empty collection would have already been returned
    // Geometry geom0 = (Geometry) geomList.iterator().next();
    // boolean isCollection = geomList.size() > 1;
    // if (isCollection) {
    // if (geom0 instanceof Polygon) {
    // return createMultiPolygon(toPolygonArray(geomList));
    // }
    // else if (geom0 instanceof LineString) {
    // return createMultiLineString(toLineStringArray(geomList));
    // }
    // else if (geom0 instanceof Point) {
    // return createMultiPoint(toPointArray(geomList));
    // }
    // Assert.shouldNeverReachHere("Unhandled class: " + geom0.getClass().getName());
    // }
    // return geom0;
    // }

    /**
     * Constructs an empty {@link LineString} geometry.
     *
     * @return an empty LineString
     */
    pub fn create_line_string() -> LineString {
        let coords: Vec<Coordinate> = vec![];
        return GeometryFactory::create_line_string_coordinates(&coords);
    }

    /**
     * Creates a LineString using the given Coordinates.
     * A null or empty array creates an empty LineString.
     *
     * @param coordinates an array without null elements, or an empty array, or null
     */
    pub fn create_line_string_coordinates(coordinates: &Vec<Coordinate>) -> LineString {
        let coordinate_sequence =
            CoordinateArraySequenceFactory::create_from_coordinates(&coordinates);
        return GeometryFactory::create_line_string_coordinate_array_sequence(&coordinate_sequence);
    }
    /**
     * Creates a LineString using the given CoordinateSequence.
     * A null or empty CoordinateSequence creates an empty LineString.
     *
     * @param coordinates a CoordinateSequence (possibly empty), or null
     */
    pub fn create_line_string_coordinate_array_sequence(
        coordinates: &CoordinateArraySequence,
    ) -> LineString {
        return LineString::new_from_coordinate_sequence(
            CoordinateArraySequence::new_from_coordinate_array_sequence(coordinates),
        );
    }

    // /**
    // * Creates an empty atomic geometry of the given dimension.
    // * If passed a dimension of -1 will create an empty {@link GeometryCollection}.
    // *
    // * @param dimension the required dimension (-1, 0, 1 or 2)
    // * @return an empty atomic geometry of given dimension
    // */
    // public Geometry createEmpty(int dimension) {
    // switch (dimension) {
    // case -1: return createGeometryCollection();
    // case 0: return createPoint();
    // case 1: return createLineString();
    // case 2: return createPolygon();
    // default:
    // throw new IllegalArgumentException("Invalid dimension: " + dimension);
    // }
    // }

    // /**
    // * Creates a deep copy of the input {@link Geometry}.
    // * The {@link CoordinateSequenceFactory} defined for this factory
    // * is used to copy the {@link CoordinateSequence}s
    // * of the input geometry.
    // * <p>
    // * This is a convenient way to change the <tt>CoordinateSequence</tt>
    // * used to represent a geometry, or to change the
    // * factory used for a geometry.
    // * <p>
    // * {@link Geometry#copy()} can also be used to make a deep copy,
    // * but it does not allow changing the CoordinateSequence type.
    // *
    // * @return a deep copy of the input geometry, using the CoordinateSequence type of this factory
    // *
    // * @see Geometry#copy()
    // */
    // public Geometry createGeometry(Geometry g)
    // {
    // GeometryEditor editor = new GeometryEditor(this);
    // return editor.edit(g, new CoordSeqCloneOp(coordinateSequenceFactory));
    // }

    // private static class CoordSeqCloneOp extends GeometryEditor.CoordinateSequenceOperation {
    // CoordinateSequenceFactory coordinateSequenceFactory;
    // public CoordSeqCloneOp(CoordinateSequenceFactory coordinateSequenceFactory) {
    // this.coordinateSequenceFactory = coordinateSequenceFactory;
    // }
    // public CoordinateSequence edit(CoordinateSequence coordSeq, Geometry geometry) {
    // return coordinateSequenceFactory.create(coordSeq);
    // }
    // }

    // /**
    // * Gets the SRID value defined for this factory.
    // *
    // * @return the factory SRID value
    // */
    // public int getSRID() {
    // return SRID;
    // }
}
