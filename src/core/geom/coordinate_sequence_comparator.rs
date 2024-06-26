use super::implementation::coordinate_array_sequence::CoordinateArraySequence;


/**
 * Compares two {@link CoordinateSequence}s.
 * For sequences of the same dimension, the ordering is lexicographic.
 * Otherwise, lower dimensions are sorted before higher.
 * The dimensions compared can be limited; if this is done
 * ordinate dimensions above the limit will not be compared.
 * <p>
 * If different behaviour is required for comparing size, dimension, or
 * coordinate values, any or all methods can be overridden.
 *
 */
pub struct CoordinateSequenceComparator {
    /**
     * The number of dimensions to test
     */
    dimension_limit: i32,
}

impl CoordinateSequenceComparator {
    /**
     * Compare two <code>double</code>s, allowing for NaN values.
     * NaN is treated as being less than any valid number.
     *
     * @param a a <code>double</code>
     * @param b a <code>double</code>
     * @return -1, 0, or 1 depending on whether a is less than, equal to or greater than b
     */
    pub fn compare_f64(a: f64, b: f64) -> i32 {
        if a < b {
            return -1;
        }
        if a > b {
            return 1;
        }

        if f64::is_nan(a) {
            if f64::is_nan(b) {
                return 0;
            }
            return -1;
        }

        if f64::is_nan(b) {
            return 1;
        }
        return 0;
    }

    /**
     * Creates a comparator which will test all dimensions.
     */
    pub fn default() -> Self {
        Self {
            dimension_limit: i32::MAX,
        }
    }

    /**
     * Creates a comparator which will test only the specified number of dimensions.
     *
     * @param dimensionLimit the number of dimensions to test
     */
    pub fn new_with_dimension_limit(dimension_limit: i32) -> Self {
        Self { dimension_limit }
    }

    /**
     * Compares two {@link CoordinateSequence}s for relative order.
     *
     * @param o1 a {@link CoordinateSequence}
     * @param o2 a {@link CoordinateSequence}
     * @return -1, 0, or 1 depending on whether o1 is less than, equal to, or greater than o2
     */
    pub fn compare_coordinate_array_sequence(
        &self,
        s1: &CoordinateArraySequence,
        s2: &CoordinateArraySequence,
    ) -> i32 {
        let size1 = s1.size();
        let size2 = s2.size();

        let dim1 = s1.get_dimension();
        let dim2 = s2.get_dimension();

        let mut min_dim = dim1;
        if dim2 < min_dim {
            min_dim = dim2;
        }
        let mut dim_limited = false;
        if self.dimension_limit <= min_dim {
            min_dim = self.dimension_limit;
            dim_limited = true;
        }

        // lower dimension is less than higher
        if !dim_limited {
            if dim1 < dim2 {
                return -1;
            }
            if dim1 > dim2 {
                return 1;
            }
        }

        // lexicographic ordering of point sequences
        let mut i = 0;
        while i < size1 && i < size2 {
            let pt_comp =
                CoordinateSequenceComparator::compare_coordinate(s1, s2, i as i32, min_dim);
            if pt_comp != 0 {
                return pt_comp;
            }
            i += 1;
        }
        if i < size1 {
            return 1;
        }
        if i < size2 {
            return -1;
        }

        return 0;
    }

    /**
     * Compares the same coordinate of two {@link CoordinateSequence}s
     * along the given number of dimensions.
     *
     * @param s1 a {@link CoordinateSequence}
     * @param s2 a {@link CoordinateSequence}
     * @param i the index of the coordinate to test
     * @param dimension the number of dimensions to test
     * @return -1, 0, or 1 depending on whether s1[i] is less than, equal to, or greater than s2[i]
     */
    pub fn compare_coordinate(
        s1: &CoordinateArraySequence,
        s2: &CoordinateArraySequence,
        i: i32,
        dimension: i32,
    ) -> i32 {
        for d in 0..dimension {
            let ord1 = s1.get_ordinate(i as usize, d);
            let ord2 = s2.get_ordinate(i as usize, d);
            let comp = CoordinateSequenceComparator::compare_f64(ord1, ord2);
            if comp != 0 {
                return comp;
            }
        }
        return 0;
    }
}
