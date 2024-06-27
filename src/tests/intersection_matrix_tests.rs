#[cfg(test)]
mod intersection_matrix_tests {
    use crate::core::geom::{dimension::Dimension, intersection_matrix::IntersectionMatrix};

    #[test]
    fn test_to_string() {
        let mut i = IntersectionMatrix::default();
        i.set_string("012*TF012".to_owned());
        assert_eq!("012*TF012", i.to_string());

        let c = IntersectionMatrix::new_from_intersection_matrix(&i);
        assert_eq!("012*TF012", c.to_string());
    }

    #[test]
    fn test_transpose() {
        let x = IntersectionMatrix::new_with_elements("012*TF012".to_owned());

        let mut i = IntersectionMatrix::new_from_intersection_matrix(&x);
        i.transpose();

        assert_eq!("0*01T12F2", i.to_string());
        assert_eq!("012*TF012", x.to_string());
    }

    #[test]
    fn test_is_disjoint() {
        assert!((IntersectionMatrix::new_with_elements("FF*FF****".to_owned())).is_disjoint());
        assert!((IntersectionMatrix::new_with_elements("FF1FF2T*0".to_owned())).is_disjoint());
        assert!(!(IntersectionMatrix::new_with_elements("*F*FF****".to_owned())).is_disjoint());
    }

    #[test]
    fn test_is_touches() {
        assert!(
            (IntersectionMatrix::new_with_elements("FT*******".to_owned()))
                .is_touches(Dimension::P, Dimension::A)
        );
        assert!(
            (IntersectionMatrix::new_with_elements("FT*******".to_owned()))
                .is_touches(Dimension::A, Dimension::P)
        );
        assert!(
            !(IntersectionMatrix::new_with_elements("FT*******".to_owned()))
                .is_touches(Dimension::P, Dimension::P)
        );
    }

    #[test]
    fn test_is_intersects() {
        assert!(!(IntersectionMatrix::new_with_elements("FF*FF****".to_owned())).is_intersects());
        assert!(!(IntersectionMatrix::new_with_elements("FF1FF2T*0".to_owned())).is_intersects());
        assert!((IntersectionMatrix::new_with_elements("*F*FF****".to_owned())).is_intersects());
    }

    #[test]
    fn test_is_crosses() {
        assert!(
            (IntersectionMatrix::new_with_elements("TFTFFFFFF".to_owned()))
                .is_crosses(Dimension::P, Dimension::L)
        );
        assert!(
            !(IntersectionMatrix::new_with_elements("TFTFFFFFF".to_owned()))
                .is_crosses(Dimension::L, Dimension::P)
        );
        assert!(
            !(IntersectionMatrix::new_with_elements("TFFFFFTFF".to_owned()))
                .is_crosses(Dimension::P, Dimension::L)
        );
        assert!(
            (IntersectionMatrix::new_with_elements("TFFFFFTFF".to_owned()))
                .is_crosses(Dimension::L, Dimension::P)
        );
        assert!(
            (IntersectionMatrix::new_with_elements("0FFFFFFFF".to_owned()))
                .is_crosses(Dimension::L, Dimension::L)
        );
        assert!(
            !(IntersectionMatrix::new_with_elements("1FFFFFFFF".to_owned()))
                .is_crosses(Dimension::L, Dimension::L)
        );
    }

    #[test]
    fn test_is_within() {
        assert!((IntersectionMatrix::new_with_elements("T0F00F000".to_owned())).is_within());
        assert!(!(IntersectionMatrix::new_with_elements("T00000FF0".to_owned())).is_within());
    }

    #[test]
    fn test_is_contains() {
        assert!(!(IntersectionMatrix::new_with_elements("T0F00F000".to_owned())).is_contains());
        assert!((IntersectionMatrix::new_with_elements("T00000FF0".to_owned())).is_contains());
    }

    #[test]
    fn test_is_overlaps() {
        assert!(
            (IntersectionMatrix::new_with_elements("2*2***2**".to_owned()))
                .is_overlaps(Dimension::P, Dimension::P)
        );
        assert!(
            (IntersectionMatrix::new_with_elements("2*2***2**".to_owned()))
                .is_overlaps(Dimension::A, Dimension::A)
        );
        assert!(
            !(IntersectionMatrix::new_with_elements("2*2***2**".to_owned()))
                .is_overlaps(Dimension::P, Dimension::A)
        );
        assert!(
            !(IntersectionMatrix::new_with_elements("2*2***2**".to_owned()))
                .is_overlaps(Dimension::L, Dimension::L)
        );
        assert!(
            (IntersectionMatrix::new_with_elements("1*2***2**".to_owned()))
                .is_overlaps(Dimension::L, Dimension::L)
        );

        assert!(
            !(IntersectionMatrix::new_with_elements("0FFFFFFF2".to_owned()))
                .is_overlaps(Dimension::P, Dimension::P)
        );
        assert!(
            !(IntersectionMatrix::new_with_elements("1FFF0FFF2".to_owned()))
                .is_overlaps(Dimension::L, Dimension::L)
        );
        assert!(
            !(IntersectionMatrix::new_with_elements("2FFF1FFF2".to_owned()))
                .is_overlaps(Dimension::A, Dimension::A)
        );
    }

    #[test]
    fn test_is_equals() {
        assert!(
            (IntersectionMatrix::new_with_elements("0FFFFFFF2".to_owned()))
                .is_equals(Dimension::P, Dimension::P)
        );
        assert!(
            (IntersectionMatrix::new_with_elements("1FFF0FFF2".to_owned()))
                .is_equals(Dimension::L, Dimension::L)
        );
        assert!(
            (IntersectionMatrix::new_with_elements("2FFF1FFF2".to_owned()))
                .is_equals(Dimension::A, Dimension::A)
        );

        assert!(
            !(IntersectionMatrix::new_with_elements("0F0FFFFF2".to_owned()))
                .is_equals(Dimension::P, Dimension::P)
        );
        assert!(
            (IntersectionMatrix::new_with_elements("1FFF1FFF2".to_owned()))
                .is_equals(Dimension::L, Dimension::L)
        );
        assert!(
            !(IntersectionMatrix::new_with_elements("2FFF1*FF2".to_owned()))
                .is_equals(Dimension::A, Dimension::A)
        );

        assert!(
            !(IntersectionMatrix::new_with_elements("0FFFFFFF2".to_owned()))
                .is_equals(Dimension::P, Dimension::L)
        );
        assert!(
            !(IntersectionMatrix::new_with_elements("1FFF0FFF2".to_owned()))
                .is_equals(Dimension::L, Dimension::A)
        );
        assert!(
            !(IntersectionMatrix::new_with_elements("2FFF1FFF2".to_owned()))
                .is_equals(Dimension::A, Dimension::P)
        );
    }
}
