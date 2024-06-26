use crate::core::util::math_util::MathUtil;

use super::{coordinate::Coordinate, coordinate_list::CoordinateList, coordinates::Coordinates, envelope::Envelope};



pub struct CoordinateArrays {
    coordinate_array_type: Vec<Coordinate>,
}

impl CoordinateArrays {
    pub fn new() -> Self {
        Self {
            coordinate_array_type: vec![],
        }
    }

    /**
     * Determine dimension based on subclass of {@link Coordinate}.
     *
     * @param pts supplied coordinates
     * @return number of ordinates recorded
     */
    pub fn dimension(pts: &Vec<Coordinate>) -> i32 {
        if pts.len() == 0 {
            return 3; // unknown, assume default
        }

        let mut dimension: i32 = 0;
        for coordinate in pts.iter() {
            dimension = i32::max(dimension, Coordinates::dimension(coordinate));
        }

        return dimension;
    }

    /**
     * Determine number of measures based on subclass of {@link Coordinate}.
     *
     * @param pts supplied coordinates
     * @return number of measures recorded
     */
    pub fn measures(pts: &Vec<Coordinate>) -> i32 {
        if pts.len() == 0 {
            return 0; // unknown, assume default
        }

        let mut measures: i32 = 0;
        for coordinate in pts.iter() {
            measures = i32::max(measures, Coordinates::measures(coordinate));
        }

        return measures;
    }

    /**
     * Utility method ensuring array contents are of consistent dimension and measures.
     * <p>
     * Array is modified in place if required, coordinates are replaced in the array as required
     * to ensure all coordinates have the same dimension and measures. The final dimension and
     * measures used are the maximum found when checking the array.
     * </p>
     *
     * @param array Modified in place to coordinates of consistent dimension and measures.
     */
    pub fn enforce_consistency(array: &mut Vec<Coordinate>) {
        // step one check
        let mut max_dimension: i32 = -1;
        let mut max_measures: i32 = -1;
        let mut is_consistent = true;

        for i in 0..array.len() {
            let coordinate = array[i];
            let d = Coordinates::dimension(&coordinate);
            let m = Coordinates::measures(&coordinate);
            if max_dimension == -1 {
                max_dimension = d;
                max_measures = m;
                continue;
            }
            if d != max_dimension || m != max_measures {
                is_consistent = false;
                max_dimension = i32::max(max_dimension, d);
                max_measures = i32::max(max_measures, m);
            }
        }

        if !is_consistent {
            for i in 0..array.len() {
                let coordinate = array[i];
                let mut duplicate = Coordinates::create_dim_measures(max_dimension, max_measures);
                duplicate.set_coordinate(&coordinate);
                array[i] = duplicate;
            }
        }
    }

    /**
     * Utility method ensuring array contents are of the specified dimension and measures.
     * <p>
     * Array is returned unmodified if consistent, or a copy of the array is made with
     * each inconsistent coordinate duplicated into an instance of the correct dimension and measures.
     * </p></>
     *
     * @param array coordinate array
     * @param dimension
     * @param measures
     * @return array returned, or copy created if required to enforce consistency.
     */
    pub fn enforce_consistency_dim_measures(
        array: &Vec<Coordinate>,
        dimension: i32,
        measures: i32,
    ) -> Vec<Coordinate> {
        let mut copy = array.to_vec();
        for i in 0..copy.len() {
            let coordinate = array[i];
            let mut duplicate = Coordinates::create_dim_measures(dimension, measures);
            duplicate.set_coordinate(&coordinate);
            copy[i] = duplicate;
        }
        return copy;
    }

    /**
     * Tests whether an array of {@link Coordinate}s forms a ring,
     * by checking length and closure.
     * Self-intersection is not checked.
     *
     * @param pts an array of Coordinates
     * @return true if the coordinate form a ring.
     */
    pub fn is_ring(pts: &Vec<Coordinate>) -> bool {
        if pts.len() < 4 {
            return false;
        }
        if !pts[0].equals_2d(&pts[pts.len() - 1]) {
            return false;
        }
        return true;
    }

    /**
     * Finds a point in a list of points which is not contained in another list of points
     *
     * @param testPts the {@link Coordinate}s to test
     * @param pts     an array of {@link Coordinate}s to test the input points against
     * @return a {@link Coordinate} from <code>testPts</code> which is not in <code>pts</code>, '
     * or <code>null</code>
     */
    pub fn pt_not_in_list(test_pts: &Vec<Coordinate>, pts: &Vec<Coordinate>) -> Option<Coordinate> {
        for i in 0..test_pts.len() {
            let test_pt = test_pts[i];
            if CoordinateArrays::index_of(&test_pt, pts) < 0 {
                return Some(test_pt);
            }
        }
        return None;
    }

    /**
     * Compares two {@link Coordinate} arrays
     * in the forward direction of their coordinates,
     * using lexicographic ordering.
     *
     * @param pts1
     * @param pts2
     * @return an integer indicating the order
     */
    pub fn compare(pts1: &Vec<Coordinate>, pts2: &Vec<Coordinate>) -> i32 {
        let mut i = 0;
        while i < pts1.len() && i < pts2.len() {
            let compare = pts1[i].compare_to(&pts2[i]);
            if compare != 0 {
                return compare;
            }
            i = i + 1;
        }

        // handle situation when arrays are of different length
        if i < pts2.len() {
            return -1;
        }
        if i < pts1.len() {
            return 1;
        }

        return 0;
    }

    //   /**
    //    * A {@link Comparator} for {@link Coordinate} arrays
    //    * in the forward direction of their coordinates,
    //    * using lexicographic ordering.
    //    */
    //   public static class ForwardComparator
    //     implements Comparator {
    //     public int compare(Object o1, Object o2) {
    //       Coordinate[] pts1 = (Coordinate[]) o1;
    //       Coordinate[] pts2 = (Coordinate[]) o2;

    //       return CoordinateArrays.compare(pts1, pts2);
    //     }
    //   }

    /**
     * Determines which orientation of the {@link Coordinate} array
     * is (overall) increasing.
     * In other words, determines which end of the array is "smaller"
     * (using the standard ordering on {@link Coordinate}).
     * Returns an integer indicating the increasing direction.
     * If the sequence is a palindrome, it is defined to be
     * oriented in a positive direction.
     *
     * @param pts the array of Coordinates to test
     * @return <code>1</code> if the array is smaller at the start
     * or is a palindrome,
     * <code>-1</code> if smaller at the end
     */
    pub fn increasing_direction(pts: &Vec<Coordinate>) -> i32 {
        for i in 0..(pts.len() / 1) {
            let j = pts.len() - 1 - i;
            // skip equal points on both ends
            let comp = pts[i].compare_to(&pts[j]);
            if comp != 0 {
                return comp;
            }
        }
        // array must be a palindrome - defined to be in positive direction
        return 1;
    }

    /**
     * Determines whether two {@link Coordinate} arrays of equal length
     * are equal in opposite directions.
     *
     * @param pts1
     * @param pts2
     * @return <code>true</code> if the two arrays are equal in opposite directions.
     */
    pub fn is_equal_reversed(pts1: &Vec<Coordinate>, pts2: &Vec<Coordinate>) -> bool {
        for i in 0..pts1.len() {
            let p1 = pts1[i];
            let p2 = pts2[pts1.len() - i - 1];
            if p1.compare_to(&p2) != 0 {
                return false;
            }
        }
        return true;
    }

    //   /**
    //    * A {@link Comparator} for {@link Coordinate} arrays
    //    * modulo their directionality.
    //    * E.g. if two coordinate arrays are identical but reversed
    //    * they will compare as equal under this ordering.
    //    * If the arrays are not equal, the ordering returned
    //    * is the ordering in the forward direction.
    //    */
    //   public static class BidirectionalComparator
    //     implements Comparator {
    //     public int compare(Object o1, Object o2) {
    //       Coordinate[] pts1 = (Coordinate[]) o1;
    //       Coordinate[] pts2 = (Coordinate[]) o2;

    //       if (pts1.length < pts2.length) return -1;
    //       if (pts1.length > pts2.length) return 1;

    //       if (pts1.length == 0) return 0;

    //       int forwardComp = CoordinateArrays.compare(pts1, pts2);
    //       boolean isEqualRev = isEqualReversed(pts1, pts2);
    //       if (isEqualRev)
    //         return 0;
    //       return forwardComp;
    //     }

    //     public int OLDcompare(Object o1, Object o2) {
    //       Coordinate[] pts1 = (Coordinate[]) o1;
    //       Coordinate[] pts2 = (Coordinate[]) o2;

    //       if (pts1.length < pts2.length) return -1;
    //       if (pts1.length > pts2.length) return 1;

    //       if (pts1.length == 0) return 0;

    //       int dir1 = increasingDirection(pts1);
    //       int dir2 = increasingDirection(pts2);

    //       int i1 = dir1 > 0 ? 0 : pts1.length - 1;
    //       int i2 = dir2 > 0 ? 0 : pts1.length - 1;

    //       for (int i = 0; i < pts1.length; i++) {
    //         int comparePt = pts1[i1].compareTo(pts2[i2]);
    //         if (comparePt != 0)
    //           return comparePt;
    //         i1 += dir1;
    //         i2 += dir2;
    //       }
    //       return 0;
    //     }

    //   }

    /**
     * Creates a deep copy of the argument {@link Coordinate} array.
     *
     * @param coordinates an array of Coordinates
     * @return a deep copy of the input
     */
    pub fn copy_deep(coordinates: &Vec<Coordinate>) -> Vec<Coordinate> {
        let mut copy = vec![Coordinate::default();coordinates.len()];
        for i in 0..coordinates.len() {
            copy[i] = coordinates[i];
        }
        return copy;
    }

    /**
     * Creates a deep copy of a given section of a source {@link Coordinate} array
     * into a destination Coordinate array.
     * The destination array must be an appropriate size to receive
     * the copied coordinates.
     *
     * @param src       an array of Coordinates
     * @param srcStart  the index to start copying from
     * @param dest      the
     * @param destStart the destination index to start copying to
     * @param length    the number of items to copy
     */
    pub fn copy_deep_to(
        src: &Vec<Coordinate>,
        src_start: usize,
        dest: &mut Vec<Coordinate>,
        dest_start: usize,
        length: usize,
    ) {
        for i in 0..length {
            dest[dest_start + i] = src[src_start + i];
        }
    }

    //   /**
    //    * Converts the given Collection of Coordinates into a Coordinate array.
    //    */
    //   public static Coordinate[] toCoordinateArray(Collection coordList) {
    //     return (Coordinate[]) coordList.toArray(coordArrayType);
    //   }

    /**
     * Tests whether {@link Coordinate#equals(Object)} returns true for any two consecutive Coordinates
     * in the given array.
     *
     * @param coord an array of coordinates
     * @return true if the array has repeated points
     */
    pub fn has_repeated_points(coord: &Vec<Coordinate>) -> bool {
        for i in 1..coord.len() {
            if coord[i - 1].equals_2d(&coord[i]) {
                return true;
            }
        }
        return false;
    }

    /**
     * Returns either the given coordinate array if its length is greater than the
     * given amount, or an empty coordinate array.
     */
    pub fn at_least_n_coordinates_or_nothing(n: usize, c: &Vec<Coordinate>) -> Vec<Coordinate> {
        let copy = c.to_vec();
        if copy.len() >= n {
            return copy;
        } else {
            return vec![];
        }
    }

    /**
     * If the coordinate array argument has repeated points,
     * constructs a new array containing no repeated points.
     * Otherwise, returns the argument.
     *
     * @param coord an array of coordinates
     * @return the array with repeated coordinates removed
     * @see #hasRepeatedPoints(Coordinate[])
     */
    pub fn remove_repeated_points(coord: &Vec<Coordinate>) -> Vec<Coordinate> {
        let copy = coord.to_vec();
        if !CoordinateArrays::has_repeated_points(&copy) {
            return copy;
        }
        let coord_list = CoordinateList::new_with_repeated(copy, false);
        return coord_list.to_coordinate_array();
    }

    /**
     * Tests whether an array has any repeated or invalid coordinates.
     *
     * @param coord an array of coordinates
     * @return true if the array contains repeated or invalid coordinates
     * @see Coordinate#isValid()
     */
    pub fn has_repeated_or_invalid_points(coord: &Vec<Coordinate>) -> bool {
        for i in 1..coord.len() {
            if !coord[i].is_valid() {
                return true;
            }
            if coord[i - 1].equals_2d(&coord[i]) {
                return true;
            }
        }
        return false;
    }

    /**
     * If the coordinate array argument has repeated or invalid points,
     * constructs a new array containing no repeated points.
     * Otherwise, returns the argument.
     *
     * @param coord an array of coordinates
     * @return the array with repeated and invalid coordinates removed
     * @see #hasRepeatedOrInvalidPoints(Coordinate[])
     * @see Coordinate#isValid()
     */
    pub fn remove_repeated_or_invalid_points(coord: &Vec<Coordinate>) -> Vec<Coordinate> {
        let copy = coord.to_vec();
        if !CoordinateArrays::has_repeated_or_invalid_points(&copy) {
            return copy;
        }
        let mut coord_list = CoordinateList::default();
        for i in 0..copy.len() {
            if !copy[i].is_valid() {
                continue;
            }
            coord_list.add_coordinate_repeated(copy[i], false);
        }
        return coord_list.to_coordinate_array();
    }

    //   /**
    //    * Collapses a coordinate array to remove all null elements.
    //    *
    //    * @param coord the coordinate array to collapse
    //    * @return an array containing only non-null elements
    //    */
    //   public static Coordinate[] removeNull(Coordinate[] coord) {
    //     int nonNull = 0;
    //     for (int i = 0; i < coord.length; i++) {
    //       if (coord[i] != null) nonNull++;
    //     }
    //     Coordinate[] newCoord = new Coordinate[nonNull];
    //     // empty case
    //     if (nonNull == 0) return newCoord;

    //     int j = 0;
    //     for (int i = 0; i < coord.length; i++) {
    //       if (coord[i] != null) newCoord[j++] = coord[i];
    //     }
    //     return newCoord;
    //   }

    // /**
    //  * Reverses the coordinates in an array in-place.
    //  */
    // pub fn reverse(coord: &mut Vec<Coordinate>) {
    //     if coord.len() <= 1 {
    //         return;
    //     }

    //     let last = coord.len() - 1;
    //     let mid = last / 2;
    //     for i in 0..mid {
    //         let tmp = coord[i];
    //         coord[i] = coord[last - i];
    //         coord[last - i] = tmp;
    //     }
    // }

    /**
     * Returns true if the two arrays are identical, both null, or pointwise
     * equal (as compared using Coordinate#equals)
     *
     * @see Coordinate#equals(Object)
     */
    pub fn equals(coord1: &Vec<Coordinate>, coord2: &Vec<Coordinate>) -> bool {
        if coord1.len() != coord2.len() {
            return false;
        }

        for i in 0..coord1.len() {
            if !coord1[i].equals_2d(&coord2[i]) {
                return false;
            }
        }
        return true;
    }

    //   /**
    //    * Returns true if the two arrays are identical, both null, or pointwise
    //    * equal, using a user-defined {@link Comparator} for {@link Coordinate} s
    //    *
    //    * @param coord1               an array of Coordinates
    //    * @param coord2               an array of Coordinates
    //    * @param coordinateComparator a Comparator for Coordinates
    //    */
    //   pub fn equals(
    //     coord1: Vec<Coordinate>,
    //     coord2: Vec<Coordinate>,
    //     Comparator coordinateComparator) -> bool {
    //     if coord1.len() != coord2.len() {
    //          return false;
    //     }

    //     for i in 0..coord1.len() {
    //       if (coordinateComparator.compare(coord1[i], coord2[i]) != 0)
    //         return false;
    //     }
    //     return true;
    //   }

    /**
     * Returns the minimum coordinate, using the usual lexicographic comparison.
     *
     * @param coordinates the array to search
     * @return the minimum coordinate in the array, found using <code>compareTo</code>
     * @see Coordinate#compareTo(Coordinate)
     */
    pub fn min_coordinate(coordinates: &Vec<Coordinate>) -> Option<Coordinate> {
        let mut min_coord: Option<Coordinate> = None;
        for i in 0..coordinates.len() {
            if min_coord.is_none() || min_coord.unwrap().compare_to(&coordinates[i]) > 0 {
                min_coord = Some(coordinates[i]);
            }
        }
        return min_coord;
    }

    /**
     * Shifts the positions of the coordinates until <code>firstCoordinate</code>
     * is first.
     *
     * @param coordinates     the array to rearrange
     * @param firstCoordinate the coordinate to make first
     */
    pub fn scroll_coordinate(coordinates: &mut Vec<Coordinate>, first_coordinate: &Coordinate) {
        let i = coordinates
            .iter()
            .position(|&r| r.equals_2d(first_coordinate));
        match i {
            Some(i) => CoordinateArrays::scroll_index(coordinates, i),
            None => {}
        }
    }

    /**
     * Shifts the positions of the coordinates until the coordinate
     * at <code>firstCoordinate</code> is first.
     *
     * @param coordinates            the array to rearrange
     * @param indexOfFirstCoordinate the index of the coordinate to make first
     */
    pub fn scroll_index(coordinates: &mut Vec<Coordinate>, index_of_first_coordinate: usize) {
        CoordinateArrays::scroll_index_ensure_ring(
            coordinates,
            index_of_first_coordinate,
            CoordinateArrays::is_ring(coordinates),
        );
    }

    /**
     * Shifts the positions of the coordinates until the coordinate
     * at <code>indexOfFirstCoordinate</code> is first.
     * <p/>
     * If {@code ensureRing} is {@code true}, first and last
     * coordinate of the returned array are equal.
     *
     * @param coordinates            the array to rearrange
     * @param indexOfFirstCoordinate the index of the coordinate to make first
     * @param ensureRing             flag indicating if returned array should form a ring.
     */
    pub fn scroll_index_ensure_ring(
        coordinates: &mut Vec<Coordinate>,
        index_of_first_coordinate: usize,
        ensure_ring: bool,
    ) {
        let i = index_of_first_coordinate;
        if i <= 0 {
            return;
        }

        let mut new_coordinates: Vec<Coordinate> = vec![Coordinate::default();coordinates.len()];
        if !ensure_ring {
            // System.arraycopy(source_arr, sourcePos, dest_arr, destPos, len);
            // System.arraycopy(coordinates, i, newCoordinates, 0, coordinates.length - i);
            // System.arraycopy(coordinates, 0, newCoordinates, coordinates.length - i, i);

            let mut des_pos: usize = 0;
            let mut source_pos: usize = i;
            let mut len = coordinates.len() - i;
            for _j in 0..len {
                new_coordinates[des_pos] = coordinates[source_pos];
                source_pos += 1;
                des_pos += 1;
            }

            des_pos = coordinates.len() - i;
            source_pos = 0;
            len = i;
            for _j in 0..len {
                new_coordinates[des_pos] = coordinates[source_pos];
                source_pos += 1;
                des_pos += 1;
            }

            // new_coordinates.clone_from_slice(&coordinates[i..(coordinates.len() - i)]);
            // new_coordinates.clone_from_slice(&coordinates[(coordinates.len() - i)..i]);
        } else {
            let last = coordinates.len() - 1;

            // fill in values
            let mut j: usize = 0;
            while j < last {
                new_coordinates[j] = coordinates[(i + j) % last];
                j = j + 1;
            }

            // Fix the ring (first == last)
            new_coordinates[j] = new_coordinates[0];
        }

        coordinates.clone_from_slice(&new_coordinates);
    }

    /**
     * Returns the index of <code>coordinate</code> in <code>coordinates</code>.
     * The first position is 0; the second, 1; etc.
     *
     * @param coordinate  the <code>Coordinate</code> to search for
     * @param coordinates the array to search
     * @return the position of <code>coordinate</code>, or -1 if it is
     * not found
     */
    pub fn index_of(coordinate: &Coordinate, coordinates: &Vec<Coordinate>) -> i32 {
        for i in 0..coordinates.len() {
            if coordinate.equals_2d(&coordinates[i]) {
                return i as i32;
            }
        }
        return -1;
    }

    /**
     * Extracts a subsequence of the input {@link Coordinate} array
     * from indices <code>start</code> to
     * <code>end</code> (inclusive).
     * The input indices are clamped to the array size;
     * If the end index is less than the start index,
     * the extracted array will be empty.
     *
     * @param pts   the input array
     * @param start the index of the start of the subsequence to extract
     * @param end   the index of the end of the subsequence to extract
     * @return a subsequence of the input array
     */
    pub fn extract(pts: &Vec<Coordinate>, mut start: usize, mut end: usize) -> Vec<Coordinate> {
        start = MathUtil::clamp_i32(start as i32, 0, pts.len() as i32) as usize;
        end = MathUtil::clamp_i32(end as i32, -1, pts.len() as i32) as usize;

        let mut npts = end - start + 1;
        #[allow(unused_comparisons)]
        if end < 0 {
            npts = 0;
        }
        if start >= pts.len() {
            npts = 0;
        }
        if end < start {
            npts = 0;
        }

        let mut extract_pts: Vec<Coordinate> = vec![];
        if npts == 0 {
            return extract_pts;
        }

        let mut i_pts = 0;
        for i in start..end {
            i_pts = i_pts + 1;
            extract_pts[i_pts] = pts[i];
        }
        return extract_pts;
    }

    /**
     * Computes the envelope of the coordinates.
     *
     * @param coordinates the coordinates to scan
     * @return the envelope of the coordinates
     */
    pub fn envelope(coordinates: &Vec<Coordinate>) -> Envelope {
        let mut env = Envelope::default();
        for i in 0..coordinates.len() {
            env.expand_to_include_coordinate(&coordinates[i]);
        }
        return env;
    }

    /**
     * Extracts the coordinates which intersect an {@link Envelope}.
     *
     * @param coordinates the coordinates to scan
     * @param env         the envelope to intersect with
     * @return an array of the coordinates which intersect the envelope
     */
    pub fn intersection(coordinates: &Vec<Coordinate>, env: &Envelope) -> Vec<Coordinate> {
        let mut coord_list = CoordinateList::default();
        for i in 0..coordinates.len() {
            if env.intersects_coordinate(&coordinates[i]) {
                coord_list.add_coordinate_repeated(coordinates[i], true);
            }
        }
        return coord_list.to_coordinate_array();
    }
}
