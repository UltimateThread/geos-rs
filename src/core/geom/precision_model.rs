use std::fmt;

use super::coordinate::Coordinate;

/**
 * Specifies the precision model of the {@link Coordinate}s in a {@link Geometry}.
 * In other words, specifies the grid of allowable points for a <code>Geometry</code>.
 * A precision model may be <b>floating</b> ({@link #FLOATING} or {@link #FLOATING_SINGLE}),
 * in which case normal floating-point value semantics apply.
 * <p>
 * For a {@link #FIXED} precision model the {@link #makePrecise(Coordinate)} method allows rounding a coordinate to
 * a "precise" value; that is, one whose
 *  precision is known exactly.
 *<p>
 * Coordinates are assumed to be precise in geometries.
 * That is, the coordinates are assumed to be rounded to the
 * precision model given for the geometry.
 * All internal operations
 * assume that coordinates are rounded to the precision model.
 * Constructive methods (such as boolean operations) always round computed
 * coordinates to the appropriate precision model.
 * <p>
 * Three types of precision model are supported:
 * <ul>
 * <li>FLOATING - represents full double precision floating point.
 * This is the default precision model used in JTS
 * <li>FLOATING_SINGLE - represents single precision floating point.
 * <li>FIXED - represents a model with a fixed number of decimal places.
 *  A Fixed Precision Model is specified by a <b>scale factor</b>.
 *  The scale factor specifies the size of the grid which numbers are rounded to.
 *  Input coordinates are mapped to fixed coordinates according to the following
 *  equations:
 *    <UL>
 *      <LI> jtsPt.x = round( (inputPt.x * scale ) / scale
 *      <LI> jtsPt.y = round( (inputPt.y * scale ) / scale
 *    </UL>
 * </ul>
 * For example, to specify 3 decimal places of precision, use a scale factor
 * of 1000. To specify -3 decimal places of precision (i.e. rounding to
 * the nearest 1000), use a scale factor of 0.001.
 * <p>
 * It is also supported to specify a precise <b>grid size</b>
 * by providing it as a negative scale factor.
 * This allows setting a precise grid size rather than using a fractional scale,
 * which provides more accurate and robust rounding.
 * For example, to specify rounding to the nearest 1000 use a scale factor of -1000.
 * <p>
 * Coordinates are represented internally as Java double-precision values.
 * Java uses the IEEE-394 floating point standard, which
 * provides 53 bits of precision. (Thus the maximum precisely representable
 * <i>integer</i> is 9,007,199,254,740,992 - or almost 16 decimal digits of precision).
 *
 *@version 1.7
 */
#[derive(Clone, Copy)]
pub struct PrecisionModel {
    /**
     * The type of PrecisionModel this represents.
     */
    model_type: PrecisionModelType,
    /**
     * The scale factor which determines the number of decimal places in fixed precision.
     */
    scale: f64,
    /**
     * If non-zero, the precise grid size specified.
     * In this case, the scale is also valid and is computed from the grid size.
     * If zero, the scale is used to compute the grid size where needed.
     */
    grid_size: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrecisionModelType {
    Floating,
    Fixed,
    FloatingSingle,
}

impl PrecisionModel {
    /**
     * Creates a <code>PrecisionModel</code> with a default precision
     * of FLOATING.
     */
    pub fn default() -> Self {
        Self {
            // default is floating precision
            model_type: PrecisionModelType::Floating,
            scale: f64::NAN,
            grid_size: f64::NAN,
        }
    }

    /**
     * Creates a <code>PrecisionModel</code> that specifies
     * an explicit precision model type.
     * If the model type is FIXED the scale factor will default to 1.
     *
     * @param modelType the type of the precision model
     */
    pub fn new_with_type(model_type: PrecisionModelType) -> Self {
        let mut new = Self {
            model_type,
            scale: f64::NAN,
            grid_size: f64::NAN,
        };

        if model_type == PrecisionModelType::Fixed {
            new.set_scale(1.0);
        }

        new
    }

    /**
     *  Creates a <code>PrecisionModel</code> that specifies Fixed precision.
     *  Fixed-precision coordinates are represented as precise internal coordinates,
     *  which are rounded to the grid defined by the scale factor.
     *  The provided scale may be negative, to specify an exact grid size.
     *  The scale is then computed as the reciprocal.
     *
     *@param  scale amount by which to multiply a coordinate after subtracting
     *      the offset, to obtain a precise coordinate.  Must be non-zero.
     */
    pub fn new_with_scale(scale: f64) -> Self {
        let mut new = Self {
            model_type: PrecisionModelType::Fixed,
            scale: f64::NAN,
            grid_size: f64::NAN,
        };
        new.set_scale(scale);
        new
    }

    /**
     *  Copy constructor to create a new <code>PrecisionModel</code>
     *  from an existing one.
     */
    pub fn from_precision_model(pm: PrecisionModel) -> Self {
        Self {
            model_type: pm.model_type,
            scale: pm.scale,
            grid_size: pm.grid_size,
        }
    }

    /**
     * Tests whether the precision model supports floating point
     * @return <code>true</code> if the precision model supports floating point
     */
    pub fn is_floating(&self) -> bool {
        self.model_type == PrecisionModelType::Floating || self.model_type == PrecisionModelType::FloatingSingle
    }

    /**
     * Returns the maximum number of significant digits provided by this
     * precision model.
     * Intended for use by routines which need to print out
     * decimal representations of precise values (such as {@link WKTWriter}).
     * <p>
     * This method would be more correctly called
     * <tt>getMinimumDecimalPlaces</tt>,
     * since it actually computes the number of decimal places
     * that is required to correctly display the full
     * precision of an ordinate value.
     * <p>
     * Since it is difficult to compute the required number of
     * decimal places for scale factors which are not powers of 10,
     * the algorithm uses a very rough approximation in this case.
     * This has the side effect that for scale factors which are
     * powers of 10 the value returned is 1 greater than the true value.
     *
     *
     * @return the maximum number of decimal places provided by this precision model
     */
    pub fn get_maximum_significant_digits(&self) -> i32 {
        let mut max_sig_digits = 16;
        if self.model_type == PrecisionModelType::Floating {
            max_sig_digits = 16;
        } else if self.model_type == PrecisionModelType::FloatingSingle {
            max_sig_digits = 6;
        } else if self.model_type == PrecisionModelType::Fixed {
            max_sig_digits = 1 + f64::ceil(f64::ln(self.get_scale()) / f64::ln(10.)) as i32;
        }
        return max_sig_digits;
    }

    /**
     * Returns the scale factor used to specify a fixed precision model.
     * The number of decimal places of precision is
     * equal to the base-10 logarithm of the scale factor.
     * Non-integral and negative scale factors are supported.
     * Negative scale factors indicate that the places
     * of precision is to the left of the decimal point.  
     *
     *@return the scale factor for the fixed precision model
     */
    pub fn get_scale(&self) -> f64 {
        self.scale
    }

    /**
     * Computes the grid size for a fixed precision model.
     * This is equal to the reciprocal of the scale factor.
     * If the grid size has been set explicity (via a negative scale factor)
     * it will be returned.
     *  
     * @return the grid size at a fixed precision scale.
     */
    pub fn grid_size(&self) -> f64 {
        if self.is_floating() {
            return f64::NAN;
        }

        if self.grid_size != 0. {
            return self.grid_size;
        }
        return 1.0 / self.scale;
    }

    /**
     * Gets the type of this precision model
     * @return the type of this precision model
     * @see Type
     */
    pub fn get_type(&self) -> PrecisionModelType {
        return self.model_type;
    }

    /**
     *  Sets the multiplying factor used to obtain a precise coordinate.
     * This method is private because PrecisionModel is an immutable (value) type.
     */
    pub fn set_scale(&mut self, scale: f64) {
        // A negative scale indicates the grid size is being set.
        // The scale is set as well, as the reciprocal.
        if scale < 0. {
            self.grid_size = f64::abs(scale);
            self.scale = 1.0 / self.grid_size;
        } else {
            self.scale = f64::abs(scale);
            // Leave gridSize as 0, to ensure it is computed using scale
            self.grid_size = 0.0;
        }
    }

//     /**
//    *  Sets <code>internal</code> to the precise representation of <code>external</code>.
//    *
//    * @param external the original coordinate
//    * @param internal the coordinate whose values will be changed to the
//    *                 precise representation of <code>external</code>
//    * @deprecated use makePrecise instead
//    */
//   public void toInternal (Coordinate external, Coordinate internal) {
//     if (isFloating()) {
//       internal.x = external.x;
//       internal.y = external.y;
//     }
//     else {
//       internal.x = makePrecise(external.x);
//       internal.y = makePrecise(external.y);
//     }
//     internal.setZ(external.getZ());
//   }

//   /**
//    *  Returns the precise representation of <code>external</code>.
//    *
//    *@param  external  the original coordinate
//    *@return           the coordinate whose values will be changed to the precise
//    *      representation of <code>external</code>
//    * @deprecated use makePrecise instead
//    */
//   public Coordinate toInternal(Coordinate external) {
//     Coordinate internal = new Coordinate(external);
//     makePrecise(internal);
//     return internal;
//   }

// /**
//    *  Returns the external representation of <code>internal</code>.
//    *
//    *@param  internal  the original coordinate
//    *@return           the coordinate whose values will be changed to the
//    *      external representation of <code>internal</code>
//    * @deprecated no longer needed, since internal representation is same as external representation
//    */
//   public Coordinate toExternal(Coordinate internal) {
//     Coordinate external = new Coordinate(internal);
//     return external;
//   }

//   /**
//    *  Sets <code>external</code> to the external representation of <code>internal</code>.
//    *
//    *@param  internal  the original coordinate
//    *@param  external  the coordinate whose values will be changed to the
//    *      external representation of <code>internal</code>
//    * @deprecated no longer needed, since internal representation is same as external representation
//    */
//   public void toExternal(Coordinate internal, Coordinate external) {
//       external.x = internal.x;
//       external.y = internal.y;
//   }

/**
   * Rounds a numeric value to the PrecisionModel grid.
   * Asymmetric Arithmetic Rounding is used, to provide
   * uniform rounding behaviour no matter where the number is
   * on the number line.
   * <p>
   * This method has no effect on NaN values.
   * <p>
   * <b>Note:</b> Java's <code>Math#rint</code> uses the "Banker's Rounding" algorithm,
   * which is not suitable for precision operations elsewhere in JTS.
   */
  pub fn make_precise(&mut self, val: f64) -> f64 {
  	// don't change NaN values
  	if f64::is_nan(val) { return val; }
  	
  	if self.model_type == PrecisionModelType::FloatingSingle {
  		let float_single_val = val;
  		return float_single_val;
  	}

  	if self.model_type == PrecisionModelType::Fixed {
  	  if self.grid_size > 0. {
  	    return f64::round(val / self.grid_size) * self.grid_size;
  	  }
  	  else {
  	    return f64::round(val * self.scale) / self.scale;
  	  }
  	}
  	// modelType == FLOATING - no rounding necessary
  	return val;
  }

  /**
   * Rounds a Coordinate to the PrecisionModel grid.
   */
  pub fn make_precise_coordinate(&mut self, coord: &mut Coordinate) {
    // optimization for full precision
    if self.model_type == PrecisionModelType::Floating { return; }

    coord.x = self.make_precise(coord.x);
    coord.y = self.make_precise(coord.y);
    //MD says it's OK that we're not makePrecise'ing the z [Jon Aquino]
  }

  pub fn equals(&self, other: PrecisionModel) -> bool {
    self.model_type == other.model_type && self.scale == other.scale
  }

  /**
   *  Compares this {@link PrecisionModel} object with the specified object for order.
   * A PrecisionModel is greater than another if it provides greater precision.
   * The comparison is based on the value returned by the
   * {@link #getMaximumSignificantDigits} method.
   * This comparison is not strictly accurate when comparing floating precision models
   * to fixed models; however, it is correct when both models are either floating or fixed.
   *
   *@param  o  the <code>PrecisionModel</code> with which this <code>PrecisionModel</code>
   *      is being compared
   *@return    a negative integer, zero, or a positive integer as this <code>PrecisionModel</code>
   *      is less than, equal to, or greater than the specified <code>PrecisionModel</code>
   */
  pub fn compare_to(&self, other: &PrecisionModel) -> i32 {
    let sig_digits = self.get_maximum_significant_digits();
    let other_sig_digits = other.get_maximum_significant_digits();

    if sig_digits == other_sig_digits { return 0; }
    else if sig_digits < other_sig_digits { return -1; }
    else { return 1; }
  }
}

impl fmt::Display for PrecisionModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut description = "UNKNOWN".to_owned();
        if self.model_type == PrecisionModelType::Floating {
            description = "Floating".to_owned();
        } else if self.model_type == PrecisionModelType::FloatingSingle {
            description = "Floating-Single".to_owned();
        } else if self.model_type == PrecisionModelType::Fixed {
            description = format!("Fixed (Scale={})", self.get_scale());
        }

        write!(f, "{}", description)
    }
}
