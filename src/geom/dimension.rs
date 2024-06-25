/**
 * Provides constants representing the dimensions of a point, a curve and a surface.
 * Also provides constants representing the dimensions of the empty geometry and
 * non-empty geometries, and the wildcard constant {@link #DONTCARE} meaning "any dimension".
 * These constants are used as the entries in {@link IntersectionMatrix}s.
 * 
 * @version 1.7
 */

pub struct Dimension {

}

impl Dimension {
/**
   *  Dimension value of a point (0).
   */
  pub const P: i32 = 0;

  /**
   *  Dimension value of a curve (1).
   */
  pub const L: i32 = 1;

  /**
   *  Dimension value of a surface (2).
   */
  pub const A: i32 = 2;

  /**
   *  Dimension value of the empty geometry (-1).
   */
  pub const FALSE: i32 = -1;

  /**
   *  Dimension value of non-empty geometries (= {P, L, A}).
   */
  pub const TRUE: i32 = -2;

  /**
   *  Dimension value for any dimension (= {FALSE, TRUE}).
   */
  pub const DONTCARE: i32 = -3;

  /**
   * Symbol for the FALSE pattern matrix entry
   */
  pub const SYM_FALSE: char = 'F';
  
  /**
   * Symbol for the TRUE pattern matrix entry
   */
  pub const SYM_TRUE: char = 'T';
  
  /**
   * Symbol for the DONTCARE pattern matrix entry
   */
  pub const SYM_DONTCARE: char = '*';
  
  /**
   * Symbol for the P (dimension 0) pattern matrix entry
   */
  pub const SYM_P: char = '0';
  
  /**
   * Symbol for the L (dimension 1) pattern matrix entry
   */
  pub const SYM_L: char = '1';
  
  /**
   * Symbol for the A (dimension 2) pattern matrix entry
   */
  pub const SYM_A: char = '2';
  
  /**
   *  Converts the dimension value to a dimension symbol, for example, <code>TRUE =&gt; 'T'</code>
   *  .
   *
   *@param  dimensionValue  a number that can be stored in the <code>IntersectionMatrix</code>
   *      . Possible values are <code>{TRUE, FALSE, DONTCARE, 0, 1, 2}</code>.
   *@return                 a character for use in the string representation of
   *      an <code>IntersectionMatrix</code>. Possible values are <code>{T, F, * , 0, 1, 2}</code>
   *      .
   */
  pub fn to_dimension_symbol(dimension_value: i32) -> Option<char> {
    match dimension_value {
        Dimension::FALSE => return Some(Dimension::SYM_FALSE),
        Dimension::TRUE => return Some(Dimension::SYM_TRUE),
        Dimension::DONTCARE => return Some(Dimension::SYM_DONTCARE),
        Dimension::P => return Some(Dimension::SYM_P),
        Dimension::L => return Some(Dimension::SYM_L),
        Dimension::A => return Some(Dimension::SYM_A),
        _ => None,
    }
  }

  /**
   *  Converts the dimension symbol to a dimension value, for example, <code>'*' =&gt; DONTCARE</code>
   *  .
   *
   *@param  dimensionSymbol  a character for use in the string representation of
   *      an <code>IntersectionMatrix</code>. Possible values are <code>{T, F, * , 0, 1, 2}</code>
   *      .
   *@return a number that can be stored in the <code>IntersectionMatrix</code>
   *      . Possible values are <code>{TRUE, FALSE, DONTCARE, 0, 1, 2}</code>.
   */
  pub fn to_dimension_value(dimension_symbol: char) -> Option<i32> {
    match char::to_ascii_uppercase(&dimension_symbol) {
      Dimension::SYM_FALSE => return Some(Dimension::FALSE),
      Dimension::SYM_TRUE => return Some(Dimension::TRUE),
      Dimension::SYM_DONTCARE => return Some(Dimension::DONTCARE),
      Dimension::SYM_P => return Some(Dimension::P),
      Dimension::SYM_L => return Some(Dimension::L),
      Dimension::SYM_A => return Some(Dimension::A),
      _ => return None,
    }
  }
}