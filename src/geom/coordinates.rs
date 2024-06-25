use super::coordinate::Coordinate;


pub struct Coordinates {

}

impl Coordinates {
   /**
   * Factory method providing access to common Coordinate implementations.
   * 
   * @param dimension
   * @return created coordinate
   */
  pub fn create_dim(dimension: i32) -> Coordinate {
    return Coordinates::create_dim_measures(dimension, 0);
  }

  /**
   * Factory method providing access to common Coordinate implementations.
   * 
   * @param dimension
   * @param measures
   * @return created coordinate
   */
  pub fn create_dim_measures(dimension: i32, measures: i32) -> Coordinate {
    if dimension == 2 {
      return Coordinate::new_coordinatexy_default();
    } else if dimension == 3 && measures == 0 {
      return Coordinate::default();
    } else if dimension == 3 && measures == 1 {
      return Coordinate::new_coordinatexym_default();
    } else if dimension == 4 && measures == 1 {
      return Coordinate::new_coordinatexyzm_default();
    }
    return Coordinate::default();
  }
  
  /**
   * Determine dimension based on subclass of {@link Coordinate}.
   * 
   * @param coordinate supplied coordinate
   * @return number of ordinates recorded
   */
  pub fn dimension(coordinate: &Coordinate) -> i32 {
    if coordinate.is_xy() {
      return 2;
    } else if coordinate.is_xym() {
      return 3;
    } else if coordinate.is_xyzm() {
      return 4;      
    }
    return 3;
  }

  /**
   * Determine number of measures based on subclass of {@link Coordinate}.
   * 
   * @param coordinate supplied coordinate
   * @return number of measures recorded
   */
  pub fn measures(coordinate: &Coordinate) -> i32 {
    if coordinate.is_xy() {
      return 0;
    } else if coordinate.is_xym() {
      return 1;
    } else if coordinate.is_xyzm() {
      return 1;
    }
    return 0;
  }

  pub fn equal(a: &Coordinate, b: &Coordinate, tolerance: f64) -> bool {
    if tolerance == 0. { return a.equals_2d(b); }
    return a.distance(b) <= tolerance;
  }
}