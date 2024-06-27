use super::{dimension::Dimension, location::Location};

/**
 * Models a <b>Dimensionally Extended Nine-Intersection Model (DE-9IM)</b> matrix.
 * DE-9IM matrix values (such as "212FF1FF2")
 * specify the topological relationship between two {@link Geometry}s.
 * This class can also represent matrix patterns (such as "T*T******")
 * which are used for matching instances of DE-9IM matrices.
 * <p>
 * DE-9IM matrices are 3x3 matrices with integer entries.
 * The matrix indices {0,1,2} represent the topological locations
 * that occur in a geometry (Interior, Boundary, Exterior).  
 * These are provided by the constants
 * {@link Location#INTERIOR}, {@link Location#BOUNDARY}, and {@link Location#EXTERIOR}.
 * <p>
 * When used to specify the topological relationship between two geometries,
 * the matrix entries represent the possible dimensions of each intersection:
 * {@link Dimension#A} = 2, {@link Dimension#L} = 1, {@link Dimension#P} = 0 and {@link Dimension#FALSE} = -1.
 * When used to represent a matrix pattern entries can have the additional values
 * {@link Dimension#TRUE} {"T") and {@link Dimension#DONTCARE} ("*").
 * <p>
 * For a description of the DE-9IM and the spatial predicates derived from it,
 * see the following references:
 * <ul>
 * <li><i><a href="http://www.opengis.org/techno/specs.htm">
 * OGC 99-049 OpenGIS Simple Features Specification for SQL</a></i>
 * , Section 2.1.13</li>
 * <li><i><a href="http://portal.opengeospatial.org/files/?artifact_id=25355">
 * OGC 06-103r4 OpenGIS Implementation Standard for Geographic information - Simple feature access - Part 1: Common architecture</a></i>
 * , Section 6.1.15 (which provides some further details on certain predicate specifications).
 * </li>
 * <li>Wikipedia article on <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a></li>
 * </ul>
 * <p>
 * Methods are provided to:
 *  <UL>
 *    <LI>set and query the elements of the matrix in a convenient fashion
 *    <LI>convert to and from the standard string representation (specified in
 *    SFS Section 2.1.13.2).
 *    <LI>test if a matrix matches a given pattern string.
 *    <li>test if a matrix (possibly with geometry dimensions) matches a standard named spatial predicate
 *  </UL>
 *
 *@version 1.7
 */

pub struct IntersectionMatrix {
    /**
     *  Internal representation of this <code>IntersectionMatrix</code>.
     */
    matrix: [[i32; 3]; 3],
}

impl IntersectionMatrix {
    /**
     *  Creates an <code>IntersectionMatrix</code> with <code>FALSE</code>
     *  dimension values.
     */
    pub fn default() -> Self {
        Self {
            matrix: [[Dimension::FALSE; 3]; 3],
        }
    }

    /**
     *  Creates an <code>IntersectionMatrix</code> with the given dimension
     *  symbols.
     *
     *@param  elements  a String of nine dimension symbols in row major order
     */
    pub fn new_with_elements(elements: String) -> Self {
        let mut new = IntersectionMatrix::default();
        new.set_string(elements);
        new
    }

    /**
     *  Creates an <code>IntersectionMatrix</code> with the same elements as
     *  <code>other</code>.
     *
     *@param  other  an <code>IntersectionMatrix</code> to copy
     */
    pub fn new_from_intersection_matrix(other: &IntersectionMatrix) -> Self {
        let mut new = IntersectionMatrix::default();
        new.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize] =
            other.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize];
        new.matrix[Location::INTERIOR as usize][Location::BOUNDARY as usize] =
            other.matrix[Location::INTERIOR as usize][Location::BOUNDARY as usize];
        new.matrix[Location::INTERIOR as usize][Location::EXTERIOR as usize] =
            other.matrix[Location::INTERIOR as usize][Location::EXTERIOR as usize];
        new.matrix[Location::BOUNDARY as usize][Location::INTERIOR as usize] =
            other.matrix[Location::BOUNDARY as usize][Location::INTERIOR as usize];
        new.matrix[Location::BOUNDARY as usize][Location::BOUNDARY as usize] =
            other.matrix[Location::BOUNDARY as usize][Location::BOUNDARY as usize];
        new.matrix[Location::BOUNDARY as usize][Location::EXTERIOR as usize] =
            other.matrix[Location::BOUNDARY as usize][Location::EXTERIOR as usize];
        new.matrix[Location::EXTERIOR as usize][Location::INTERIOR as usize] =
            other.matrix[Location::EXTERIOR as usize][Location::INTERIOR as usize];
        new.matrix[Location::EXTERIOR as usize][Location::BOUNDARY as usize] =
            other.matrix[Location::EXTERIOR as usize][Location::BOUNDARY as usize];
        new.matrix[Location::EXTERIOR as usize][Location::EXTERIOR as usize] =
            other.matrix[Location::EXTERIOR as usize][Location::EXTERIOR as usize];
        new
    }

    /**
     * Adds one matrix to another.
     * Addition is defined by taking the maximum dimension value of each position
     * in the summand matrices.
     *
     * @param im the matrix to add
     */
    pub fn add(&mut self, im: &IntersectionMatrix) {
        for i in 0..3 {
            for j in 0..3 {
                self.set_at_least_row_column_dimension(i, j, im.get(i, j));
            }
        }
    }

    /**
     *  Tests if the dimension value matches <tt>TRUE</tt>
     *  (i.e.  has value 0, 1, 2 or TRUE).
     *
     *@param  actualDimensionValue     a number that can be stored in the <code>IntersectionMatrix</code>
     *      . Possible values are <code>{TRUE, FALSE, DONTCARE, 0, 1, 2}</code>.
     *@return true if the dimension value matches TRUE
     */
    pub fn is_true(&self, actual_dimension_value: i32) -> bool {
        if actual_dimension_value >= 0 || actual_dimension_value == Dimension::TRUE {
            return true;
        }
        return false;
    }

    /**
     *  Tests if the dimension value satisfies the dimension symbol.
     *
     *@param  actualDimensionValue     a number that can be stored in the <code>IntersectionMatrix</code>
     *      . Possible values are <code>{TRUE, FALSE, DONTCARE, 0, 1, 2}</code>.
     *@param  requiredDimensionSymbol  a character used in the string
     *      representation of an <code>IntersectionMatrix</code>. Possible values
     *      are <code>{T, F, * , 0, 1, 2}</code>.
     *@return                          true if the dimension symbol matches
     *      the dimension value
     */
    pub fn matches_i32_char(actual_dimension_value: i32, required_dimension_symbol: char) -> bool {
        if required_dimension_symbol == Dimension::SYM_DONTCARE {
            return true;
        }
        if required_dimension_symbol == Dimension::SYM_TRUE
            && (actual_dimension_value >= 0 || actual_dimension_value == Dimension::TRUE)
        {
            return true;
        }
        if required_dimension_symbol == Dimension::SYM_FALSE
            && actual_dimension_value == Dimension::FALSE
        {
            return true;
        }
        if required_dimension_symbol == Dimension::SYM_P && actual_dimension_value == Dimension::P {
            return true;
        }
        if required_dimension_symbol == Dimension::SYM_L && actual_dimension_value == Dimension::L {
            return true;
        }
        if required_dimension_symbol == Dimension::SYM_A && actual_dimension_value == Dimension::A {
            return true;
        }
        return false;
    }

    /**
     *  Tests if each of the actual dimension symbols in a matrix string satisfies the
     *  corresponding required dimension symbol in a pattern string.
     *
     *@param  actualDimensionSymbols    nine dimension symbols to validate.
     *      Possible values are <code>{T, F, * , 0, 1, 2}</code>.
     *@param  requiredDimensionSymbols  nine dimension symbols to validate
     *      against. Possible values are <code>{T, F, * , 0, 1, 2}</code>.
     *@return                           true if each of the required dimension
     *      symbols encompass the corresponding actual dimension symbol
     */
    pub fn matches_string_string(
        actual_dimension_symbols: String,
        required_dimension_symbols: String,
    ) -> bool {
        let m = IntersectionMatrix::new_with_elements(actual_dimension_symbols);
        return m.matches_string(required_dimension_symbols);
    }

    /**
     *  Changes the value of one of this <code>IntersectionMatrix</code>s
     *  elements.
     *
     *@param  row             the row of this <code>IntersectionMatrix</code>,
     *      indicating the interior, boundary or exterior of the first <code>Geometry</code>
     *@param  column          the column of this <code>IntersectionMatrix</code>,
     *      indicating the interior, boundary or exterior of the second <code>Geometry</code>
     *@param  dimensionValue  the new value of the element
     */
    pub fn set_row_column_value(&mut self, row: usize, column: usize, dimension_value: i32) {
        self.matrix[row][column] = dimension_value;
    }

    /**
     *  Changes the elements of this <code>IntersectionMatrix</code> to the
     *  dimension symbols in <code>dimensionSymbols</code>.
     *
     *@param  dimensionSymbols  nine dimension symbols to which to set this <code>IntersectionMatrix</code>
     *      s elements. Possible values are <code>{T, F, * , 0, 1, 2}</code>
     */
    pub fn set_string(&mut self, dimension_symbols: String) {
        for i in 0..dimension_symbols.len() {
            let row = i / 3;
            let col = i % 3;
            let character = dimension_symbols.chars().nth(i);
            match character {
                Some(character) => {
                    let dimension_value = Dimension::to_dimension_value(character);
                    match dimension_value {
                        Some(dimension_value) => {
                            self.matrix[row][col] = dimension_value;
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
    }

    /**
     *  Changes the specified element to <code>minimumDimensionValue</code> if the
     *  element is less.
     *
     *@param  row                    the row of this <code>IntersectionMatrix</code>
     *      , indicating the interior, boundary or exterior of the first <code>Geometry</code>
     *@param  column                 the column of this <code>IntersectionMatrix</code>
     *      , indicating the interior, boundary or exterior of the second <code>Geometry</code>
     *@param  minimumDimensionValue  the dimension value with which to compare the
     *      element. The order of dimension values from least to greatest is
     *      <code>{DONTCARE, TRUE, FALSE, 0, 1, 2}</code>.
     */
    pub fn set_at_least_row_column_dimension(
        &mut self,
        row: usize,
        column: usize,
        minimum_dimension_value: i32,
    ) {
        if self.matrix[row][column] < minimum_dimension_value {
            self.matrix[row][column] = minimum_dimension_value;
        }
    }

    /**
     *  If row &gt;= 0 and column &gt;= 0, changes the specified element to <code>minimumDimensionValue</code>
     *  if the element is less. Does nothing if row &lt;0 or column &lt; 0.
     *
     *@param  row                    the row of this <code>IntersectionMatrix</code>
     *      , indicating the interior, boundary or exterior of the first <code>Geometry</code>
     *@param  column                 the column of this <code>IntersectionMatrix</code>
     *      , indicating the interior, boundary or exterior of the second <code>Geometry</code>
     *@param  minimumDimensionValue  the dimension value with which to compare the
     *      element. The order of dimension values from least to greatest is
     *      <code>{DONTCARE, TRUE, FALSE, 0, 1, 2}</code>.
     */
    pub fn set_at_least_row_column_dimension_if_valid(
        &mut self,
        row: usize,
        column: usize,
        minimum_dimension_value: i32,
    ) {
        self.set_at_least_row_column_dimension(row, column, minimum_dimension_value);
    }

    /**
     *  For each element in this <code>IntersectionMatrix</code>, changes the
     *  element to the corresponding minimum dimension symbol if the element is
     *  less.
     *
     *@param  minimumDimensionSymbols  nine dimension symbols with which to
     *      compare the elements of this <code>IntersectionMatrix</code>. The
     *      order of dimension values from least to greatest is <code>{DONTCARE, TRUE, FALSE, 0, 1, 2}</code>
     *      .
     */
    pub fn set_at_least_string(&mut self, minimum_dimension_symbols: String) {
        for i in 0..minimum_dimension_symbols.len() {
            let row = i / 3;
            let col = i % 3;

            let character = minimum_dimension_symbols.chars().nth(i);
            match character {
                Some(character) => {
                    let dimension_value = Dimension::to_dimension_value(character);
                    match dimension_value {
                        Some(dimension_value) => {
                            self.set_at_least_row_column_dimension(row, col, dimension_value);
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
    }

    /**
     *  Changes the elements of this <code>IntersectionMatrix</code> to <code>dimensionValue</code>
     *  .
     *
     *@param  dimensionValue  the dimension value to which to set this <code>IntersectionMatrix</code>
     *      s elements. Possible values <code>{TRUE, FALSE, DONTCARE, 0, 1, 2}</code>
     *      .
     */
    pub fn set_all(&mut self, dimension_value: i32) {
        for ai in 0..3 {
            for bi in 0..3 {
                self.matrix[ai][bi] = dimension_value;
            }
        }
    }

    /**
     *  Returns the value of one of this matrix
     *  entries.
     *  The value of the provided index is one of the
     *  values from the {@link Location} class.  
     *  The value returned is a constant
     *  from the {@link Dimension} class.
     *
     *@param  row     the row of this <code>IntersectionMatrix</code>, indicating
     *      the interior, boundary or exterior of the first <code>Geometry</code>
     *@param  column  the column of this <code>IntersectionMatrix</code>,
     *      indicating the interior, boundary or exterior of the second <code>Geometry</code>
     *@return         the dimension value at the given matrix position.
     */
    pub fn get(&self, row: usize, column: usize) -> i32 {
        return self.matrix[row][column];
    }

    /**
     * Tests if this matrix matches <code>[FF*FF****]</code>.
     *
     *@return    <code>true</code> if the two <code>Geometry</code>s related by
     *      this matrix are disjoint
     */
    pub fn is_disjoint(&self) -> bool {
        return self.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize]
            == Dimension::FALSE
            && self.matrix[Location::INTERIOR as usize][Location::BOUNDARY as usize]
                == Dimension::FALSE
            && self.matrix[Location::BOUNDARY as usize][Location::INTERIOR as usize]
                == Dimension::FALSE
            && self.matrix[Location::BOUNDARY as usize][Location::BOUNDARY as usize]
                == Dimension::FALSE;
    }

    /**
     *  Tests if <code>isDisjoint</code> returns false.
     *
     *@return <code>true</code> if the two <code>Geometry</code>s related by
     *      this matrix intersect
     */
    pub fn is_intersects(&self) -> bool {
        return !self.is_disjoint();
    }

    /**
     *  Tests if this matrix matches
     *  <code>[FT*******]</code>, <code>[F**T*****]</code> or <code>[F***T****]</code>.
     *
     *@param  dimensionOfGeometryA  the dimension of the first <code>Geometry</code>
     *@param  dimensionOfGeometryB  the dimension of the second <code>Geometry</code>
     *@return                       <code>true</code> if the two <code>Geometry</code>
     *      s related by this matrix touch; Returns false
     *      if both <code>Geometry</code>s are points.
     */
    pub fn is_touches(&self, dimension_of_geometry_a: i32, dimension_of_geometry_b: i32) -> bool {
        if dimension_of_geometry_a > dimension_of_geometry_b {
            //no need to get transpose because pattern matrix is symmetrical
            return self.is_touches(dimension_of_geometry_b, dimension_of_geometry_a);
        }
        if (dimension_of_geometry_a == Dimension::A && dimension_of_geometry_b == Dimension::A)
            || (dimension_of_geometry_a == Dimension::L && dimension_of_geometry_b == Dimension::L)
            || (dimension_of_geometry_a == Dimension::L && dimension_of_geometry_b == Dimension::A)
            || (dimension_of_geometry_a == Dimension::P && dimension_of_geometry_b == Dimension::A)
            || (dimension_of_geometry_a == Dimension::P && dimension_of_geometry_b == Dimension::L)
        {
            return self.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize]
                == Dimension::FALSE
                && (self.is_true(
                    self.matrix[Location::INTERIOR as usize][Location::BOUNDARY as usize],
                ) || self.is_true(
                    self.matrix[Location::BOUNDARY as usize][Location::INTERIOR as usize],
                ) || self.is_true(
                    self.matrix[Location::BOUNDARY as usize][Location::BOUNDARY as usize],
                ));
        }
        return false;
    }

    /**
     * Tests whether this geometry crosses the
     * specified geometry.
     * <p>
     * The <code>crosses</code> predicate has the following equivalent definitions:
     * <ul>
     * <li>The geometries have some but not all interior points in common.
     * <li>The DE-9IM Intersection Matrix for the two geometries matches
     *   <ul>
     *    <li><code>[T*T******]</code> (for P/L, P/A, and L/A situations)
     *    <li><code>[T*****T**]</code> (for L/P, L/A, and A/L situations)
     *    <li><code>[0********]</code> (for L/L situations)
     *   </ul>
     * </ul>
     * For any other combination of dimensions this predicate returns <code>false</code>.
     * <p>
     * The SFS defined this predicate only for P/L, P/A, L/L, and L/A situations.
     * JTS extends the definition to apply to L/P, A/P and A/L situations as well.
     * This makes the relation symmetric.
     *
     *@param  dimensionOfGeometryA  the dimension of the first <code>Geometry</code>
     *@param  dimensionOfGeometryB  the dimension of the second <code>Geometry</code>
     *@return                       <code>true</code> if the two <code>Geometry</code>s
     *      related by this matrix cross.
     */
    pub fn is_crosses(&self, dimension_of_geometry_a: i32, dimension_of_geometry_b: i32) -> bool {
        if (dimension_of_geometry_a == Dimension::P && dimension_of_geometry_b == Dimension::L)
            || (dimension_of_geometry_a == Dimension::P && dimension_of_geometry_b == Dimension::A)
            || (dimension_of_geometry_a == Dimension::L && dimension_of_geometry_b == Dimension::A)
        {
            return self
                .is_true(self.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize])
                && self.is_true(
                    self.matrix[Location::INTERIOR as usize][Location::EXTERIOR as usize],
                );
        }
        if (dimension_of_geometry_a == Dimension::L && dimension_of_geometry_b == Dimension::P)
            || (dimension_of_geometry_a == Dimension::A && dimension_of_geometry_b == Dimension::P)
            || (dimension_of_geometry_a == Dimension::A && dimension_of_geometry_b == Dimension::L)
        {
            return self
                .is_true(self.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize])
                && self.is_true(
                    self.matrix[Location::EXTERIOR as usize][Location::INTERIOR as usize],
                );
        }
        if dimension_of_geometry_a == Dimension::L && dimension_of_geometry_b == Dimension::L {
            return self.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize] == 0;
        }
        return false;
    }

    /**
     * Tests whether this matrix matches <code>[T*F**F***]</code>.
     *
     *@return    <code>true</code> if the first <code>Geometry</code> is within
     *      the second
     */
    pub fn is_within(&self) -> bool {
        return self.is_true(self.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize])
            && self.matrix[Location::INTERIOR as usize][Location::EXTERIOR as usize]
                == Dimension::FALSE
            && self.matrix[Location::BOUNDARY as usize][Location::EXTERIOR as usize]
                == Dimension::FALSE;
    }

    /**
     * Tests whether this matrix matches [T*****FF*[.
     *
     *@return    <code>true</code> if the first <code>Geometry</code> contains the
     *      second
     */
    pub fn is_contains(&self) -> bool {
        return self.is_true(self.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize])
            && self.matrix[Location::EXTERIOR as usize][Location::INTERIOR as usize]
                == Dimension::FALSE
            && self.matrix[Location::EXTERIOR as usize][Location::BOUNDARY as usize]
                == Dimension::FALSE;
    }

    /**
     * Tests if this matrix matches
     *    <code>[T*****FF*]</code>
     * or <code>[*T****FF*]</code>
     * or <code>[***T**FF*]</code>
     * or <code>[****T*FF*]</code>
     *
     *@return    <code>true</code> if the first <code>Geometry</code> covers the
     *      second
     */
    pub fn is_covers(&self) -> bool {
        let has_point_in_common = self
            .is_true(self.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize])
            || self.is_true(self.matrix[Location::INTERIOR as usize][Location::BOUNDARY as usize])
            || self.is_true(self.matrix[Location::BOUNDARY as usize][Location::INTERIOR as usize])
            || self.is_true(self.matrix[Location::BOUNDARY as usize][Location::BOUNDARY as usize]);

        return has_point_in_common
            && self.matrix[Location::EXTERIOR as usize][Location::INTERIOR as usize]
                == Dimension::FALSE
            && self.matrix[Location::EXTERIOR as usize][Location::BOUNDARY as usize]
                == Dimension::FALSE;
    }

    /**
     *Tests if this matrix matches
     *    <code>[T*F**F***]</code>
     * or <code>[*TF**F***]</code>
     * or <code>[**FT*F***]</code>
     * or <code>[**F*TF***]</code>
     *
     *@return    <code>true</code> if the first <code>Geometry</code>
     * is covered by the second
     */
    pub fn is_covered_by(&self) -> bool {
        let has_point_in_common = self
            .is_true(self.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize])
            || self.is_true(self.matrix[Location::INTERIOR as usize][Location::BOUNDARY as usize])
            || self.is_true(self.matrix[Location::BOUNDARY as usize][Location::INTERIOR as usize])
            || self.is_true(self.matrix[Location::BOUNDARY as usize][Location::BOUNDARY as usize]);

        return has_point_in_common
            && self.matrix[Location::INTERIOR as usize][Location::EXTERIOR as usize]
                == Dimension::FALSE
            && self.matrix[Location::BOUNDARY as usize][Location::EXTERIOR as usize]
                == Dimension::FALSE;
    }

    /**
     *  Tests whether the argument dimensions are equal and
     *  this matrix matches the pattern <tt>[T*F**FFF*]</tt>.
     *  <p>
     *  <b>Note:</b> This pattern differs from the one stated in
     *  <i>Simple feature access - Part 1: Common architecture</i>.
     *  That document states the pattern as <tt>[TFFFTFFFT]</tt>.  This would
     *  specify that
     *  two identical <tt>POINT</tt>s are not equal, which is not desirable behaviour.
     *  The pattern used here has been corrected to compute equality in this situation.
     *
     *@param  dimensionOfGeometryA  the dimension of the first <code>Geometry</code>
     *@param  dimensionOfGeometryB  the dimension of the second <code>Geometry</code>
     *@return                       <code>true</code> if the two <code>Geometry</code>s
     *      related by this matrix are equal; the
     *      <code>Geometry</code>s must have the same dimension to be equal
     */
    pub fn is_equals(&self, dimension_of_geometry_a: i32, dimension_of_geometry_b: i32) -> bool {
        if dimension_of_geometry_a != dimension_of_geometry_b {
            return false;
        }
        return self.is_true(self.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize])
            && self.matrix[Location::INTERIOR as usize][Location::EXTERIOR as usize]
                == Dimension::FALSE
            && self.matrix[Location::BOUNDARY as usize][Location::EXTERIOR as usize]
                == Dimension::FALSE
            && self.matrix[Location::EXTERIOR as usize][Location::INTERIOR as usize]
                == Dimension::FALSE
            && self.matrix[Location::EXTERIOR as usize][Location::BOUNDARY as usize]
                == Dimension::FALSE;
    }

    pub fn equals(&self, other: &IntersectionMatrix) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.matrix[i][j] != other.matrix[i][j] {
                    return false;
                }
            }
        }
        true
    }

    /**
     * Tests if this matrix matches
     *  <UL>
     *    <LI><tt>[T*T***T**]</tt> (for two points or two surfaces)
     *    <LI><tt>[1*T***T**]</tt> (for two curves)
     *  </UL>.
     *
     *@param  dimensionOfGeometryA  the dimension of the first <code>Geometry</code>
     *@param  dimensionOfGeometryB  the dimension of the second <code>Geometry</code>
     *@return                       <code>true</code> if the two <code>Geometry</code>s
     *      related by this matrix overlap. For this
     *      function to return <code>true</code>, the <code>Geometry</code>s must
     *      be two points, two curves or two surfaces.
     */
    pub fn is_overlaps(&self, dimension_of_geometry_a: i32, dimension_of_geometry_b: i32) -> bool {
        if (dimension_of_geometry_a == Dimension::P && dimension_of_geometry_b == Dimension::P)
            || (dimension_of_geometry_a == Dimension::A && dimension_of_geometry_b == Dimension::A)
        {
            return self
                .is_true(self.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize])
                && self.is_true(
                    self.matrix[Location::INTERIOR as usize][Location::EXTERIOR as usize],
                )
                && self.is_true(
                    self.matrix[Location::EXTERIOR as usize][Location::INTERIOR as usize],
                );
        }
        if dimension_of_geometry_a == Dimension::L && dimension_of_geometry_b == Dimension::L {
            return self.matrix[Location::INTERIOR as usize][Location::INTERIOR as usize] == 1
                && self.is_true(
                    self.matrix[Location::INTERIOR as usize][Location::EXTERIOR as usize],
                )
                && self.is_true(
                    self.matrix[Location::EXTERIOR as usize][Location::INTERIOR as usize],
                );
        }
        return false;
    }

    /**
     * Tests whether this matrix matches the given matrix pattern.
     *
     *@param  pattern A pattern containing nine dimension symbols with which to
     *      compare the entries of this matrix. Possible
     *      symbol values are <code>{T, F, * , 0, 1, 2}</code>.
     *@return <code>true</code> if this matrix matches the pattern
     */
    pub fn matches_string(&self, pattern: String) -> bool {
        if pattern.len() != 9 {
            return false;
        }
        for ai in 0..3 {
            for bi in 0..3 {
                let character = pattern.chars().nth(3 * ai + bi);
                match character {
                    Some(character) => {
                        if !IntersectionMatrix::matches_i32_char(self.matrix[ai][bi], character) {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    /**
     *  Transposes this IntersectionMatrix.
     *
     *@return    this <code>IntersectionMatrix</code> as a convenience
     */
    pub fn transpose(&mut self) {
        let mut temp = self.matrix[1][0];
        self.matrix[1][0] = self.matrix[0][1];
        self.matrix[0][1] = temp;
        temp = self.matrix[2][0];
        self.matrix[2][0] = self.matrix[0][2];
        self.matrix[0][2] = temp;
        temp = self.matrix[2][1];
        self.matrix[2][1] = self.matrix[1][2];
        self.matrix[1][2] = temp;
    }

    /**
     *  Returns a nine-character <code>String</code> representation of this <code>IntersectionMatrix</code>
     *  .
     *
     *@return    the nine dimension symbols of this <code>IntersectionMatrix</code>
     *      in row-major order.
     */
    pub fn to_string(&self) -> String {
        let mut output = "".to_owned();
        for ai in 0..3 {
            for bi in 0..3 {
                let character = Dimension::to_dimension_symbol(self.matrix[ai][bi]);
                match character {
                    Some(character) => output.push(character),
                    None => {}
                }
            }
        }
        output
    }
}
