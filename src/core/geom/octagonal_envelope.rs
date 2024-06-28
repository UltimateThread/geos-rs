use super::{
    coordinate::Coordinate, envelope::Envelope,
    implementation::coordinate_array_sequence::CoordinateArraySequence, line_string::LineString,
};

#[derive(Clone, Copy)]
pub struct OctagonalEnvelope {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    min_a: f64,
    max_a: f64,
    min_b: f64,
    max_b: f64,
}

impl OctagonalEnvelope {
    /**
     * Gets the octagonal envelope of a geometry
     * @param geom the geometry
     * @return the octagonal envelope of the geometry
     */
    //   pub fn octagonalEnvelope(geom: Polygon) -> Polygon {
    //     return (new OctagonalEnvelope(geom)).toGeometry(geom.getFactory());
    //   }

    fn compute_a(x: f64, y: f64) -> f64 {
        return x + y;
    }

    fn compute_b(x: f64, y: f64) -> f64 {
        return x - y;
    }

    /**
     * Creates a new null bounding octagon
     */
    pub fn default() -> Self {
        Self {
            min_x: f64::NAN,
            max_x: 0.0,
            min_y: 0.0,
            max_y: 0.0,
            min_a: 0.0,
            max_a: 0.0,
            min_b: 0.0,
            max_b: 0.0,
        }
    }

    /**
     * Creates a new null bounding octagon bounding a {@link Coordinate}
     *
     * @param p the coordinate to bound
     */
    pub fn new_with_coordinate(p: &Coordinate) -> Self {
        let mut new = OctagonalEnvelope::default();
        new.expand_to_include_coordinate(p);
        new
    }

    /**
     * Creates a new null bounding octagon bounding a pair of {@link Coordinate}s
     *
     * @param p0 a coordinate to bound
     * @param p1 a coordinate to bound
     */
    pub fn new_with_coordinates(p0: &Coordinate, p1: &Coordinate) -> Self {
        let mut new = OctagonalEnvelope::default();
        new.expand_to_include_coordinate(p0);
        new.expand_to_include_coordinate(p1);
        new
    }

    /**
     * Creates a new null bounding octagon bounding an {@link Envelope}
     */
    pub fn new_with_envelope(env: &Envelope) -> Self {
        let mut new = OctagonalEnvelope::default();
        new.expand_to_include_envelope(env);
        new
    }

    /**
     * Creates a new null bounding octagon bounding an {@link OctagonalEnvelope}
     * (the copy constructor).
     */
    pub fn new_with_octagonal_envelope(oct: &OctagonalEnvelope) -> Self {
        let mut new = OctagonalEnvelope::default();
        new.expand_to_include_octagonal_envelope(oct);
        new
    }

    /**
     * Creates a new null bounding octagon bounding a {@link Geometry}
     */
    pub fn new_with_line_string(geom: &LineString) -> Self {
        let mut new = OctagonalEnvelope::default();
        new.expand_to_include_coordinate_array_sequence(&geom.get_coordinate_sequence());
        new
    }

    pub fn get_min_x(&self) -> f64 {
        return self.min_x;
    }
    pub fn get_max_x(&self) -> f64 {
        return self.max_x;
    }
    pub fn get_min_y(&self) -> f64 {
        return self.min_y;
    }
    pub fn get_max_y(&self) -> f64 {
        return self.max_y;
    }
    pub fn get_min_a(&self) -> f64 {
        return self.min_a;
    }
    pub fn get_max_a(&self) -> f64 {
        return self.max_a;
    }
    pub fn get_min_b(&self) -> f64 {
        return self.min_b;
    }
    pub fn get_max_b(&self) -> f64 {
        return self.max_b;
    }

    pub fn is_null(&self) -> bool {
        return f64::is_nan(self.min_x);
    }

    /**
     *  Sets the value of this object to the null value
     */
    pub fn set_to_null(&mut self) {
        self.min_x = f64::NAN;
    }

    pub fn expand_to_include_coordinate_array_sequence(&mut self, seq: &CoordinateArraySequence) {
        for i in 0..seq.size() {
            let x = seq.get_x(i);
            let y = seq.get_y(i);
            self.expand_to_include_xy(x, y);
        }
    }

    pub fn expand_to_include_octagonal_envelope(&mut self, oct: &OctagonalEnvelope) {
        if oct.is_null() {
            return;
        }

        if self.is_null() {
            self.min_x = oct.min_x;
            self.max_x = oct.max_x;
            self.min_y = oct.min_y;
            self.max_y = oct.max_y;
            self.min_a = oct.min_a;
            self.max_a = oct.max_a;
            self.min_b = oct.min_b;
            self.max_b = oct.max_b;
            return;
        }
        if oct.min_x < self.min_x {
            self.min_x = oct.min_x;
        }
        if oct.max_x > self.max_x {
            self.max_x = oct.max_x;
        }
        if oct.min_y < self.min_y {
            self.min_y = oct.min_y;
        }
        if oct.max_y > self.max_y {
            self.max_y = oct.max_y;
        }
        if oct.min_a < self.min_a {
            self.min_a = oct.min_a;
        }
        if oct.max_a > self.max_a {
            self.max_a = oct.max_a;
        }
        if oct.min_b < self.min_b {
            self.min_b = oct.min_b;
        }
        if oct.max_b > self.max_b {
            self.max_b = oct.max_b;
        }
        return;
    }

    pub fn expand_to_include_coordinate(&mut self, p: &Coordinate) {
        self.expand_to_include_xy(p.x, p.y);
    }

    pub fn expand_to_include_envelope(&mut self, env: &Envelope) {
        self.expand_to_include_xy(env.get_min_x(), env.get_min_y());
        self.expand_to_include_xy(env.get_min_x(), env.get_max_y());
        self.expand_to_include_xy(env.get_max_x(), env.get_min_y());
        self.expand_to_include_xy(env.get_max_x(), env.get_max_y());
    }

    pub fn expand_to_include_xy(&mut self, x: f64, y: f64) {
        let a = OctagonalEnvelope::compute_a(x, y);
        let b = OctagonalEnvelope::compute_b(x, y);

        if self.is_null() {
            self.min_x = x;
            self.max_x = x;
            self.min_y = y;
            self.max_y = y;
            self.min_a = a;
            self.max_a = a;
            self.min_b = b;
            self.max_b = b;
        } else {
            if x < self.min_x {
                self.min_x = x;
            }
            if x > self.max_x {
                self.max_x = x;
            }
            if y < self.min_y {
                self.min_y = y;
            }
            if y > self.max_y {
                self.max_y = y;
            }
            if a < self.min_a {
                self.min_a = a;
            }
            if a > self.max_a {
                self.max_a = a;
            }
            if b < self.min_b {
                self.min_b = b;
            }
            if b > self.max_b {
                self.max_b = b;
            }
        }
    }

    pub fn expand_by_distance(&mut self, distance: f64) {
        if self.is_null() {
            return;
        }

        let diagonal_distance = f64::sqrt(2.) * distance;

        self.min_x -= distance;
        self.max_x += distance;
        self.min_y -= distance;
        self.max_y += distance;
        self.min_a -= diagonal_distance;
        self.max_a += diagonal_distance;
        self.min_b -= diagonal_distance;
        self.max_b += diagonal_distance;

        if !self.is_valid() {
            self.set_to_null();
        }
    }

    /**
     * Tests if the extremal values for this octagon are valid.
     *
     * @return <code>true</code> if this object has valid values
     */
    pub fn is_valid(&self) -> bool {
        if self.is_null() {
            return true;
        }
        return self.min_x <= self.max_x
            && self.min_y <= self.max_y
            && self.min_a <= self.max_a
            && self.min_b <= self.max_b;
    }

    pub fn intersects_octagonal_envelope(&self, other: &OctagonalEnvelope) -> bool {
        if self.is_null() || other.is_null() {
            return false;
        }

        if self.min_x > other.max_x {
            return false;
        }
        if self.max_x < other.min_x {
            return false;
        }
        if self.min_y > other.max_y {
            return false;
        }
        if self.max_y < other.min_y {
            return false;
        }
        if self.min_a > other.max_a {
            return false;
        }
        if self.max_a < other.min_a {
            return false;
        }
        if self.min_b > other.max_b {
            return false;
        }
        if self.max_b < other.min_b {
            return false;
        }
        return true;
    }

    pub fn intersects_coordinate(&self, p: &Coordinate) -> bool {
        if self.min_x > p.x {
            return false;
        }
        if self.max_x < p.x {
            return false;
        }
        if self.min_y > p.y {
            return false;
        }
        if self.max_y < p.y {
            return false;
        }

        let a = OctagonalEnvelope::compute_a(p.x, p.y);
        let b = OctagonalEnvelope::compute_b(p.x, p.y);
        if self.min_a > a {
            return false;
        }
        if self.max_a < a {
            return false;
        }
        if self.min_b > b {
            return false;
        }
        if self.max_b < b {
            return false;
        }
        return true;
    }

    pub fn contains_octagonal_envelope(&self, other: &OctagonalEnvelope) -> bool {
        if self.is_null() || other.is_null() {
            return false;
        }

        return other.min_x >= self.min_x
            && other.max_x <= self.max_x
            && other.min_y >= self.min_y
            && other.max_y <= self.max_y
            && other.min_a >= self.min_a
            && other.max_a <= self.max_a
            && other.min_b >= self.min_b
            && other.max_b <= self.max_b;
    }

    // TODO: Implement ME!
    //   public Geometry toGeometry(GeometryFactory geomFactory)
    //   {
    //     if (isNull()) {
    //       return geomFactory.createPoint();
    //     }

    //     Coordinate px00 = new Coordinate(minX, minA - minX);
    //     Coordinate px01 = new Coordinate(minX, minX - minB);

    //     Coordinate px10 = new Coordinate(maxX, maxX - maxB);
    //     Coordinate px11 = new Coordinate(maxX, maxA - maxX);

    //     Coordinate py00 = new Coordinate(minA - minY, minY);
    //     Coordinate py01 = new Coordinate(minY + maxB, minY);

    //     Coordinate py10 = new Coordinate(maxY + minB, maxY);
    //     Coordinate py11 = new Coordinate(maxA - maxY, maxY);

    //     PrecisionModel pm = geomFactory.getPrecisionModel();
    //     pm.makePrecise(px00);
    //     pm.makePrecise(px01);
    //     pm.makePrecise(px10);
    //     pm.makePrecise(px11);
    //     pm.makePrecise(py00);
    //     pm.makePrecise(py01);
    //     pm.makePrecise(py10);
    //     pm.makePrecise(py11);

    //     CoordinateList coordList = new CoordinateList();
    //     coordList.add(px00, false);
    //     coordList.add(px01, false);
    //     coordList.add(py10, false);
    //     coordList.add(py11, false);
    //     coordList.add(px11, false);
    //     coordList.add(px10, false);
    //     coordList.add(py01, false);
    //     coordList.add(py00, false);

    //     if (coordList.size() == 1) {
    //       return geomFactory.createPoint(px00);
    //     }
    //     if (coordList.size() == 2) {
    //       Coordinate[] pts = coordList.toCoordinateArray();
    //       return geomFactory.createLineString(pts);
    //     }
    //     // must be a polygon, so add closing point
    //     coordList.add(px00, false);
    //     Coordinate[] pts = coordList.toCoordinateArray();
    //     return geomFactory.createPolygon(geomFactory.createLinearRing(pts));
    //   }

    //   private static class BoundingOctagonComponentFilter
    //   implements GeometryComponentFilter
    //   {
    //     OctagonalEnvelope oe;

    //     BoundingOctagonComponentFilter(OctagonalEnvelope oe) {
    //       this.oe = oe;
    //     }

    //      public void filter(Geometry geom)
    //      {
    //        if (geom instanceof LineString) {
    //          oe.expandToInclude( ((LineString) geom).getCoordinateSequence());
    //        }
    //        else if (geom instanceof Point) {
    //          oe.expandToInclude( ((Point) geom).getCoordinateSequence());
    //        }
    //      }
    //   }
}
