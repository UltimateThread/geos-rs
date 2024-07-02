#[cfg(test)]
mod centroid_tests {
    use crate::core::geom::{
        coordinate::Coordinate, geometry_factory::GeometryFactory, multi_polygon::MultiPolygon,
        polygon::Polygon,
    };

    const TOLERANCE: f64 = 1e-10;

    /** Compute the centroid of a geometry as an area-weighted average of the centroids
     * of its components.
     *
     * @param g a polygonal geometry
     * @return Coordinate of the geometry's centroid
     */
    fn area_weighted_centroid(multi_polygon: &MultiPolygon) -> Coordinate {
        let total_area = multi_polygon.get_area();
        let mut cx = 0.;
        let mut cy = 0.;

        for i in 0..multi_polygon.get_num_polygons() {
            let component = multi_polygon.get_polygon_at_index(i);
            let area_fraction = component.get_area() / total_area;

            let component_centroid = component.get_centroid().get_coordinate();

            cx += area_fraction * component_centroid.unwrap().x;
            cy += area_fraction * component_centroid.unwrap().y;
        }

        return Coordinate::new_xy(cx, cy);
    }

    #[test]
    fn test_centroid_multi_polygon() {
        // Verify that the computed centroid of a MultiPolygon is equivalent to the
        // area-weighted average of its components.
        let mut polygons: Vec<Polygon> = vec![];
        let coords1: Vec<Coordinate> = vec![
            Coordinate::new_xy(-92.661322, 36.58994900000003),
            Coordinate::new_xy(-92.66132199999993, 36.58994900000005),
            Coordinate::new_xy(-92.66132199999993, 36.589949000000004),
            Coordinate::new_xy(-92.661322, 36.589949),
            Coordinate::new_xy(-92.661322, 36.5899490000000),
        ];
        let p1 = GeometryFactory::create_polygon_with_coordinates(&coords1);
        polygons.push(p1);

        let coords2: Vec<Coordinate> = vec![
            Coordinate::new_xy(-92.65560500000008, 36.58708800000005),
            Coordinate::new_xy(-92.65560499999992, 36.58708800000005),
            Coordinate::new_xy(-92.65560499998745, 36.587087999992576),
            Coordinate::new_xy(-92.655605, 36.587088),
            Coordinate::new_xy(-92.65560500000008, 36.5870880000000),
        ];
        let p2 = GeometryFactory::create_polygon_with_coordinates(&coords2);
        polygons.push(p2);

        let coords3: Vec<Coordinate> = vec![
            Coordinate::new_xy(-92.65512450000065, 36.586800000000466),
            Coordinate::new_xy(-92.65512449999994, 36.58680000000004),
            Coordinate::new_xy(-92.65512449998666, 36.5867999999905),
            Coordinate::new_xy(-92.65512450000065, 36.58680000000046),
        ];
        let p3 = GeometryFactory::create_polygon_with_coordinates(&coords3);
        polygons.push(p3);

        let multi_polygon = MultiPolygon::new_with_polygons(&polygons);

        assert!(
            area_weighted_centroid(&multi_polygon).equals_2d_with_tolerance(
                &multi_polygon.get_centroid().get_coordinate().unwrap(),
                TOLERANCE
            )
        );
    }
}
