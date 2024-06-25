#[cfg(test)]
mod coordinate_list_tests {
    use crate::geom::{coordinate::Coordinate, coordinate_list::CoordinateList};

    #[test]
    fn test_forward() {
        let coords1: Vec<f64> = vec![0., 0., 1., 1., 2., 2.];
        let coords2: Vec<f64> = vec![0., 0., 1., 1., 2., 2.];
        check_value(
            coord_list(coords1).to_coordinate_array_forward(true),
            coords2,
        );
    }

    #[test]
    fn test_reverse() {
        let coords1: Vec<f64> = vec![0., 0., 1., 1., 2., 2.];
        let coords2: Vec<f64> = vec![2., 2., 1., 1., 0., 0.];
        check_value(
            coord_list(coords1).to_coordinate_array_forward(false),
            coords2,
        );
    }

    #[test]
    fn test_reverse_empty() {
        check_value(
            coord_list(vec![]).to_coordinate_array_forward(false),
            vec![],
        );
    }

    fn check_value(coord_array: Vec<Coordinate>, ords: Vec<f64>) {
        assert_eq!(coord_array.len() * 2, ords.len());

        let mut i = 0;
        while i < coord_array.len() {
            let pt = coord_array[i];
            assert_eq!(pt.get_x(), ords[2 * i]);
            assert_eq!(pt.get_y(), ords[2 * i + 1]);
            i += 2;
        }
    }

    fn coord_list(ords: Vec<f64>) -> CoordinateList {
        let mut cl = CoordinateList::default();
        let mut i = 0;
        while i < ords.len() {
            cl.add_coordinate_repeated(Coordinate::new_xy(ords[i], ords[i + 1]), false);
            i += 2;
        }
        return cl;
    }
}
