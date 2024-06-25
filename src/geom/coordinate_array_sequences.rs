use super::{coordinate::Coordinate, implementation::{coordinate_array_sequence::CoordinateArraySequence, coordinate_array_sequence_factory::CoordinateArraySequenceFactory}};



/**
 * Utility functions for manipulating {@link CoordinateSequence}
 *
 * @version 1.7
 */
pub struct CoordinateArraySequences {}

impl CoordinateArraySequences {
    /**
     * Reverses the coordinates in a sequence in-place.
     *
     * @param seq the coordinate sequence to reverse
     */
    pub fn reverse(seq: &mut CoordinateArraySequence) {
        if seq.size() <= 1 {
            return;
        }

        let last = seq.size() - 1;
        let mid = last / 2;
        for i in 0..=mid {
            CoordinateArraySequences::swap(seq, i, last - i);
        }
    }

    /**
     * Swaps two coordinates in a sequence.
     *
     * @param seq the sequence to modify
     * @param i the index of a coordinate to swap
     * @param j the index of a coordinate to swap
     */
    pub fn swap(seq: &mut CoordinateArraySequence, i: usize, j: usize) {
        if i == j {
            return;
        }
        for dim in 0..seq.get_dimension() {
            let tmp = seq.get_ordinate(i, dim);
            seq.set_ordinate(i, dim, seq.get_ordinate(j, dim));
            seq.set_ordinate(j, dim, tmp);
        }
    }

    /**
     * Copies a section of a {@link CoordinateSequence} to another {@link CoordinateSequence}.
     * The sequences may have different dimensions;
     * in this case only the common dimensions are copied.
     *
     * @param src the sequence to copy from
     * @param srcPos the position in the source sequence to start copying at
     * @param dest the sequence to copy to
     * @param destPos the position in the destination sequence to copy to
     * @param length the number of coordinates to copy
     */
    pub fn copy(
        src: &CoordinateArraySequence,
        src_pos: usize,
        dest: &mut CoordinateArraySequence,
        dest_pos: usize,
        length: usize,
    ) {
        for i in 0..length {
            CoordinateArraySequences::copy_coord(src, src_pos + i, dest, dest_pos + i);
        }
    }

    /**
     * Copies a coordinate of a {@link CoordinateSequence} to another {@link CoordinateSequence}.
     * The sequences may have different dimensions;
     * in this case only the common dimensions are copied.
     *
     * @param src the sequence to copy from
     * @param srcPos the source coordinate to copy
     * @param dest the sequence to copy to
     * @param destPos the destination coordinate to copy to
     */
    pub fn copy_coord(
        src: &CoordinateArraySequence,
        src_pos: usize,
        dest: &mut CoordinateArraySequence,
        dest_pos: usize,
    ) {
        let min_dim = i32::min(src.get_dimension(), dest.get_dimension());
        for dim in 0..min_dim {
            dest.set_ordinate(dest_pos, dim, src.get_ordinate(src_pos, dim));
        }
    }

    /**
     * Tests whether a {@link CoordinateSequence} forms a valid {@link LinearRing},
     * by checking the sequence length and closure
     * (whether the first and last points are identical in 2D).
     * Self-intersection is not checked.
     *
     * @param seq the sequence to test
     * @return true if the sequence is a ring
     * @see LinearRing
     */
    pub fn is_ring(seq: &CoordinateArraySequence) -> bool {
        let n = seq.size();
        if n == 0 {
            return true;
        }
        // too few points
        if n <= 3 {
            return false;
        }
        // test if closed
        return seq.get_ordinate(0, CoordinateArraySequence::X)
            == seq.get_ordinate(n - 1, CoordinateArraySequence::X)
            && seq.get_ordinate(0, CoordinateArraySequence::Y)
                == seq.get_ordinate(n - 1, CoordinateArraySequence::Y);
    }

    /**
     * Ensures that a CoordinateSequence forms a valid ring,
     * returning a new closed sequence of the correct length if required.
     * If the input sequence is already a valid ring, it is returned
     * without modification.
     * If the input sequence is too short or is not closed,
     * it is extended with one or more copies of the start point.
     *
     * @param fact the CoordinateSequenceFactory to use to create the new sequence
     * @param seq the sequence to test
     * @return the original sequence, if it was a valid ring, or a new sequence which is valid.
     */
    pub fn ensure_valid_ring(seq: &CoordinateArraySequence) -> CoordinateArraySequence {
        let copy = seq.copy();
        let n = copy.size();
        // empty sequence is valid
        if n == 0 {
            return copy;
        }
        // too short - make a new one
        if n <= 3 {
            return CoordinateArraySequences::create_closed_ring(&copy, 4);
        }

        let is_closed = copy.get_ordinate(0, CoordinateArraySequence::X)
            == copy.get_ordinate(n - 1, CoordinateArraySequence::X)
            && copy.get_ordinate(0, CoordinateArraySequence::Y)
                == copy.get_ordinate(n - 1, CoordinateArraySequence::Y);
        if is_closed {
            return copy;
        }
        // make a new closed ring
        return CoordinateArraySequences::create_closed_ring(seq, n + 1);
    }

    pub fn create_closed_ring(
        seq: &CoordinateArraySequence,
        size: usize,
    ) -> CoordinateArraySequence {
        let mut newseq =
            CoordinateArraySequenceFactory::create_with_size_dimension(size, seq.get_dimension());
        let n = seq.size();
        CoordinateArraySequences::copy(seq, 0, &mut newseq, 0, n);
        // fill remaining coordinates with start point
        for i in n..size {
            CoordinateArraySequences::copy(seq, 0, &mut newseq, i, 1);
        }
        return newseq;
    }

    pub fn extend(seq: &CoordinateArraySequence, size: usize) -> CoordinateArraySequence {
        let mut newseq =
            CoordinateArraySequenceFactory::create_with_size_dimension(size, seq.get_dimension());
        let n = seq.size();
        CoordinateArraySequences::copy(seq, 0, &mut newseq, 0, n);
        // fill remaining coordinates with end point, if it exists
        if n > 0 {
            for i in n..size {
                CoordinateArraySequences::copy(seq, n - 1, &mut newseq, i, 1);
            }
        }
        return newseq;
    }

    /**
     * Tests whether two {@link CoordinateSequence}s are equal.
     * To be equal, the sequences must be the same length.
     * They do not need to be of the same dimension,
     * but the ordinate values for the smallest dimension of the two
     * must be equal.
     * Two <code>NaN</code> ordinates values are considered to be equal.
     *
     * @param cs1 a CoordinateSequence
     * @param cs2 a CoordinateSequence
     * @return true if the sequences are equal in the common dimensions
     */
    pub fn is_equal(cs1: &CoordinateArraySequence, cs2: &CoordinateArraySequence) -> bool {
        let cs1_size = cs1.size();
        let cs2_size = cs2.size();
        if cs1_size != cs2_size {
            return false;
        }
        let dim = i32::min(cs1.get_dimension(), cs2.get_dimension());
        for i in 0..cs1_size {
            for d in 0..dim {
                let v1 = cs1.get_ordinate(i, d);
                let v2 = cs2.get_ordinate(i, d);
                if cs1.get_ordinate(i, d) == cs2.get_ordinate(i, d) {
                    continue;
                } else if f64::is_nan(v1) && f64::is_nan(v2) {
                    // special check for NaNs
                    continue;
                } else {
                    return false;
                }
            }
        }
        return true;
    }

    /**
     *  Returns the minimum coordinate, using the usual lexicographic comparison.
     *
     *@param  seq  the coordinate sequence to search
     *@return  the minimum coordinate in the sequence, found using <code>compareTo</code>
     *@see Coordinate#compareTo(Object)
     */
    pub fn min_coordinate(seq: &CoordinateArraySequence) -> Option<Coordinate> {
        let mut min_coord: Option<Coordinate> = None;
        for i in 0..seq.size() {
            let test_coord = seq.get_coordinate_index(i);
            if min_coord.is_none() || min_coord.unwrap().compare_to(&test_coord) > 0 {
                min_coord = Some(test_coord);
            }
        }
        return min_coord;
    }

    /**
     *  Returns the index of the minimum coordinate of the whole
     *  coordinate sequence, using the usual lexicographic comparison.
     *
     *@param  seq  the coordinate sequence to search
     *@return  the index of the minimum coordinate in the sequence, found using <code>compareTo</code>
     *@see Coordinate#compareTo(Object)
     */
    pub fn min_coordinate_index_sequence(seq: &CoordinateArraySequence) -> usize {
        return CoordinateArraySequences::min_coordinate_index_sequence_from_to(
            seq,
            0,
            seq.size() - 1,
        );
    }

    /**
     *  Returns the index of the minimum coordinate of a part of
     *  the coordinate sequence (defined by {@code from} and {@code to},
     *  using the usual lexicographic comparison.
     *
     *@param  seq   the coordinate sequence to search
     *@param  from  the lower search index
     *@param  to    the upper search index
     *@return  the index of the minimum coordinate in the sequence, found using <code>compareTo</code>
     *@see Coordinate#compareTo(Object)
     */
    pub fn min_coordinate_index_sequence_from_to(
        seq: &CoordinateArraySequence,
        from: usize,
        to: usize,
    ) -> usize {
        let mut min_coord_index: i32 = -1;
        let mut min_coord: Option<Coordinate> = None;
        for i in from..=to {
            let test_coord = seq.get_coordinate_index(i);
            if min_coord.is_none() || min_coord.unwrap().compare_to(&test_coord) > 0 {
                min_coord = Some(test_coord);
                min_coord_index = i as i32;
            }
        }
        return min_coord_index as usize;
    }

    /**
     *  Shifts the positions of the coordinates until <code>firstCoordinate</code>
     *  is first.
     *
     *@param  seq      the coordinate sequence to rearrange
     *@param  firstCoordinate  the coordinate to make first
     */
    pub fn scroll_coordinate(seq: &mut CoordinateArraySequence, first_coordinate: &Coordinate) {
        let i = CoordinateArraySequences::index_of(first_coordinate, seq);
        if i <= 0 {
            return;
        }
        CoordinateArraySequences::scroll_coordinate_index(seq, i as usize);
    }

    /**
     *  Shifts the positions of the coordinates until the coordinate at  <code>firstCoordinateIndex</code>
     *  is first.
     *
     *@param  seq      the coordinate sequence to rearrange
     *@param  indexOfFirstCoordinate  the index of the coordinate to make first
     */
    pub fn scroll_coordinate_index(
        seq: &mut CoordinateArraySequence,
        index_of_first_coordinate: usize,
    ) {
        CoordinateArraySequences::scroll_coordinate_index_ensure_ring(
            seq,
            index_of_first_coordinate,
            CoordinateArraySequences::is_ring(seq),
        );
    }

    /**
     *  Shifts the positions of the coordinates until the coordinate at  <code>firstCoordinateIndex</code>
     *  is first.
     *
     *@param  seq      the coordinate sequence to rearrange
     *@param  indexOfFirstCoordinate
     *                 the index of the coordinate to make first
     *@param  ensureRing
     *                 makes sure that {@code} will be a closed ring upon exit
     */
    pub fn scroll_coordinate_index_ensure_ring(
        seq: &mut CoordinateArraySequence,
        index_of_first_coordinate: usize,
        ensure_ring: bool,
    ) {
        let i = index_of_first_coordinate;
        if i <= 0 {
            return;
        }

        // make a copy of the sequence
        let copy = seq.copy();

        // test if ring, determine last index
        let mut last = seq.size();
        if ensure_ring {
            last = seq.size() - 1;
        }

        // fill in values
        for j in 0..last {
            {
                for k in 0..seq.get_dimension() {
                    seq.set_ordinate(
                        j,
                        k,
                        copy.get_ordinate((index_of_first_coordinate + j) % last, k),
                    );
                }
            }

            // Fix the ring (first == last)
            if ensure_ring {
                for k in 0..seq.get_dimension() {
                    seq.set_ordinate(last, k, seq.get_ordinate(0, k));
                }
            }
        }
    }

    /**
     *  Returns the index of <code>coordinate</code> in a {@link CoordinateSequence}
     *  The first position is 0; the second, 1; etc.
     *
     *@param  coordinate   the <code>Coordinate</code> to search for
     *@param  seq  the coordinate sequence to search
     *@return              the position of <code>coordinate</code>, or -1 if it is
     *      not found
     */
    pub fn index_of(coordinate: &Coordinate, seq: &CoordinateArraySequence) -> i32 {
        for i in 0..seq.size() {
            if coordinate.x == seq.get_ordinate(i, CoordinateArraySequence::X)
                && coordinate.y == seq.get_ordinate(i, CoordinateArraySequence::Y)
            {
                return i as i32;
            }
        }
        return -1;
    }
}
