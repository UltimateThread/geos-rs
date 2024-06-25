#[cfg(test)]
mod coordinate_array_sequences_tests {
    use crate::geom::{coordinate::Coordinate, coordinate_array_sequences::CoordinateArraySequences, implementation::{coordinate_array_sequence::CoordinateArraySequence, coordinate_array_sequence_factory::CoordinateArraySequenceFactory}, precision_model::PrecisionModel};

    const ORDINATE_VALUES: [[f64; 2]; 20] = [
        [75.76, 77.43],
        [41.35, 90.75],
        [73.74, 41.67],
        [20.87, 86.49],
        [17.49, 93.59],
        [67.75, 80.63],
        [63.01, 52.57],
        [32.9, 44.44],
        [79.36, 29.8],
        [38.17, 88.0],
        [19.31, 49.71],
        [57.03, 19.28],
        [63.76, 77.35],
        [45.26, 85.15],
        [51.71, 50.38],
        [92.16, 19.85],
        [64.18, 27.7],
        [64.74, 65.1],
        [80.07, 13.55],
        [55.54, 94.07],
    ];

    #[test]
    fn test_copy_to_larger_dim() {
        let cs2d = create_test_sequence(10, 2);
        let mut cs3d = CoordinateArraySequenceFactory::create_with_size_dimension(10, 3);
        let cs3d_size = cs3d.size();
        CoordinateArraySequences::copy(&cs2d, 0, &mut cs3d, 0, cs3d_size);
        assert!(CoordinateArraySequences::is_equal(&cs2d, &cs3d));
    }

    #[test]
    fn test_copy_to_smaller_dim() {
        let cs3d = create_test_sequence(10, 3);
        let mut cs2d = CoordinateArraySequenceFactory::create_with_size_dimension(10, 2);
        let cs2d_size = cs2d.size();
        CoordinateArraySequences::copy(&cs3d, 0, &mut cs2d, 0, cs2d_size);
        assert!(CoordinateArraySequences::is_equal(&cs2d, &cs3d));
    }

    #[test]
    fn test_scroll_ring() {
        do_test_scroll_ring(2);
        do_test_scroll_ring(3);
        do_test_scroll_ring(4);
    }

    #[test]
    fn test_scroll() {
        do_test_scroll(2);
        do_test_scroll(3);
        do_test_scroll(4);
    }

    #[test]
    fn test_index_of() {
        do_test_index_of(2);
        do_test_index_of(5);
        do_test_index_of(7);
    }

    #[test]
    fn test_min_coordinate_index() {
        do_test_min_coordinate_index(2);
        do_test_min_coordinate_index(5);
        do_test_min_coordinate_index(7);
    }

    #[test]
    fn test_is_ring() {
        do_test_is_ring(2);
        do_test_is_ring(5);
        do_test_is_ring(7);
    }

    #[test]
    fn test_copy() {
        do_test_copy(2);
        do_test_copy(5);
        do_test_copy(7);
    }

    #[test]
    fn test_reverse() {
        do_test_reverse(2);
        do_test_reverse(5);
        do_test_reverse(7);
    }

    fn create_sequence_from_ordinates(dim: i32) -> CoordinateArraySequence {
        let mut sequence =
        CoordinateArraySequenceFactory::create_with_size_dimension(ORDINATE_VALUES.len(), dim);
        for i in 0..ORDINATE_VALUES.len() {
            sequence.set_ordinate(i, 0, ORDINATE_VALUES[i][0]);
            sequence.set_ordinate(i, 1, ORDINATE_VALUES[i][1]);
        }
        return fill_non_planar_dimensions(&sequence);
    }

    fn create_test_sequence(size: usize, dim: i32) -> CoordinateArraySequence {
        let mut cs = CoordinateArraySequenceFactory::create_with_size_dimension(size, dim);
        // initialize with a data signature where coords look like [1, 10, 100, ...]
        for i in 0..size {
            for d in 0..dim {
                let value = i * i32::pow(10, d as u32) as usize;
                cs.set_ordinate(i, d, value as f64);
            }
        }
        return cs;
    }

    fn do_test_reverse(dimension: i32) {
        // arrange
        let sequence = create_sequence_from_ordinates(dimension);
        let mut reversed = sequence.copy();

        // act
        CoordinateArraySequences::reverse(&mut reversed);

        // assert
        for i in 0..sequence.size() {
            check_coordinate_at(&sequence, i, &reversed, sequence.size() - i - 1, dimension);
        }
    }

    fn do_test_copy(dimension: i32) {
        // arrange
        let sequence = create_sequence_from_ordinates(dimension);
        if sequence.size() <= 7 {
            return;
        }

        let mut full_copy =
        CoordinateArraySequenceFactory::create_with_size_dimension(sequence.size(), dimension);
        let mut partial_copy = CoordinateArraySequenceFactory::create_with_size_dimension(
            sequence.size() - 5,
            dimension,
        );

        // act
        CoordinateArraySequences::copy(&sequence, 0, &mut full_copy, 0, sequence.size());
        let partial_copy_size = partial_copy.size();
        CoordinateArraySequences::copy(&sequence, 2, &mut partial_copy, 0, partial_copy_size);

        // assert
        for i in 0..full_copy.size() {
            check_coordinate_at(&sequence, i, &mut full_copy, i, dimension);
        }
        for i in 0..partial_copy.size() {
            check_coordinate_at(&sequence, 2 + i, &mut partial_copy, i, dimension);
        }

        // ToDo test if dimensions don't match
    }

    fn do_test_is_ring(dimension: i32) {
        // arrange
        let ring = create_circle(dimension, Coordinate::default(), 5.);
        let no_ring = create_circular_string(dimension, Coordinate::default(), 5., 0.1, 22);
        let empty = create_almost_ring(dimension, 0);
        let incomplete1 = create_almost_ring(dimension, 1);
        let incomplete2 = create_almost_ring(dimension, 2);
        let incomplete3 = create_almost_ring(dimension, 3);
        let incomplete4a = create_almost_ring(dimension, 4);
        // TODO: Fix me!
        // let incomplete4b = CoordinateArraySequences::ensure_valid_ring(&incomplete4a);

        // act
        let is_ring_ring = CoordinateArraySequences::is_ring(&ring);
        let is_ring_no_ring = CoordinateArraySequences::is_ring(&no_ring);
        let is_ring_empty = CoordinateArraySequences::is_ring(&empty);
        let is_ring_incomplete1 = CoordinateArraySequences::is_ring(&incomplete1);
        let is_ring_incomplete2 = CoordinateArraySequences::is_ring(&incomplete2);
        let is_ring_incomplete3 = CoordinateArraySequences::is_ring(&incomplete3);
        let is_ring_incomplete4a = CoordinateArraySequences::is_ring(&incomplete4a);
        // TODO: Fix me!
        // let is_ring_incomplete4b = CoordinateArraySequences::is_ring(&incomplete4b);

        // assert
        assert!(is_ring_ring);
        assert!(!is_ring_no_ring);
        assert!(is_ring_empty);
        assert!(!is_ring_incomplete1);
        assert!(!is_ring_incomplete2);
        assert!(!is_ring_incomplete3);
        assert!(!is_ring_incomplete4a);
        // TODO: Fix me!
        // assert!(is_ring_incomplete4b);
    }

    fn do_test_index_of(dimension: i32) {
        // arrange
        let sequence = create_sequence_from_ordinates(dimension);

        // act & assert
        let coordinates = sequence.to_coordinate_array();
        for i in 0..sequence.size() {
            assert_eq!(
                i,
                CoordinateArraySequences::index_of(&coordinates[i], &sequence) as usize
            );
        }
    }

    fn do_test_min_coordinate_index(dimension: i32) {
        let mut sequence = create_sequence_from_ordinates(dimension);
        if sequence.size() <= 6 {
            return;
        }

        let min_index = sequence.size() / 2;
        sequence.set_ordinate(min_index, 0, 5.);
        sequence.set_ordinate(min_index, 1, 5.);

        assert_eq!(
            min_index,
            CoordinateArraySequences::min_coordinate_index_sequence(&sequence)
        );
        assert_eq!(
            min_index,
            CoordinateArraySequences::min_coordinate_index_sequence_from_to(
                &sequence,
                2,
                sequence.size() - 2
            )
        );
    }

    fn do_test_scroll(dimension: i32) {
        // arrange
        let sequence = create_circular_string(dimension, Coordinate::new_xy(20., 20.), 7., 0.1, 22);
        let mut scrolled = sequence.copy();

        // act
        CoordinateArraySequences::scroll_coordinate_index(&mut scrolled, 12);

        // assert
        let mut io = 12;
        for is in 0..scrolled.size() {
            check_coordinate_at(&sequence, io, &scrolled, is, dimension);
            io += 1;
            io %= scrolled.size();
        }
    }

    fn do_test_scroll_ring(dimension: i32) {
        // arrange
        let sequence = create_circle(dimension, Coordinate::new_xy(10., 10.), 9.);
        let mut scrolled = sequence.copy();

        // act
        CoordinateArraySequences::scroll_coordinate_index(&mut scrolled, 12);

        // assert
        let mut io = 12;
        for is in 0..scrolled.size() {
            check_coordinate_at(&sequence, io, &scrolled, is, dimension);
            io += 1;
            io %= scrolled.size() - 1;
        }
        let scrolled_size = scrolled.size() - 1;
        check_coordinate_at(&scrolled, 0, &scrolled, scrolled_size, dimension);
    }

    fn check_coordinate_at(
        seq1: &CoordinateArraySequence,
        pos1: usize,
        seq2: &CoordinateArraySequence,
        pos2: usize,
        dim: i32,
    ) {
        assert_eq!(seq1.get_ordinate(pos1, 0), seq2.get_ordinate(pos2, 0));
        assert_eq!(seq1.get_ordinate(pos1, 1), seq2.get_ordinate(pos2, 1));

        // check additional ordinates
        for j in 2..dim {
            if seq1.get_ordinate(pos1, j).is_nan() {
                assert!(seq1.get_ordinate(pos1, j).is_nan());
                assert!(seq2.get_ordinate(pos2, j).is_nan());
            } else {
                assert_eq!(seq1.get_ordinate(pos1, j), seq2.get_ordinate(pos2, j));
            }
        }
    }

    fn create_almost_ring(dimension: i32, mut num: usize) -> CoordinateArraySequence {
        if num > 4 {
            num = 4;
        }

        let mut sequence =
        CoordinateArraySequenceFactory::create_with_size_dimension(num, dimension);
        if num == 0 {
            return fill_non_planar_dimensions(&sequence);
        }

        sequence.set_ordinate(0, 0, 10.);
        sequence.set_ordinate(0, 0, 10.);
        if num == 1 {
            return fill_non_planar_dimensions(&sequence);
        }

        sequence.set_ordinate(0, 0, 20.);
        sequence.set_ordinate(0, 0, 10.);
        if num == 2 {
            return fill_non_planar_dimensions(&sequence);
        }

        sequence.set_ordinate(0, 0, 20.);
        sequence.set_ordinate(0, 0, 20.);
        if num == 3 {
            return fill_non_planar_dimensions(&sequence);
        }

        sequence.set_ordinate(0, 0, 10.0000000000001);
        sequence.set_ordinate(0, 0, 9.9999999999999);
        return fill_non_planar_dimensions(&sequence);
    }

    fn fill_non_planar_dimensions(
        seq: &CoordinateArraySequence,
    ) -> CoordinateArraySequence {
        let mut copy = seq.copy();
        if copy.get_dimension() < 3 {
            return copy;
        }

        for i in 0..copy.size() {
            for j in 2..copy.get_dimension() {
                let value = i as i32 * i32::pow(10, j as u32 - 1);
                copy.set_ordinate(i, j, value as f64);
            }
        }

        return copy;
    }

    fn create_circle(
        dimension: i32,
        center: Coordinate,
        radius: f64,
    ) -> CoordinateArraySequence {
        // Get a complete circular string
        let mut res = create_circular_string(dimension, center, radius, 0., 49);

        // ensure it is closed
        for i in 0..dimension {
            res.set_ordinate(48, i, res.get_ordinate(0, i));
        }

        return res;
    }

    fn create_circular_string(
        dimension: i32,
        center: Coordinate,
        radius: f64,
        start_angle: f64,
        num_points: i32,
    ) -> CoordinateArraySequence {
        let num_segments_circle = 48.;
        let angle_circle = 2. * std::f64::consts::PI;
        let angle_step = angle_circle / num_segments_circle;

        let mut sequence = CoordinateArraySequenceFactory::create_with_size_dimension(
            num_points as usize,
            dimension,
        );
        let mut pm = PrecisionModel::new_with_scale(100.);
        let mut angle = start_angle;
        for i in 0..num_points {
            let dx = f64::cos(angle) * radius;
            sequence.set_ordinate(i as usize, 0, pm.make_precise(center.x + dx));
            let dy = f64::sin(angle) * radius;
            sequence.set_ordinate(i as usize, 1, pm.make_precise(center.y + dy));

            // set other ordinate values to predictable values
            for j in 2..dimension {
                let value = i32::pow(10, j as u32 - 1) * i;
                sequence.set_ordinate(i as usize, j, value as f64);
            }

            angle += angle_step;
            angle %= angle_circle;
        }

        return sequence;
    }
}
