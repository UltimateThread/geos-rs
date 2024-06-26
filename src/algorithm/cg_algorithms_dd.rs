/*
 * Copyright (c) 2016 Martin Davis.
 *
 * All rights reserved. This program and the accompanying materials
 * are made available under the terms of the Eclipse Public License 2.0
 * and Eclipse Distribution License v. 1.0 which accompanies this distribution.
 * The Eclipse Public License is available at http://www.eclipse.org/legal/epl-v20.html
 * and the Eclipse Distribution License is available at
 *
 * http://www.eclipse.org/org/documents/edl-v10.php.
 */

use crate::{geom::coordinate::Coordinate, math::dd::DD};

/**
 * Implements basic computational geometry algorithms using {@link DD} arithmetic.
 * 
 * @author Martin Davis
 *
 */
pub struct CGAlgorithmsDD {

}

impl CGAlgorithmsDD {
    /**
     * A value which is safely greater than the
     * relative round-off error in double-precision numbers
     */
    const DP_SAFE_EPSILON: f64 = 1e-15;
    // private CGAlgorithmsDD() {}

    /**
     * Returns the index of the direction of the point {@code q} relative to
     * a vector specified by {@code p1-p2}.
     * 
     * @param p1 the origin point of the vector
     * @param p2 the final point of the vector
     * @param q the point to compute the direction to
     * 
     * @return {@code 1} if q is counter-clockwise (left) from p1-p2
     *         {@code -1} if q is clockwise (right) from p1-p2
     *         {@code 0} if q is collinear with p1-p2
     */
    pub fn orientation_index_coordinates(p1: &Coordinate, p2: &Coordinate, q: &Coordinate) -> i32 {
      return CGAlgorithmsDD::orientation_index_xy(p1.x, p1.y, p2.x, p2.y, q.x, q.y);
    }
    
    /**
     * Returns the index of the direction of the point {@code q} relative to
     * a vector specified by {@code p1-p2}.
     * 
     * @param p1x the x ordinate of the vector origin point
     * @param p1y the y ordinate of the vector origin point
     * @param p2x the x ordinate of the vector final point
     * @param p2y the y ordinate of the vector final point
     * @param qx the x ordinate of the query point
     * @param qy the y ordinate of the query point
     * 
     * @return 1 if q is counter-clockwise (left) from p1-p2
     *        -1 if q is clockwise (right) from p1-p2
     *         0 if q is collinear with p1-p2
     */
    pub fn orientation_index_xy(p1x: f64, p1y: f64, p2x: f64, p2y: f64, qx: f64, qy: f64) -> i32
    {
      // fast filter for orientation index
      // avoids use of slow extended-precision arithmetic in many cases
      let index = CGAlgorithmsDD::orientation_index_filter(p1x, p1y, p2x, p2y, qx, qy);
      if index <= 1 { return index; }
      
      // normalize coordinates
      let mut dx1 = DD::value_of_f64(p2x);
      dx1.self_add_f64(-p1x);
      let mut dy1 = DD::value_of_f64(p2y);
      dy1.self_add_f64(-p1y);
      let mut dx2 = DD::value_of_f64(qx);
      dx2.self_add_f64(-p2x);
      let mut dy2 = DD::value_of_f64(qy);
      dy2.self_add_f64(-p2y);
  
      // sign of determinant - unrolled for performance
      dx1.self_multiply_dd(&dy2);
      dy1.self_multiply_dd(&dx2);
      dx1.self_subtract_dd(&dy1);
      return dx1.signum();
    }
    
    /**
     * Computes the sign of the determinant of the 2x2 matrix
     * with the given entries.
     * 
     * @return -1 if the determinant is negative,
     *          1 if the determinant is positive,
     *          0 if the determinant is 0.
     */
    pub fn sign_of_det2x_2dd(x1: &DD, y1: &DD, x2: &DD, y2: &DD) -> i32 {
      let mut det = x1.multiply_dd(y2);
      det.self_subtract_dd(&y1.multiply_dd(x2));
      return det.signum();
    }
  
    /**
     * Computes the sign of the determinant of the 2x2 matrix
     * with the given entries.
     * 
     * @return -1 if the determinant is negative,
     *          1 if the determinant is positive,
     *          0 if the determinant is 0.
     */
    pub fn sign_of_det2x2_f64(dx1: f64, dy1: f64, dx2: f64, dy2: f64) -> i32 {
      let x1 = DD::value_of_f64(dx1);
      let y1 = DD::value_of_f64(dy1);
      let x2 = DD::value_of_f64(dx2);
      let y2 = DD::value_of_f64(dy2);
  
      let mut det = x1.multiply_dd(&y2);
      det.self_subtract_dd(&y1.multiply_dd(&x2));
      return det.signum();
    }
  
    /**
     * A filter for computing the orientation index of three coordinates.
     * <p>
     * If the orientation can be computed safely using standard DP
     * arithmetic, this routine returns the orientation index.
     * Otherwise, a value i > 1 is returned.
     * In this case the orientation index must 
     * be computed using some other more robust method.
     * The filter is fast to compute, so can be used to 
     * avoid the use of slower robust methods except when they are really needed,
     * thus providing better average performance.
     * <p>
     * Uses an approach due to Jonathan Shewchuk, which is in the public domain.
     * 
     * @param pax A coordinate
     * @param pay A coordinate
     * @param pbx B coordinate
     * @param pby B coordinate
     * @param pcx C coordinate
     * @param pcy C coordinate
     * @return the orientation index if it can be computed safely
     * @return i > 1 if the orientation index cannot be computed safely
     */
    pub fn orientation_index_filter(pax: f64, pay: f64,
        pbx: f64, pby: f64, pcx: f64, pcy: f64) -> i32
    {
      let detsum: f64;
  
      let detleft = (pax - pcx) * (pby - pcy);
      let detright = (pay - pcy) * (pbx - pcx);
      let det = detleft - detright;
  
      if detleft > 0.0 {
        if detright <= 0.0 {
          return CGAlgorithmsDD::signum(det);
        }
        else {
          detsum = detleft + detright;
        }
      }
      else if detleft < 0.0 {
        if detright >= 0.0 {
          return CGAlgorithmsDD::signum(det);
        }
        else {
          detsum = -detleft - detright;
        }
      }
      else {
        return CGAlgorithmsDD::signum(det);
      }
  
      let errbound = CGAlgorithmsDD::DP_SAFE_EPSILON * detsum;
      if (det >= errbound) || (-det >= errbound) {
        return CGAlgorithmsDD::signum(det);
      }
  
      return 2;
    }
  
    pub fn signum(x: f64) -> i32 {
      if x > 0. { return 1; }
      if x < 0. { return -1; }
      return 0;
    }
  
    /**
     * Computes an intersection point between two lines
     * using DD arithmetic.
     * If the lines are parallel (either identical
     * or separate) a null value is returned.
     * 
     * @param p1 an endpoint of line segment 1
     * @param p2 an endpoint of line segment 1
     * @param q1 an endpoint of line segment 2
     * @param q2 an endpoint of line segment 2
     * @return an intersection point if one exists, or null if the lines are parallel
     */
    pub fn intersection(p1: &Coordinate, p2: &Coordinate, q1: &Coordinate, q2: &Coordinate) -> Option<Coordinate> {
      let mut px = DD::new_x(p1.y);
      px.self_subtract_f64(p2.y);
      let mut py = DD::new_x(p2.x);
      py.self_subtract_f64(p1.x);
      let mut pw = DD::new_x(p1.x);
      pw.self_multiply_f64(p2.y);
      let mut new_x = DD::new_x(p2.x);
      new_x.self_multiply_f64(p1.y);
      pw.self_subtract_dd(&new_x);
  
      let mut qx = DD::new_x(q1.y);
      qx.self_subtract_f64(q2.y);
      let mut qy = DD::new_x(q2.x);
      qy.self_subtract_f64(q1.x);
      let mut qw = DD::new_x(q1.x);
      qw.self_multiply_f64(q2.y);
      let mut new_qw = DD::new_x(q2.x);
      new_qw.self_multiply_f64(q1.y);
      qw.self_subtract_dd(&new_qw);
  
      let mut x = py.multiply_dd(&qw);
      x.self_subtract_dd(&qy.multiply_dd(&pw));
      let mut y = qx.multiply_dd(&pw);
      y.self_subtract_dd(&px.multiply_dd(&qw));
      let mut w = px.multiply_dd(&qy);
      w.self_subtract_dd(&qx.multiply_dd(&py));
  
      x.self_divide_dd(&w);
      y.self_divide_dd(&w);
      let x_int = x.double_value();
      let y_int = y.double_value();
  
      if (f64::is_nan(x_int)) || (f64::is_infinite(x_int) || f64::is_nan(y_int)) || (f64::is_infinite(y_int)) {
        return None;
      }
  
      return Some(Coordinate::new_xy(x_int, y_int));
    }
}