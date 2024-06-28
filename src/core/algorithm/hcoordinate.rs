use crate::core::geom::coordinate::Coordinate;

/**
 * Represents a homogeneous coordinate in a 2-D coordinate space.
 * In JTS {@link HCoordinate}s are used as a clean way
 * of computing intersections between line segments.
 *
 * @author David Skea
 * @version 1.7
 */

pub struct HCoordinate {
    x: f64,
    y: f64,
    w: f64,
}

impl HCoordinate {
    /**
     * Computes the (approximate) intersection point between two line segments
     * using homogeneous coordinates.
     * <p>
     * Note that this algorithm is
     * not numerically stable; i.e. it can produce intersection points which
     * lie outside the envelope of the line segments themselves.  In order
     * to increase the precision of the calculation input points should be normalized
     * before passing them to this routine.
     *
     * @deprecated use {@link Intersection#intersection(Coordinate, Coordinate, Coordinate, Coordinate)}
     */
    pub fn intersection(
        p1: &Coordinate,
        p2: &Coordinate,
        q1: &Coordinate,
        q2: &Coordinate,
    ) -> Option<Coordinate> {
        // unrolled computation
        let px = p1.y - p2.y;
        let py = p2.x - p1.x;
        let pw = p1.x * p2.y - p2.x * p1.y;

        let qx = q1.y - q2.y;
        let qy = q2.x - q1.x;
        let qw = q1.x * q2.y - q2.x * q1.y;

        let x = py * qw - qy * pw;
        let y = qx * pw - px * qw;
        let w = px * qy - qx * py;

        let x_int = x / w;
        let y_int = y / w;

        if (f64::is_nan(x_int))
            || (f64::is_infinite(x_int) || f64::is_nan(y_int))
            || (f64::is_infinite(y_int))
        {
            return None;
        }

        return Some(Coordinate::new_xy(x_int, y_int));
    }

    /*
    public static Coordinate OLDintersection(
        Coordinate p1, Coordinate p2,
        Coordinate q1, Coordinate q2)
        throws NotRepresentableException
    {
      HCoordinate l1 = new HCoordinate(p1, p2);
      HCoordinate l2 = new HCoordinate(q1, q2);
      HCoordinate intHCoord = new HCoordinate(l1, l2);
      Coordinate intPt = intHCoord.getCoordinate();
      return intPt;
    }
    */

    pub fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 1.0,
        }
    }

    pub fn new_with_xyw(_x: f64, _y: f64, _w: f64) -> Self {
        Self {
            x: _x,
            y: _y,
            w: _w,
        }
    }

    pub fn new_with_xy(_x: f64, _y: f64) -> Self {
        Self {
            x: _x,
            y: _y,
            w: 1.0,
        }
    }

    pub fn new_with_coordinate(p: &Coordinate) -> Self {
        Self {
            x: p.x,
            y: p.y,
            w: 1.0,
        }
    }

    pub fn new_with_hcoordinates(p1: &HCoordinate, p2: &HCoordinate) -> Self {
        Self {
            x: p1.y * p2.w - p2.y * p1.w,
            y: p2.x * p1.w - p1.x * p2.w,
            w: p1.x * p2.y - p2.x * p1.y,
        }
    }

    /**
     * Constructs a homogeneous coordinate which is the intersection of the lines
     * define by the homogenous coordinates represented by two
     * {@link Coordinate}s.
     *
     * @param p1
     * @param p2
     */
    pub fn new_with_coordinates(p1: &Coordinate, p2: &Coordinate) -> Self {
        Self {
            // optimization when it is known that w = 1
            x: p1.y - p2.y,
            y: p2.x - p1.x,
            w: p1.x * p2.y - p2.x * p1.y,
        }
    }

    pub fn new_with_coordinates_4(
        p1: &Coordinate,
        p2: &Coordinate,
        q1: &Coordinate,
        q2: &Coordinate,
    ) -> Self {
        // unrolled computation
        let px = p1.y - p2.y;
        let py = p2.x - p1.x;
        let pw = p1.x * p2.y - p2.x * p1.y;

        let qx = q1.y - q2.y;
        let qy = q2.x - q1.x;
        let qw = q1.x * q2.y - q2.x * q1.y;

        Self {
            x: py * qw - qy * pw,
            y: qx * pw - px * qw,
            w: px * qy - qx * py,
        }
    }

    pub fn get_x(&self) -> f64 {
        let a = self.x / self.w;
        if (f64::is_nan(a)) || (f64::is_infinite(a)) {
            return f64::NAN;
        }
        return a;
    }

    pub fn get_y(&self) -> f64 {
        let a = self.y / self.w;
        if (f64::is_nan(a)) || (f64::is_infinite(a)) {
            return f64::NAN;
        }
        return a;
    }

    pub fn get_coordinate(&self) -> Coordinate {
        let mut p = Coordinate::default();
        p.x = self.get_x();
        p.y = self.get_y();
        return p;
    }
}
