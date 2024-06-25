use std::fmt;

use super::coordinate::Coordinate;

#[derive(Clone, Copy)]
pub struct Envelope {
    /**
     *  the minimum x-coordinate
     */
    minx: f64,

    /**
     *  the maximum x-coordinate
     */
    maxx: f64,

    /**
     *  the minimum y-coordinate
     */
    miny: f64,

    /**
     *  the maximum y-coordinate
     */
    maxy: f64,
}

impl Envelope {
    /**
     * Test the point q to see whether it intersects the Envelope defined by p1-p2
     * @param p1 one extremal point of the envelope
     * @param p2 another extremal point of the envelope
     * @param q the point to test for intersection
     * @return <code>true</code> if q intersects the envelope p1-p2
     */
    pub fn intersects_3(p1: &Coordinate, p2: &Coordinate, q: &Coordinate) -> bool {
        //OptimizeIt shows that Math#min and Math#max here are a bottleneck.
        //Replace with direct comparisons. [Jon Aquino]
        let mut qx_comp1 = p2.x;
        if p1.x < p2.x {
            qx_comp1 = p1.x;
        }

        let mut qx_comp2 = p2.x;
        if p1.x > p2.x {
            qx_comp2 = p1.x;
        }

        let mut qy_comp1 = p2.y;
        if p1.y < p2.y {
            qy_comp1 = p1.y;
        }

        let mut qy_comp2 = p2.y;
        if p1.y > p2.y {
            qy_comp2 = p1.y;
        }

        if ((q.x >= qx_comp1) && (q.x <= qx_comp2)) && ((q.y >= qy_comp1) && (q.y <= qy_comp2)) {
            return true;
        }
        return false;
    }

    /**
     * Tests whether the envelope defined by p1-p2
     * and the envelope defined by q1-q2
     * intersect.
     *
     * @param p1 one extremal point of the envelope P
     * @param p2 another extremal point of the envelope P
     * @param q1 one extremal point of the envelope Q
     * @param q2 another extremal point of the envelope Q
     * @return <code>true</code> if Q intersects P
     */
    pub fn intersects_4(
        p1: &Coordinate,
        p2: &Coordinate,
        q1: &Coordinate,
        q2: &Coordinate,
    ) -> bool {
        let mut minq = f64::min(q1.x, q2.x);
        let mut maxq = f64::max(q1.x, q2.x);
        let mut minp = f64::min(p1.x, p2.x);
        let mut maxp = f64::max(p1.x, p2.x);

        if minp > maxq {
            return false;
        }
        if maxp < minq {
            return false;
        }

        minq = f64::min(q1.y, q2.y);
        maxq = f64::max(q1.y, q2.y);
        minp = f64::min(p1.y, p2.y);
        maxp = f64::max(p1.y, p2.y);

        if minp > maxq {
            return false;
        }
        if maxp < minq {
            return false;
        }
        return true;
    }

    pub fn default() -> Self {
        Self {
            minx: 0.,
            maxx: -1.,
            miny: 0.,
            maxy: -1.,
        }
    }

    /**
     *  Creates an <code>Envelope</code> for a region defined by maximum and minimum values.
     *
     *@param  x1  the first x-value
     *@param  x2  the second x-value
     *@param  y1  the first y-value
     *@param  y2  the second y-value
     */
    pub fn new_xy(x1: f64, x2: f64, y1: f64, y2: f64) -> Self {
        let minx: f64;
        let maxx: f64;
        let miny: f64;
        let maxy: f64;

        if x1 < x2 {
            minx = x1;
            maxx = x2;
        } else {
            minx = x2;
            maxx = x1;
        }
        if y1 < y2 {
            miny = y1;
            maxy = y2;
        } else {
            miny = y2;
            maxy = y1;
        }

        Self {
            minx,
            maxx,
            miny,
            maxy,
        }
    }

    /**
     *  Creates an <code>Envelope</code> for a region defined by two Coordinates.
     *
     *@param  p1  the first Coordinate
     *@param  p2  the second Coordinate
     */
    pub fn new_coordinates(p1: &Coordinate, p2: &Coordinate) -> Self {
        let minx: f64;
        let maxx: f64;
        let miny: f64;
        let maxy: f64;

        if p1.x < p2.x {
            minx = p1.x;
            maxx = p2.x;
        } else {
            minx = p2.x;
            maxx = p1.x;
        }
        if p1.y < p2.y {
            miny = p1.y;
            maxy = p2.y;
        } else {
            miny = p2.y;
            maxy = p1.y;
        }

        Self {
            minx,
            maxx,
            miny,
            maxy,
        }
    }

    /**
     *  Creates an <code>Envelope</code> for a region defined by a single Coordinate.
     *
     *@param  p  the Coordinate
     */
    pub fn new_coordinate(p: &Coordinate) -> Self {
        Self {
            minx: p.x,
            maxx: p.x,
            miny: p.y,
            maxy: p.y,
        }
    }

    /**
     *  Create an <code>Envelope</code> from an existing Envelope.
     *
     *@param  env  the Envelope to initialize from
     */
    pub fn new_envelope(env: &Envelope) -> Self {
        Self {
            minx: env.minx,
            maxx: env.maxx,
            miny: env.miny,
            maxy: env.maxy,
        }
    }

    /**
     * Creates a copy of this envelope object.
     *
     * @return a copy of this envelope
     */
    pub fn copy(&self) -> Envelope {
        return Envelope::new_envelope(self);
    }

    /**
     *  Makes this <code>Envelope</code> a "null" envelope, that is, the envelope
     *  of the empty geometry.
     */
    pub fn set_to_null(&mut self) {
        self.minx = 0.;
        self.maxx = -1.;
        self.miny = 0.;
        self.maxy = -1.;
    }

    /**
     *  Returns <code>true</code> if this <code>Envelope</code> is a "null"
     *  envelope.
     *
     *@return    <code>true</code> if this <code>Envelope</code> is uninitialized
     *      or is the envelope of the empty geometry.
     */
    pub fn is_null(&self) -> bool {
        return self.maxx < self.minx;
    }

    /**
     *  Returns the difference between the maximum and minimum x values.
     *
     *@return    max x - min x, or 0 if this is a null <code>Envelope</code>
     */
    pub fn get_width(&self) -> f64 {
        if self.is_null() {
            return 0.;
        }
        return self.maxx - self.minx;
    }

    /**
     *  Returns the difference between the maximum and minimum y values.
     *
     *@return    max y - min y, or 0 if this is a null <code>Envelope</code>
     */
    pub fn get_height(&self) -> f64 {
        if self.is_null() {
            return 0.;
        }
        return self.maxy - self.miny;
    }

    /**
     * Gets the length of the diameter (diagonal) of the envelope.
     *
     * @return the diameter length
     */
    pub fn get_diameter(&self) -> f64 {
        if self.is_null() {
            return 0.;
        }
        let w = self.get_width();
        let h = self.get_height();
        return f64::hypot(w, h);
    }

    /**
     *  Returns the <code>Envelope</code>s minimum x-value. min x &gt; max x
     *  indicates that this is a null <code>Envelope</code>.
     *
     *@return    the minimum x-coordinate
     */
    pub fn get_min_x(&self) -> f64 {
        return self.minx;
    }

    /**
     *  Returns the <code>Envelope</code>s maximum x-value. min x &gt; max x
     *  indicates that this is a null <code>Envelope</code>.
     *
     *@return    the maximum x-coordinate
     */
    pub fn get_max_x(&self) -> f64 {
        return self.maxx;
    }

    /**
     *  Returns the <code>Envelope</code>s minimum y-value. min y &gt; max y
     *  indicates that this is a null <code>Envelope</code>.
     *
     *@return    the minimum y-coordinate
     */
    pub fn get_min_y(&self) -> f64 {
        return self.miny;
    }

    /**
     *  Returns the <code>Envelope</code>s maximum y-value. min y &gt; max y
     *  indicates that this is a null <code>Envelope</code>.
     *
     *@return    the maximum y-coordinate
     */
    pub fn get_max_y(&self) -> f64 {
        return self.maxy;
    }

    /**
     * Gets the area of this envelope.
     *
     * @return the area of the envelope
     * @return 0.0 if the envelope is null
     */
    pub fn get_area(&self) -> f64 {
        return self.get_width() * self.get_height();
    }

    /**
     * Gets the minimum extent of this envelope across both dimensions.
     *
     * @return the minimum extent of this envelope
     */
    pub fn min_extent(&self) -> f64 {
        if self.is_null() {
            return 0.0;
        }
        let w = self.get_width();
        let h = self.get_height();
        if w < h {
            return w;
        }
        return h;
    }

    /**
     * Gets the maximum extent of this envelope across both dimensions.
     *
     * @return the maximum extent of this envelope
     */
    pub fn max_extent(&self) -> f64 {
        if self.is_null() {
            return 0.0;
        }
        let w = self.get_width();
        let h = self.get_height();
        if w > h {
            return w;
        }
        return h;
    }

    /**
     *  Enlarges this <code>Envelope</code> so that it contains
     *  the given {@link Coordinate}.
     *  Has no effect if the point is already on or within the envelope.
     *
     *@param  p  the Coordinate to expand to include
     */
    pub fn expand_to_include_coordinate(&mut self, p: &Coordinate) {
        if self.is_null() {
            self.minx = p.x;
            self.maxx = p.x;
            self.miny = p.y;
            self.maxy = p.y;
        } else {
            if p.x < self.minx {
                self.minx = p.x;
            }
            if p.x > self.maxx {
                self.maxx = p.x;
            }
            if p.y < self.miny {
                self.miny = p.y;
            }
            if p.y > self.maxy {
                self.maxy = p.y;
            }
        }
    }

    /**
     * Expands this envelope by a given distance in all directions.
     * Both positive and negative distances are supported.
     *
     * @param distance the distance to expand the envelope
     */
    pub fn expand_by(&mut self, distance: f64) {
        if self.is_null() {
            return;
        }

        self.minx -= distance;
        self.maxx += distance;
        self.miny -= distance;
        self.maxy += distance;

        // check for envelope disappearing
        if self.minx > self.maxx || self.miny > self.maxy {
            self.set_to_null();
        }
    }

    /**
     * Expands this envelope by a given distance in all directions.
     * Both positive and negative distances are supported.
     *
     * @param deltaX the distance to expand the envelope along the the X axis
     * @param deltaY the distance to expand the envelope along the the Y axis
     */
    pub fn expand_by_delta_xy(&mut self, delta_x: f64, delta_y: f64) {
        if self.is_null() {
            return;
        }

        self.minx -= delta_x;
        self.maxx += delta_x;
        self.miny -= delta_y;
        self.maxy += delta_y;

        // check for envelope disappearing
        if self.minx > self.maxx || self.miny > self.maxy {
            self.set_to_null();
        }
    }

    /**
     *  Enlarges this <code>Envelope</code> so that it contains
     *  the given point.
     *  Has no effect if the point is already on or within the envelope.
     *
     *@param  x  the value to lower the minimum x to or to raise the maximum x to
     *@param  y  the value to lower the minimum y to or to raise the maximum y to
     */
    pub fn expand_to_include_xy(&mut self, x: f64, y: f64) {
        if self.is_null() {
            self.minx = x;
            self.maxx = x;
            self.miny = y;
            self.maxy = y;
        } else {
            if x < self.minx {
                self.minx = x;
            }
            if x > self.maxx {
                self.maxx = x;
            }
            if y < self.miny {
                self.miny = y;
            }
            if y > self.maxy {
                self.maxy = y;
            }
        }
    }

    /**
     *  Enlarges this <code>Envelope</code> so that it contains
     *  the <code>other</code> Envelope.
     *  Has no effect if <code>other</code> is wholly on or
     *  within the envelope.
     *
     *@param  other  the <code>Envelope</code> to expand to include
     */
    pub fn expand_to_include_envelope(&mut self, other: &Envelope) {
        if other.is_null() {
            return;
        }
        if self.is_null() {
            self.minx = other.get_min_x();
            self.maxx = other.get_max_x();
            self.miny = other.get_min_y();
            self.maxy = other.get_max_y();
        } else {
            if other.minx < self.minx {
                self.minx = other.minx;
            }
            if other.maxx > self.maxx {
                self.maxx = other.maxx;
            }
            if other.miny < self.miny {
                self.miny = other.miny;
            }
            if other.maxy > self.maxy {
                self.maxy = other.maxy;
            }
        }
    }

    /**
     * Translates this envelope by given amounts in the X and Y direction.
     *
     * @param transX the amount to translate along the X axis
     * @param transY the amount to translate along the Y axis
     */
    pub fn translate(&mut self, trans_x: f64, trans_y: f64) {
        if self.is_null() {
            return;
        }

        let x1 = self.get_min_x() + trans_x;
        let x2 = self.get_max_x() + trans_x;
        let y1 = self.get_min_y() + trans_y;
        let y2 = self.get_max_y() + trans_y;

        let minx: f64;
        let maxx: f64;
        let miny: f64;
        let maxy: f64;

        if x1 < x2 {
            minx = x1;
            maxx = x2;
        } else {
            minx = x2;
            maxx = x1;
        }
        if y1 < y2 {
            miny = y1;
            maxy = y2;
        } else {
            miny = y2;
            maxy = y1;
        }

        self.minx = minx;
        self.maxx = maxx;
        self.miny = miny;
        self.maxy = maxy;
    }

    /**
     * Computes the coordinate of the centre of this envelope (as long as it is non-null
     *
     * @return the centre coordinate of this envelope
     * <code>null</code> if the envelope is null
     */
    pub fn centre(&self) -> Option<Coordinate> {
        if self.is_null() {
            return None;
        }

        return Some(Coordinate::new_coordinatexy(
            (self.get_min_x() + self.get_max_x()) / 2.0,
            (self.get_min_y() + self.get_max_y()) / 2.0,
        ));
    }

    /**
     * Computes the intersection of two {@link Envelope}s.
     *
     * @param env the envelope to intersect with
     * @return a new Envelope representing the intersection of the envelopes (this will be
     * the null envelope if either argument is null, or they do not intersect
     */
    pub fn intersection_envelope(&self, env: &Envelope) -> Envelope {
        if self.is_null() || env.is_null() || !self.intersects_envelope(env) {
            return Envelope::default();
        }

        let mut int_min_x = env.minx;
        if self.minx > env.minx {
            int_min_x = self.minx;
        }

        let mut int_min_y = env.miny;
        if self.miny > env.miny {
            int_min_y = self.miny;
        }

        let mut int_max_x = env.maxx;
        if self.maxx < env.maxx {
            int_max_x = self.maxx;
        }

        let mut int_max_y = env.maxy;
        if self.maxy < env.maxy {
            int_max_y = self.maxy;
        }

        return Envelope::new_xy(int_min_x, int_max_x, int_min_y, int_max_y);
    }

    /**
     * Tests if the region defined by <code>other</code>
     * intersects the region of this <code>Envelope</code>.
     * <p>
     * A null envelope never intersects.
     *
     *@param  other  the <code>Envelope</code> which this <code>Envelope</code> is
     *          being checked for intersecting
     *@return        <code>true</code> if the <code>Envelope</code>s intersect
     */
    pub fn intersects_envelope(&self, other: &Envelope) -> bool {
        if self.is_null() || other.is_null() {
            return false;
        }
        return !(other.minx > self.maxx
            || other.maxx < self.minx
            || other.miny > self.maxy
            || other.maxy < self.miny);
    }

    /**
     * Tests if the extent defined by two extremal points
     * intersects the extent of this <code>Envelope</code>.
     *
     *@param a a point
     *@param b another point
     *@return   <code>true</code> if the extents intersect
     */
    pub fn intersects_coordinate_ab(&self, a: &Coordinate, b: &Coordinate) -> bool {
        if self.is_null() {
            return false;
        }

        let mut envminx = b.x;
        if a.x < b.x {
            envminx = a.x;
        }
        if envminx > self.maxx {
            return false;
        }

        let mut envmaxx = b.x;
        if a.x > b.x {
            envmaxx = a.x;
        }
        if envmaxx < self.minx {
            return false;
        }

        let mut envminy = b.y;
        if a.y < b.y {
            envminy = a.y;
        }
        if envminy > self.maxy {
            return false;
        }

        let mut envmaxy = b.y;
        if a.y > b.y {
            envmaxy = a.y;
        }
        if envmaxy < self.miny {
            return false;
        }

        return true;
    }

    /**
     * Tests if the region defined by <code>other</code>
     * is disjoint from the region of this <code>Envelope</code>.
     * <p>
     * A null envelope is always disjoint.
     *
     *@param  other  the <code>Envelope</code> being checked for disjointness
     *@return        <code>true</code> if the <code>Envelope</code>s are disjoint
     *
     *@see #intersects(Envelope)
     */
    pub fn disjoint_envelope(&self, other: &Envelope) -> bool {
        return !self.intersects_envelope(other);
    }

    /**
     * @deprecated Use #intersects instead. In the future, #overlaps may be
     * changed to be a true overlap check; that is, whether the intersection is
     * two-dimensional.
     */
    pub fn overlaps_envelope(&self, other: &Envelope) -> bool {
        return self.intersects_envelope(other);
    }

    /**
     * Tests if the point <code>p</code>
     * intersects (lies inside) the region of this <code>Envelope</code>.
     *
     *@param  p  the <code>Coordinate</code> to be tested
     *@return <code>true</code> if the point intersects this <code>Envelope</code>
     */
    pub fn intersects_coordinate(&self, p: &Coordinate) -> bool {
        return self.intersects_xy(p.x, p.y);
    }

    /**
     * @deprecated Use #intersects instead.
     */
    pub fn overlaps_coordinate(&self, p: &Coordinate) -> bool {
        return self.intersects_coordinate(p);
    }

    /**
     *  Check if the point <code>(x, y)</code>
     *  intersects (lies inside) the region of this <code>Envelope</code>.
     *
     *@param  x  the x-ordinate of the point
     *@param  y  the y-ordinate of the point
     *@return        <code>true</code> if the point overlaps this <code>Envelope</code>
     */
    pub fn intersects_xy(&self, x: f64, y: f64) -> bool {
        if self.is_null() {
            return false;
        }
        return !(x > self.maxx || x < self.minx || y > self.maxy || y < self.miny);
    }
    /**
     * @deprecated Use #intersects instead.
     */
    pub fn overlaps_xy(&self, x: f64, y: f64) -> bool {
        return self.intersects_xy(x, y);
    }

    /**
     * Tests if the <code>Envelope other</code>
     * lies wholely inside this <code>Envelope</code> (inclusive of the boundary).
     * <p>
     * Note that this is <b>not</b> the same definition as the SFS <tt>contains</tt>,
     * which would exclude the envelope boundary.
     *
     *@param  other the <code>Envelope</code> to check
     *@return true if <code>other</code> is contained in this <code>Envelope</code>
     *
     *@see #covers(Envelope)
     */
    pub fn contains_envelope(&self, other: &Envelope) -> bool {
        return self.covers_envelope(other);
    }

    /**
     * Tests if the given point lies in or on the envelope.
     * <p>
     * Note that this is <b>not</b> the same definition as the SFS <tt>contains</tt>,
     * which would exclude the envelope boundary.
     *
     *@param  p  the point which this <code>Envelope</code> is
     *      being checked for containing
     *@return    <code>true</code> if the point lies in the interior or
     *      on the boundary of this <code>Envelope</code>.
     *      
     *@see #covers(Coordinate)
     */
    pub fn contains_coordinate(&self, p: &Coordinate) -> bool {
        return self.covers_coordinate(p);
    }

    /**
     * Tests if the given point lies in or on the envelope.
     * <p>
     * Note that this is <b>not</b> the same definition as the SFS <tt>contains</tt>,
     * which would exclude the envelope boundary.
     *
     *@param  x  the x-coordinate of the point which this <code>Envelope</code> is
     *      being checked for containing
     *@param  y  the y-coordinate of the point which this <code>Envelope</code> is
     *      being checked for containing
     *@return    <code>true</code> if <code>(x, y)</code> lies in the interior or
     *      on the boundary of this <code>Envelope</code>.
     *      
     *@see #covers(double, double)
     */
    pub fn contains_xy(&self, x: f64, y: f64) -> bool {
        return self.covers_xy(x, y);
    }

    /**
     * Tests if an envelope is properly contained in this one.
     * The envelope is properly contained if it is contained
     * by this one but not equal to it.
     *
     * @param other the envelope to test
     * @return true if the envelope is properly contained
     */
    pub fn contains_properly(&self, other: &Envelope) -> bool {
        if self.equals(other) {
            return false;
        }
        return self.covers_envelope(other);
    }

    /**
     * Tests if the given point lies in or on the envelope.
     *
     *@param  x  the x-coordinate of the point which this <code>Envelope</code> is
     *      being checked for containing
     *@param  y  the y-coordinate of the point which this <code>Envelope</code> is
     *      being checked for containing
     *@return    <code>true</code> if <code>(x, y)</code> lies in the interior or
     *      on the boundary of this <code>Envelope</code>.
     */
    pub fn covers_xy(&self, x: f64, y: f64) -> bool {
        if self.is_null() {
            return false;
        }
        return x >= self.minx && x <= self.maxx && y >= self.miny && y <= self.maxy;
    }

    /**
     * Tests if the given point lies in or on the envelope.
     *
     *@param  p  the point which this <code>Envelope</code> is
     *      being checked for containing
     *@return    <code>true</code> if the point lies in the interior or
     *      on the boundary of this <code>Envelope</code>.
     */
    pub fn covers_coordinate(&self, p: &Coordinate) -> bool {
        return self.covers_xy(p.x, p.y);
    }

    /**
     * Tests if the <code>Envelope other</code>
     * lies wholely inside this <code>Envelope</code> (inclusive of the boundary).
     *
     *@param  other the <code>Envelope</code> to check
     *@return true if this <code>Envelope</code> covers the <code>other</code>
     */
    pub fn covers_envelope(&self, other: &Envelope) -> bool {
        if self.is_null() || other.is_null() {
            return false;
        }
        return other.get_min_x() >= self.minx
            && other.get_max_x() <= self.maxx
            && other.get_min_y() >= self.miny
            && other.get_max_y() <= self.maxy;
    }

    /**
     * Computes the distance between this and another
     * <code>Envelope</code>.
     * The distance between overlapping Envelopes is 0.  Otherwise, the
     * distance is the Euclidean distance between the closest points.
     */
    pub fn distance_envelope(&self, env: &Envelope) -> f64 {
        if self.intersects_envelope(env) {
            return 0.;
        }

        let mut dx = 0.0;
        if self.maxx < env.minx {
            dx = env.minx - self.maxx;
        } else if self.minx > env.maxx {
            dx = self.minx - env.maxx;
        }

        let mut dy = 0.0;
        if self.maxy < env.miny {
            dy = env.miny - self.maxy;
        } else if self.miny > env.maxy {
            dy = self.miny - env.maxy;
        }

        // if either is zero, the envelopes overlap either vertically or horizontally
        if dx == 0.0 {
            return dy;
        }
        if dy == 0.0 {
            return dx;
        }
        return f64::hypot(dx, dy);
    }

    pub fn equals(&self, other: &Envelope) -> bool {
        if self.is_null() {
            return other.is_null();
        }
        return self.maxx == other.get_max_x()
            && self.maxy == other.get_max_y()
            && self.minx == other.get_min_x()
            && self.miny == other.get_min_y();
    }

    /**
     * Compares two envelopes using lexicographic ordering.
     * The ordering comparison is based on the usual numerical
     * comparison between the sequence of ordinates.
     * Null envelopes are less than all non-null envelopes.
     *
     * @param o an Envelope object
     */
    pub fn compare_to_envelope(&self, env: &Envelope) -> i32 {
        // compare nulls if present
        if self.is_null() {
            if env.is_null() {
                return 0;
            }
            return -1;
        } else {
            if env.is_null() {
                return 1;
            }
        }
        // compare based on numerical ordering of ordinates
        if self.minx < env.minx {
            return -1;
        }
        if self.minx > env.minx {
            return 1;
        }
        if self.miny < env.miny {
            return -1;
        }
        if self.miny > env.miny {
            return 1;
        }
        if self.maxx < env.maxx {
            return -1;
        }
        if self.maxx > env.maxx {
            return 1;
        }
        if self.maxy < env.maxy {
            return -1;
        }
        if self.maxy > env.maxy {
            return 1;
        }
        return 0;
    }
}

impl fmt::Display for Envelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Env[{} : {}, {} : {}]",
            self.minx, self.maxx, self.miny, self.maxy
        )
    }
}
