// use crate::geom::{coordinate::Coordinate, precision_model::PrecisionModel};

/**
 * A <code>LineIntersector</code> is an algorithm that can both test whether
 * two line segments intersect and compute the intersection point(s)
 * if they do.
 * <p>
 * There are three possible outcomes when determining whether two line segments intersect:
 * <ul>
 * <li>{@link #NO_INTERSECTION} - the segments do not intersect
 * <li>{@link #POINT_INTERSECTION} - the segments intersect in a single point
 * <li>{@link #COLLINEAR_INTERSECTION} - the segments are collinear and they intersect in a line segment
 * </ul>
 * For segments which intersect in a single point, the point may be either an endpoint
 * or in the interior of each segment.  
 * If the point lies in the interior of both segments, 
 * this is termed a <i>proper intersection</i>.
 * The method {@link #isProper()} test for this situation.
 * <p>
 * The intersection point(s) may be computed in a precise or non-precise manner.
 * Computing an intersection point precisely involves rounding it 
 * via a supplied {@link PrecisionModel}.  
 * <p>
 * LineIntersectors do not perform an initial envelope intersection test 
 * to determine if the segments are disjoint.
 * This is because this class is likely to be used in a context where 
 * envelope overlap is already known to occur (or be likely).
 *
 * @version 1.7
 */

pub struct LineIntersector {
    // result: i32,
    // inputLines: [[Coordinate;2];2],
    // intPt: [Coordinate;2],
    //  // The indexes of the endpoints of the intersection lines, in order along
    //  // the corresponding line
    // intLineIndex: [[i32;2];2],
    // isProper: bool,
    // pa: Coordinate,
    // pb: Coordinate,
    //  // If makePrecise is true, computed intersection coordinates will be made precise
    //  // using Coordinate#makePrecise
    // precisionModel: Option<PrecisionModel>,
}

impl LineIntersector {
// /**
//    * Indicates that line segments do not intersect
//    */
//   pub const NO_INTERSECTION: i32 = 0;
  
//   /**
//    * Indicates that line segments intersect in a single point
//    */
//   pub const POINT_INTERSECTION: i32 = 1;
  
//   /**
//    * Indicates that line segments intersect in a line segment
//    */
//   pub const COLLINEAR_INTERSECTION: i32 = 2;

//   pub fn default() -> Self {
//     let input_lines = [[Coordinate::default();2];2];
//     let int_line_index = [[0;2];2];
//     let coords = [Coordinate::default();2];
//     // alias the intersection points for ease of reference
//     let pa = coords[0];
//     let pb = coords[1];

//     let new = Self {
//         result: 0,
//         inputLines: input_lines,
//         intPt: coords,
//         intLineIndex: int_line_index,
//         pa,
//         pb,
//         isProper: false,
//         precisionModel: None,
//     };

//     new
//   }

//   /**
//    * Computes the "edge distance" of an intersection point p along a segment.
//    * The edge distance is a metric of the point along the edge.
//    * The metric used is a robust and easy to compute metric function.
//    * It is <b>not</b> equivalent to the usual Euclidean metric.
//    * It relies on the fact that either the x or the y ordinates of the
//    * points in the edge are unique, depending on whether the edge is longer in
//    * the horizontal or vertical direction.
//    * <p>
//    * NOTE: This function may produce incorrect distances
//    *  for inputs where p is not precisely on p1-p2
//    * (E.g. p = (139,9) p1 = (139,10), p2 = (280,1) produces distance 0.0, which is incorrect.
//    * <p>
//    * My hypothesis is that the function is safe to use for points which are the
//    * result of <b>rounding</b> points which lie on the line,
//    * but not safe to use for <b>truncated</b> points.
//    */
//   pub fn computeEdgeDistance(p: &Coordinate, p0: &Coordinate, p1: &Coordinate) -> f64 {
//     let dx = f64::abs(p1.x - p0.x);
//     let dy = f64::abs(p1.y - p0.y);

//     let mut dist;   // sentinel value
//     if p.equals_2d(p0) {
//       dist = 0.0;
//     } else if p.equals_2d(p1) {
//       if dx > dy {
//         dist = dx;
//       }
//       else {
//           dist = dy;
//         }
//     }
//     else {
//       let pdx = f64::abs(p.x - p0.x);
//       let pdy = f64::abs(p.y - p0.y);
//       if dx > dy {
//         dist = pdx;
//       }
//       else {
//         dist = pdy;
//       }
//       // <FIX>
//       // hack to ensure that non-endpoints always have a non-zero distance
//       if dist == 0.0 && ! p.equals_2d(p0) {
//         dist = f64::max(pdx, pdy);
//       }
//     }
//     assert!(!(dist == 0.0 && !p.equals_2d(p0)), "Bad distance calculation");
//     return dist;
//   }

//   pub fn nonRobustComputeEdgeDistance(p: &Coordinate, p1: &Coordinate, p2: &Coordinate) -> f64 {
//     let dx = p.x - p1.x;
//     let dy = p.y - p1.y;
//     let dist = f64::hypot(dx, dy);   // dummy value
//     assert!(!(dist == 0.0 && !p.equals_2d(p1)), "Invalid distance calculation");
//     return dist;
//   }

//   /**
//    * Force computed intersection to be rounded to a given precision model
//    * @param precisionModel
//    * @deprecated use <code>setPrecisionModel</code> instead
//    */
//   pub fn setMakePrecise(&mut self, precisionModel: PrecisionModel) {
//     self.precisionModel = Some(precisionModel);
//   }

//   /**
//    * Force computed intersection to be rounded to a given precision model.
//    * No getter is provided, because the precision model is not required to be specified.
//    * @param precisionModel
//    */
//   pub fn setPrecisionModel(&mut self, precisionModel: PrecisionModel) {
//     self.precisionModel = Some(precisionModel);
//   }

//   /**
//    * Gets an endpoint of an input segment.
//    * 
//    * @param segmentIndex the index of the input segment (0 or 1)
//    * @param ptIndex the index of the endpoint (0 or 1)
//    * @return the specified endpoint
//    */
//   pub fn getEndpoint(&self, segmentIndex: usize, ptIndex: usize) -> Coordinate {
//     return self.inputLines[segmentIndex][ptIndex];
//   }

//   pub fn isCollinear(&self) -> bool {
//     return self.result == LineIntersector::COLLINEAR_INTERSECTION;
//   }

//   /**
//    * Computes the intersection of the lines p1-p2 and p3-p4.
//    * This function computes both the boolean value of the hasIntersection test
//    * and the (approximate) value of the intersection point itself (if there is one).
//    */
//   pub fn computeIntersection(&mut self, p1: &Coordinate, p2: &Coordinate, p3: &Coordinate, p4: &Coordinate) {
//     self.inputLines[0][0] = Coordinate::from_coordinate(p1);
//     self.inputLines[0][1] = Coordinate::from_coordinate(p2);
//     self.inputLines[1][0] = Coordinate::from_coordinate(p3);
//     self.inputLines[1][1] = Coordinate::from_coordinate(p4);
//     self.result = computeIntersect(p1, p2, p3, p4);
//   }

// //   public String toString() {
// //     return WKTWriter.toLineString(inputLines[0][0], inputLines[0][1]) + " - "
// //     + WKTWriter.toLineString(inputLines[1][0], inputLines[1][1])
// //                  + getTopologySummary();
// //   }

// //   private String getTopologySummary()
// //   {
// //     StringBuilder catBuilder = new StringBuilder();
// //     if (isEndPoint()) catBuilder.append(" endpoint");
// //     if (isProper) catBuilder.append(" proper");
// //     if (isCollinear()) catBuilder.append(" collinear");
// //     return catBuilder.toString();
// //   }

//   protected boolean isEndPoint() {
//     return hasIntersection() && !isProper;
//   }

//   /**
//    * Tests whether the input geometries intersect.
//    *
//    * @return true if the input geometries intersect
//    */
//   public boolean hasIntersection() {
//     return result != NO_INTERSECTION;
//   }

//   /**
//    * Returns the number of intersection points found.  This will be either 0, 1 or 2.
//    * 
//    * @return the number of intersection points found (0, 1, or 2)
//    */
//   public int getIntersectionNum() { return result; }

//   /**
//    * Returns the intIndex'th intersection point
//    *
//    * @param intIndex is 0 or 1
//    *
//    * @return the intIndex'th intersection point
//    */
//   public Coordinate getIntersection(int intIndex)  { return intPt[intIndex]; }

//   protected void computeIntLineIndex() {
//     if (intLineIndex == null) {
//       intLineIndex = new int[2][2];
//       computeIntLineIndex(0);
//       computeIntLineIndex(1);
//     }
//   }

//   /**
//    * Test whether a point is a intersection point of two line segments.
//    * Note that if the intersection is a line segment, this method only tests for
//    * equality with the endpoints of the intersection segment.
//    * It does <b>not</b> return true if
//    * the input point is internal to the intersection segment.
//    *
//    * @return true if the input point is one of the intersection points.
//    */
//   public boolean isIntersection(Coordinate pt) {
//     for (int i = 0; i < result; i++) {
//       if (intPt[i].equals2D(pt)) {
//         return true;
//       }
//     }
//     return false;
//   }

//   /**
//    * Tests whether either intersection point is an interior point of one of the input segments.
//    *
//    * @return <code>true</code> if either intersection point is in the interior of one of the input segments
//    */
//   public boolean isInteriorIntersection()
//   {
//     if (isInteriorIntersection(0)) return true;
//     if (isInteriorIntersection(1)) return true;
//     return false;
//   }

//   /**
//    * Tests whether either intersection point is an interior point of the specified input segment.
//    *
//    * @return <code>true</code> if either intersection point is in the interior of the input segment
//    */
//   public boolean isInteriorIntersection(int inputLineIndex)
//   {
//     for (int i = 0; i < result; i++) {
//       if (! (   intPt[i].equals2D(inputLines[inputLineIndex][0])
//              || intPt[i].equals2D(inputLines[inputLineIndex][1]) )) {
//         return true;
//       }
//     }
//     return false;
//   }

//   /**
//    * Tests whether an intersection is proper.
//    * <br>
//    * The intersection between two line segments is considered proper if
//    * they intersect in a single point in the interior of both segments
//    * (e.g. the intersection is a single point and is not equal to any of the
//    * endpoints).
//    * <p>
//    * The intersection between a point and a line segment is considered proper
//    * if the point lies in the interior of the segment (e.g. is not equal to
//    * either of the endpoints).
//    *
//    * @return true if the intersection is proper
//    */
//   public boolean isProper() {
//     return hasIntersection() && isProper;
//   }

//   /**
//    * Computes the intIndex'th intersection point in the direction of
//    * a specified input line segment
//    *
//    * @param segmentIndex is 0 or 1
//    * @param intIndex is 0 or 1
//    *
//    * @return the intIndex'th intersection point in the direction of the specified input line segment
//    */
//   public Coordinate getIntersectionAlongSegment(int segmentIndex, int intIndex) {
//     // lazily compute int line array
//     computeIntLineIndex();
//     return intPt[intLineIndex[segmentIndex][intIndex]];
//   }

//   /**
//    * Computes the index (order) of the intIndex'th intersection point in the direction of
//    * a specified input line segment
//    *
//    * @param segmentIndex is 0 or 1
//    * @param intIndex is 0 or 1
//    *
//    * @return the index of the intersection point along the input segment (0 or 1)
//    */
//   public int getIndexAlongSegment(int segmentIndex, int intIndex) {
//     computeIntLineIndex();
//     return intLineIndex[segmentIndex][intIndex];
//   }

//   protected void computeIntLineIndex(int segmentIndex) {
//     double dist0 = getEdgeDistance(segmentIndex, 0);
//     double dist1 = getEdgeDistance(segmentIndex, 1);
//     if (dist0 > dist1) {
//       intLineIndex[segmentIndex][0] = 0;
//       intLineIndex[segmentIndex][1] = 1;
//     }
//     else {
//       intLineIndex[segmentIndex][0] = 1;
//       intLineIndex[segmentIndex][1] = 0;
//     }
//   }

//   /**
//    * Computes the "edge distance" of an intersection point along the specified input line segment.
//    *
//    * @param segmentIndex is 0 or 1
//    * @param intIndex is 0 or 1
//    *
//    * @return the edge distance of the intersection point
//    */
//   public double getEdgeDistance(int segmentIndex, int intIndex) {
//     double dist = computeEdgeDistance(intPt[intIndex], inputLines[segmentIndex][0],
//         inputLines[segmentIndex][1]);
//     return dist;
//   }
}