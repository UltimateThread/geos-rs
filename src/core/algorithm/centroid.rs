use crate::core::geom::{
    coordinate::Coordinate, line_string::LineString, multi_polygon::MultiPolygon, point::Point,
    polygon::Polygon,
};

use super::orientation::Orientation;

/**
 * Computes the centroid of a {@link Geometry} of any dimension.
 * For collections the centroid is computed for the collection of
 * non-empty elements of highest dimension.
 * The centroid of an empty geometry is {@code null}.
 *
 * <h3>Algorithm</h3>
 *
 * <ul>
 * <li><b>Dimension 2</b> - the centroid is computed
 * as the weighted sum of the centroids
 * of a decomposition of the area into (possibly overlapping) triangles.
 * Holes and multipolygons are handled correctly.
 * See http://www.faqs.org/faqs/graphics/algorithms-faq/
 * for further details of the basic approach.
 *
 * <li><b>Dimension 1</b> - Computes the average of the midpoints
 * of all line segments weighted by the segment length.
 * Zero-length lines are treated as points.
 *
 * <li><b>Dimension 0</b> - Compute the average coordinate over all points.
 * Repeated points are all included in the average.
 * </ul>
 *
 * @see InteriorPoint
 * @see org.locationtech.jts.algorithm.construct.MaximumInscribedCircle
 * @see org.locationtech.jts.algorithm.construct.LargestEmptyCircle
 *  
 * @version 1.7
 */

pub struct Centroid {
    area_base_pt: Option<Coordinate>, // the point all triangles are based at
    triangle_cent3: Coordinate,       // temporary variable to hold centroid of triangle
    areasum2: f64,                    /* Partial area sum */
    cg3: Coordinate,                  // partial centroid sum

    // data for linear centroid computation, if needed
    line_cent_sum: Coordinate,
    total_length: f64,

    pt_count: i32,
    pt_cent_sum: Coordinate,
}

impl Centroid {
    /**
     * Computes the centroid point of a geometry.
     *
     * @param geom the geometry to use
     * @return the centroid point, or null if the geometry is empty
     */
    pub fn get_centroid_from_point(point: &Point) -> Option<Coordinate> {
        let cent = Centroid::new_from_point(point);
        return cent.get_centroid();
    }

    pub fn get_centroid_from_polygon(polygon: &Polygon) -> Option<Coordinate> {
        let cent = Centroid::new_from_polygon(polygon);
        return cent.get_centroid();
    }

    pub fn get_centroid_from_multi_polygon(multi_polygon: &MultiPolygon) -> Option<Coordinate> {
        let cent = Centroid::new_from_multi_polygon(multi_polygon);
        return cent.get_centroid();
    }

    pub fn default() -> Self {
        Self {
            area_base_pt: None,
            triangle_cent3: Coordinate::default(),
            areasum2: 0.,
            cg3: Coordinate::default(),
            line_cent_sum: Coordinate::default(),
            total_length: 0.,
            pt_count: 0,
            pt_cent_sum: Coordinate::default(),
        }
    }

    pub fn new_from_point(point: &Point) -> Self {
        let mut new = Centroid::default();
        if let Some(coordinate) = point.get_coordinate() {
            Centroid::add_point(&mut new, &coordinate);
        }

        new
    }

    pub fn new_from_line_string(line_string: &LineString) -> Self {
        let mut new = Centroid::default();
        new.add_line_segments(&line_string.get_coordinates());

        new
    }

    pub fn new_from_polygon(polygon: &Polygon) -> Self {
        let mut new = Centroid::default();
        new.add_polygon(polygon);

        new
    }

    pub fn new_from_multi_polygon(multi_polygon: &MultiPolygon) -> Self {
        let mut new = Centroid::default();
        for i in 0..multi_polygon.get_num_polygons() {
            new.add_polygon(&multi_polygon.get_polygon_at_index(i));
        }

        new
    }

    /**
     * Gets the computed centroid.
     *
     * @return the computed centroid, or null if the input is empty
     */
    pub fn get_centroid(&self) -> Option<Coordinate> {
        /*
         * The centroid is computed from the highest dimension components present in the input.
         * I.e. areas dominate lineal geometry, which dominates points.
         * Degenerate geometry are computed using their effective dimension
         * (e.g. areas may degenerate to lines or points)
         */
        let mut cent = Coordinate::default();
        if f64::abs(self.areasum2) > 0.0 {
            /*
             * Input contains areal geometry
             */
            cent.x = self.cg3.x / 3. / self.areasum2;
            cent.y = self.cg3.y / 3. / self.areasum2;
        } else if self.total_length > 0.0 {
            /*
             * Input contains lineal geometry
             */
            cent.x = self.line_cent_sum.x / self.total_length;
            cent.y = self.line_cent_sum.y / self.total_length;
        } else if self.pt_count > 0 {
            /*
             * Input contains puntal geometry only
             */
            cent.x = self.pt_cent_sum.x / self.pt_count as f64;
            cent.y = self.pt_cent_sum.y / self.pt_count as f64;
        } else {
            return None;
        }
        return Some(cent);
    }

    pub fn set_area_base_point(&mut self, base_pt: &Coordinate) {
        self.area_base_pt = Some(Coordinate::from_coordinate(base_pt));
    }

    pub fn add_polygon(&mut self, poly: &Polygon) {
        self.add_shell(&poly.get_exterior_ring().get_coordinates());
        for i in 0..poly.get_num_interior_ring() {
            self.add_hole(&poly.get_interior_ring_n(i).get_coordinates());
        }
    }

    pub fn add_shell(&mut self, pts: &Vec<Coordinate>) {
        if pts.len() > 0 {
            self.set_area_base_point(&pts[0]);
        }
        let is_positive_area = !Orientation::is_ccw_vec(pts);
        for i in 0..(pts.len() - 1) {
            if let Some(area_base_pt) = self.area_base_pt {
                self.add_triangle(&area_base_pt, &pts[i], &pts[i + 1], is_positive_area);
            }
        }
        self.add_line_segments(pts);
    }

    pub fn add_hole(&mut self, pts: &Vec<Coordinate>) {
        let is_positive_area = Orientation::is_ccw_vec(pts);
        for i in 0..(pts.len() - 1) {
            if let Some(area_base_pt) = self.area_base_pt {
                self.add_triangle(&area_base_pt, &pts[i], &pts[i + 1], is_positive_area);
            }
        }
        self.add_line_segments(pts);
    }

    pub fn add_triangle(
        &mut self,
        p0: &Coordinate,
        p1: &Coordinate,
        p2: &Coordinate,
        is_positive_area: bool,
    ) {
        let mut sign = -1.0;
        if is_positive_area {
            sign = 1.0;
        }
        Centroid::centroid3(p0, p1, p2, &mut self.triangle_cent3);
        let area2 = Centroid::area2(p0, p1, p2);
        self.cg3.x += sign * area2 * self.triangle_cent3.x;
        self.cg3.y += sign * area2 * self.triangle_cent3.y;
        self.areasum2 += sign * area2;
    }

    /**
     * Computes three times the centroid of the triangle p1-p2-p3.
     * The factor of 3 is
     * left in to permit division to be avoided until later.
     */
    pub fn centroid3(p1: &Coordinate, p2: &Coordinate, p3: &Coordinate, c: &mut Coordinate) {
        c.x = p1.x + p2.x + p3.x;
        c.y = p1.y + p2.y + p3.y;
        return;
    }

    /**
     * Returns twice the signed area of the triangle p1-p2-p3.
     * The area is positive if the triangle is oriented CCW, and negative if CW.
     */
    pub fn area2(p1: &Coordinate, p2: &Coordinate, p3: &Coordinate) -> f64 {
        return (p2.x - p1.x) * (p3.y - p1.y) - (p3.x - p1.x) * (p2.y - p1.y);
    }

    /**
     * Adds the line segments defined by an array of coordinates
     * to the linear centroid accumulators.
     *
     * @param pts an array of {@link Coordinate}s
     */
    pub fn add_line_segments(&mut self, pts: &Vec<Coordinate>) {
        let mut line_len = 0.0;
        for i in 0..(pts.len() - 1) {
            let segment_len = pts[i].distance(&pts[i + 1]);
            if segment_len == 0.0 {
                continue;
            }

            line_len += segment_len;

            let midx = (pts[i].x + pts[i + 1].x) / 2.;
            self.line_cent_sum.x += segment_len * midx;
            let midy = (pts[i].y + pts[i + 1].y) / 2.;
            self.line_cent_sum.y += segment_len * midy;
        }
        self.total_length += line_len;
        if line_len == 0.0 && pts.len() > 0 {
            self.add_point(&pts[0]);
        }
    }

    /**
     * Adds a point to the point centroid accumulator.
     * @param pt a {@link Coordinate}
     */
    pub fn add_point(&mut self, pt: &Coordinate) {
        self.pt_count += 1;
        self.pt_cent_sum.x += pt.x;
        self.pt_cent_sum.y += pt.y;
    }
}
