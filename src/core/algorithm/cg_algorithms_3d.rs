use crate::core::{geom::coordinate::Coordinate, math::vector_3d::Vector3D};

/**
 * Basic computational geometry algorithms
 * for geometry and coordinates defined in 3-dimensional Cartesian space.
 *
 * @author mdavis
 *
 */

pub struct CGAlgorithms3D {}

impl CGAlgorithms3D {
    pub fn distance(p0: &Coordinate, p1: &Coordinate) -> f64 {
        // default to 2D distance if either Z is not set
        if f64::is_nan(p0.get_z()) || f64::is_nan(p1.get_z()) {
            return p0.distance(p1);
        }

        let dx = p0.x - p1.x;
        let dy = p0.y - p1.y;
        let dz = p0.get_z() - p1.get_z();
        return f64::sqrt(dx * dx + dy * dy + dz * dz);
    }

    pub fn distance_point_segment(p: &Coordinate, ac: &Coordinate, bc: &Coordinate) -> f64 {
        // if start = end, then just compute distance to one of the endpoints
        if ac.equals_3d(bc) {
            return CGAlgorithms3D::distance(p, ac);
        }

        // otherwise use comp.graphics.algorithms Frequently Asked Questions method
        /*
         * (1) r = AC dot AB
         *         ---------
         *         ||AB||^2
         *
         * r has the following meaning:
         *   r=0 P = A
         *   r=1 P = B
         *   r<0 P is on the backward extension of AB
         *   r>1 P is on the forward extension of AB
         *   0<r<1 P is interior to AB
         */

        let len2 = (bc.x - ac.x) * (bc.x - ac.x)
            + (bc.y - ac.y) * (bc.y - ac.y)
            + (bc.get_z() - ac.get_z()) * (bc.get_z() - ac.get_z());
        if f64::is_nan(len2) {
            return f64::NAN;
        }
        let r = ((p.x - ac.x) * (bc.x - ac.x)
            + (p.y - ac.y) * (bc.y - ac.y)
            + (p.get_z() - ac.get_z()) * (bc.get_z() - ac.get_z()))
            / len2;

        if r <= 0.0 {
            return CGAlgorithms3D::distance(p, ac);
        }
        if r >= 1.0 {
            return CGAlgorithms3D::distance(p, bc);
        }

        // compute closest point q on line segment
        let qx = ac.x + r * (bc.x - ac.x);
        let qy = ac.y + r * (bc.y - ac.y);
        let qz = ac.get_z() + r * (bc.get_z() - ac.get_z());
        // result is distance from p to q
        let dx = p.x - qx;
        let dy = p.y - qy;
        let dz = p.get_z() - qz;
        return f64::sqrt(dx * dx + dy * dy + dz * dz);
    }

    /**
     * Computes the distance between two 3D segments.
     *
     * @param A the start point of the first segment
     * @param B the end point of the first segment
     * @param C the start point of the second segment
     * @param D the end point of the second segment
     * @return the distance between the segments
     */
    pub fn distance_segment_segment(
        ac: &Coordinate,
        bc: &Coordinate,
        cc: &Coordinate,
        dd: &Coordinate,
    ) -> f64 {
        /*
         This calculation is susceptible to round off errors when
         passed large ordinate values.
         It may be possible to improve this by using {@link DD} arithmetic.
        */
        if ac.equals_3d(bc) {
            return CGAlgorithms3D::distance_point_segment(ac, cc, dd);
        }
        if cc.equals_3d(bc) {
            return CGAlgorithms3D::distance_point_segment(cc, ac, bc);
        }

        /*
         Algorithm derived from http://softsurfer.com/Archive/algorithm_0106/algorithm_0106.htm
        */
        let a = Vector3D::dot_4(ac, bc, ac, bc);
        let b = Vector3D::dot_4(ac, bc, cc, dd);
        let c = Vector3D::dot_4(cc, dd, cc, dd);
        let d = Vector3D::dot_4(ac, bc, cc, ac);
        let e = Vector3D::dot_4(cc, dd, cc, ac);

        let denom = a * c - b * b;
        if f64::is_nan(denom) {
            return f64::NAN;
        }

        let s: f64;
        let t: f64;
        if denom <= 0.0 {
            /*
             The lines are parallel.
             In this case solve for the parameters s and t by assuming s is 0.
            */
            s = 0.;
            // choose largest denominator for optimal numeric conditioning
            if b > c {
                t = d / b;
            } else {
                t = e / c;
            }
        } else {
            s = (b * e - c * d) / denom;
            t = (a * e - b * d) / denom;
        }
        if s < 0. {
            return CGAlgorithms3D::distance_point_segment(ac, cc, dd);
        } else if s > 1. {
            return CGAlgorithms3D::distance_point_segment(bc, cc, dd);
        } else if t < 0. {
            return CGAlgorithms3D::distance_point_segment(cc, ac, bc);
        } else if t > 1. {
            return CGAlgorithms3D::distance_point_segment(dd, ac, bc);
        }
        /*
         The closest points are in interiors of segments,
         so compute them directly
        */
        let x1 = ac.x + s * (bc.x - ac.x);
        let y1 = ac.y + s * (bc.y - ac.y);
        let z1 = ac.get_z() + s * (bc.get_z() - ac.get_z());

        let x2 = cc.x + t * (dd.x - cc.x);
        let y2 = cc.y + t * (dd.y - cc.y);
        let z2 = cc.get_z() + t * (dd.get_z() - cc.get_z());

        // length (p1-p2)
        return CGAlgorithms3D::distance(
            &Coordinate::new_xyz(x1, y1, z1),
            &Coordinate::new_xyz(x2, y2, z2),
        );
    }
}
