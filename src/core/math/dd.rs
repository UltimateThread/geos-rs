/**
 * Implements extended-precision floating-point numbers
 * which maintain 106 bits (approximately 30 decimal digits) of precision.
 * <p>
 * A DoubleDouble uses a representation containing two double-precision values.
 * A number x is represented as a pair of doubles, x.hi and x.lo,
 * such that the number represented by x is x.hi + x.lo, where
 * <pre>
 *    |x.lo| &lt;= 0.5*ulp(x.hi)
 * </pre>
 * and ulp(y) means "unit in the last place of y".  
 * The basic arithmetic operations are implemented using
 * convenient properties of IEEE-754 floating-point arithmetic.
 * <p>
 * The range of values which can be represented is the same as in IEEE-754.  
 * The precision of the representable numbers
 * is twice as great as IEEE-754 double precision.
 * <p>
 * The correctness of the arithmetic algorithms relies on operations
 * being performed with standard IEEE-754 double precision and rounding.
 * This is the Java standard arithmetic model, but for performance reasons
 * Java implementations are not
 * constrained to using this standard by default.  
 * Some processors (notably the Intel Pentium architecture) perform
 * floating point operations in (non-IEEE-754-standard) extended-precision.
 * A JVM implementation may choose to use the non-standard extended-precision
 * as its default arithmetic mode.
 * To prevent this from happening, this code uses the
 * Java <tt>strictfp</tt> modifier,
 * which forces all operations to take place in the standard IEEE-754 rounding model.
 * <p>
 * The API provides both a set of value-oriented operations
 * and a set of mutating operations.
 * Value-oriented operations treat DoubleDouble values as
 * immutable; operations on them return new objects carrying the result
 * of the operation.  This provides a simple and safe semantics for
 * writing DoubleDouble expressions.  However, there is a performance
 * penalty for the object allocations required.
 * The mutable interface updates object values in-place.
 * It provides optimum memory performance, but requires
 * care to ensure that aliasing errors are not created
 * and constant values are not changed.
 * <p>
 * For example, the following code example constructs three DD instances:
 * two to hold the input values and one to hold the result of the addition.
 * <pre>
 *     DD a = new DD(2.0);
 *     DD b = new DD(3.0);
 *     DD c = a.add(b);
 * </pre>
 * In contrast, the following approach uses only one object:
 * <pre>
 *     DD a = new DD(2.0);
 *     a.selfAdd(3.0);
 * </pre>
 * <p>
 * This implementation uses algorithms originally designed variously by
 * Knuth, Kahan, Dekker, and Linnainmaa.  
 * Douglas Priest developed the first C implementation of these techniques.
 * Other more recent C++ implementation are due to Keith M. Briggs and David Bailey et al.
 *
 * <h3>References</h3>
 * <ul>
 * <li>Priest, D., <i>Algorithms for Arbitrary Precision Floating Point Arithmetic</i>,
 * in P. Kornerup and D. Matula, Eds., Proc. 10th Symposium on Computer Arithmetic,
 * IEEE Computer Society Press, Los Alamitos, Calif., 1991.
 * <li>Yozo Hida, Xiaoye S. Li and David H. Bailey,
 * <i>Quad-Double Arithmetic: Algorithms, Implementation, and Application</i>,
 * manuscript, Oct 2000; Lawrence Berkeley National Laboratory Report BNL-46996.
 * <li>David Bailey, <i>High Precision Software Directory</i>;
 * <tt>http://crd.lbl.gov/~dhbailey/mpdist/index.html</tt>
 * </ul>
 *
 *
 * @author Martin Davis
 *
 */

#[derive(Clone, Copy)]
pub struct DD {
    /**
     * The high-order component of the double-double precision value.
     */
    hi: f64,

    /**
     * The low-order component of the double-double precision value.
     */
    lo: f64,
}

impl DD {
    /**
     * The smallest representable relative difference between two {link @ DoubleDouble} values
     */
    const EPS: f64 = 1.23259516440783e-32; /* = 2^-106 */

    /**
     * The value to split a double-precision value on during multiplication
     */
    const SPLIT: f64 = 134217729.0; // 2^27+1, for IEEE double

    /*------------------------------------------------------------
     *   Output
     *------------------------------------------------------------
     */

    const MAX_PRINT_DIGITS: i32 = 32;
    //  const TEN: DD = DD::valueOfF64(10.0);
    //  const ONE: DD = DD::valueOfF64(1.0);
    const SCI_NOT_EXPONENT_CHAR: &'static str = "E";
    const SCI_NOT_ZERO: &'static str = "0.0E0";

    /**
     * Creates a new DoubleDouble with value 0.0.
     */
    pub fn default() -> Self {
        Self { hi: 0.0, lo: 0.0 }
    }

    /**
     * Creates a new DoubleDouble with value x.
     *
     * @param x the value to initialize
     */
    pub fn new_x(x: f64) -> Self {
        Self { hi: x, lo: 0.0 }
    }

    /**
     * Creates a new DoubleDouble with value (hi, lo).
     *
     * @param hi the high-order component
     * @param lo the high-order component
     */
    pub fn new_hi_lo(hi: f64, lo: f64) -> Self {
        Self { hi, lo }
    }

    /**
     * Creates a new DoubleDouble with value equal to the argument.
     *
     * @param dd the value to initialize
     */
    pub fn new_from_dd(dd: &DD) -> Self {
        Self {
            hi: dd.hi,
            lo: dd.lo,
        }
    }

    // /**
    //  * Creates a new DoubleDouble with value equal to the argument.
    //  *
    //  * @param str the value to initialize by
    //  * @throws NumberFormatException if <tt>str</tt> is not a valid representation of a number
    //  */
    // pub fn new_from_string(str: &str) -> Self {
    //   this(parse(str));
    // }

    /**
     * The value nearest to the constant Pi.
     */
    pub fn new_pi() -> Self {
        Self {
            hi: 3.141592653589793116e+00,
            lo: 1.224646799147353207e-16,
        }
    }

    /**
     * The value nearest to the constant 2 * Pi.
     */
    pub fn new_two_pi() -> Self {
        Self {
            hi: 6.283185307179586232e+00,
            lo: 2.449293598294706414e-16,
        }
    }

    /**
     * The value nearest to the constant Pi / 2.
     */
    pub fn new_pi_2() -> Self {
        Self {
            hi: 1.570796326794896558e+00,
            lo: 6.123233995736766036e-17,
        }
    }

    /**
     * The value nearest to the constant e (the natural logarithm base).
     */
    pub fn new_e() -> Self {
        Self {
            hi: 2.718281828459045091e+00,
            lo: 1.445646891729250158e-16,
        }
    }

    /**
     * A value representing the result of an operation which does not return a valid number.
     */
    pub fn new_nan() -> Self {
        Self {
            hi: f64::NAN,
            lo: f64::NAN,
        }
    }

    pub fn create_nan() -> DD {
        DD::new_hi_lo(f64::NAN, f64::NAN)
    }

    // /**
    //  * Converts the string argument to a DoubleDouble number.
    //  *
    //  * @param str a string containing a representation of a numeric value
    //  * @return the extended precision version of the value
    //  * @throws NumberFormatException if <tt>s</tt> is not a valid representation of a number
    //  */
    // pub fn valueOfString(String str) -> DD {
    //   return parse(str);
    //   }

    /**
     * Converts the <tt>double</tt> argument to a DoubleDouble number.
     *
     * @param x a numeric value
     * @return the extended precision version of the value
     */
    pub fn value_of_f64(x: f64) -> DD {
        return DD::new_x(x);
    }

    /**
     * Creates a new DoubleDouble with the value of the argument.
     *
     * @param dd the DoubleDouble value to copy
     * @return a copy of the input value
     */
    pub fn copy(dd: &DD) -> DD {
        return DD::new_from_dd(dd);
    }

    pub fn copy_self(&self) -> DD {
        let copy = DD::new_from_dd(self);
        return copy;
    }

    /*
    double getHighComponent() { return hi; }

    double getLowComponent() { return lo; }
    */

    // Testing only - should not be public
    /*
    public void RENORM()
    {
      double s = hi + lo;
      double err = lo - (s - hi);
      hi = s;
      lo = err;
    }
    */

    /**
     * Set the value for the DD object. This method supports the mutating
     * operations concept described in the class documentation (see above).
     * @param value a DD instance supplying an extended-precision value.
     * @return a self-reference to the DD instance.
     */
    pub fn set_value_dd(&mut self, dd: &DD) {
        self.hi = dd.hi;
        self.lo = dd.lo;
    }

    /**
     * Set the value for the DD object. This method supports the mutating
     * operations concept described in the class documentation (see above).
     * @param value a floating point value to be stored in the instance.
     * @return a self-reference to the DD instance.
     */
    pub fn set_value_f64(&mut self, value: f64) {
        self.hi = value;
        self.lo = 0.0;
    }

    /**
     * Returns a new DoubleDouble whose value is <tt>(this + y)</tt>.
     *
     * @param y the addend
     * @return <tt>(this + y)</tt>
     */
    pub fn add_dd(&self, y: &DD) -> DD {
        let mut copy = DD::copy(self);
        copy.self_add_dd(y);
        return copy;
    }

    /**
     * Returns a new DoubleDouble whose value is <tt>(this + y)</tt>.
     *
     * @param y the addend
     * @return <tt>(this + y)</tt>
     */
    pub fn add_f64(&self, y: f64) -> DD {
        let mut copy = DD::copy(self);
        copy.self_add_f64(y);
        return copy;
    }

    /**
     * Adds the argument to the value of <tt>this</tt>.
     * To prevent altering constants,
     * this method <b>must only</b> be used on values known to
     * be newly created.
     *
     * @param y the addend
     * @return this object, increased by y
     */
    pub fn self_add_dd(&mut self, y: &DD) {
        self.self_add_hi_lo(y.hi, y.lo);
    }

    /**
     * Adds the argument to the value of <tt>this</tt>.
     * To prevent altering constants,
     * this method <b>must only</b> be used on values known to
     * be newly created.
     *
     * @param y the addend
     * @return this object, increased by y
     */
    pub fn self_add_f64(&mut self, y: f64) {
        let hh: f64;
        let h: f64;
        let ss: f64;
        let mut s: f64;
        let e: f64;
        let f: f64;

        ss = self.hi + y;
        e = ss - self.hi;
        s = ss - e;
        s = (y - e) + (self.hi - s);
        f = s + self.lo;
        hh = ss + f;
        h = f + (ss - hh);
        self.hi = hh + h;
        self.lo = h + (hh - self.hi);
    }

    pub fn self_add_hi_lo(&mut self, yhi: f64, ylo: f64) {
        let hh: f64;
        let h: f64;
        let tt: f64;
        let mut t: f64;
        let ss: f64;
        let mut s: f64;
        let mut e: f64;
        let f: f64;

        ss = self.hi + yhi;
        tt = self.lo + ylo;
        e = ss - self.hi;
        f = tt - self.lo;
        s = ss - e;
        t = tt - f;
        s = (yhi - e) + (self.hi - s);
        t = (ylo - f) + (self.lo - t);
        e = s + tt;
        hh = ss + e;
        h = e + (ss - hh);
        e = t + h;

        let zhi: f64 = hh + e;
        let zlo: f64 = e + (hh - zhi);
        self.hi = zhi;
        self.lo = zlo;
    }

    /**
     * Computes a new DoubleDouble object whose value is <tt>(this - y)</tt>.
     *
     * @param y the subtrahend
     * @return <tt>(this - y)</tt>
     */
    pub fn subtract_dd(&self, y: &DD) -> DD {
        return self.add_dd(&y.negate());
    }

    /**
     * Computes a new DoubleDouble object whose value is <tt>(this - y)</tt>.
     *
     * @param y the subtrahend
     * @return <tt>(this - y)</tt>
     */
    pub fn subtract_f64(&self, y: f64) -> DD {
        return self.add_f64(-y);
    }

    /**
     * Subtracts the argument from the value of <tt>this</tt>.
     * To prevent altering constants,
     * this method <b>must only</b> be used on values known to
     * be newly created.
     *
     * @param y the addend
     * @return this object, decreased by y
     */
    pub fn self_subtract_dd(&mut self, y: &DD) {
        if self.is_nan() {
            return;
        }
        self.self_add_hi_lo(-y.hi, -y.lo);
    }

    /**
     * Subtracts the argument from the value of <tt>this</tt>.
     * To prevent altering constants,
     * this method <b>must only</b> be used on values known to
     * be newly created.
     *
     * @param y the addend
     * @return this object, decreased by y
     */
    pub fn self_subtract_f64(&mut self, y: f64) {
        if self.is_nan() {
            return;
        }
        return self.self_add_hi_lo(-y, 0.0);
    }

    /**
     * Returns a new DoubleDouble whose value is <tt>-this</tt>.
     *
     * @return <tt>-this</tt>
     */
    pub fn negate(&self) -> DD {
        if self.is_nan() {
            return self.copy_self();
        }
        return DD::new_hi_lo(-self.hi, -self.lo);
    }

    /**
     * Returns a new DoubleDouble whose value is <tt>(this * y)</tt>.
     *
     * @param y the multiplicand
     * @return <tt>(this * y)</tt>
     */
    pub fn multiply_dd(&self, y: &DD) -> DD {
        if y.is_nan() {
            return DD::create_nan();
        }
        let mut copy = DD::copy(self);
        copy.self_multiply_dd(y);
        return copy;
    }

    /**
     * Returns a new DoubleDouble whose value is <tt>(this * y)</tt>.
     *
     * @param y the multiplicand
     * @return <tt>(this * y)</tt>
     */
    pub fn multiply_f64(&self, y: f64) -> DD {
        if f64::is_nan(y) {
            return DD::create_nan();
        }
        let mut copy = DD::copy(self);
        copy.self_multiply_hi_lo(y, 0.0);
        return copy;
    }

    /**
     * Multiplies this object by the argument, returning <tt>this</tt>.
     * To prevent altering constants,
     * this method <b>must only</b> be used on values known to
     * be newly created.
     *
     * @param y the value to multiply by
     * @return this object, multiplied by y
     */
    pub fn self_multiply_dd(&mut self, y: &DD) {
        self.self_multiply_hi_lo(y.hi, y.lo);
    }

    /**
     * Multiplies this object by the argument, returning <tt>this</tt>.
     * To prevent altering constants,
     * this method <b>must only</b> be used on values known to
     * be newly created.
     *
     * @param y the value to multiply by
     * @return this object, multiplied by y
     */
    pub fn self_multiply_f64(&mut self, y: f64) {
        self.self_multiply_hi_lo(y, 0.0);
    }

    pub fn self_multiply_hi_lo(&mut self, yhi: f64, ylo: f64) {
        let mut hx: f64;
        let tx: f64;
        let mut hy: f64;
        let ty: f64;
        let mut cc: f64;
        let mut c: f64;

        cc = DD::SPLIT * self.hi;
        hx = cc - self.hi;
        c = DD::SPLIT * yhi;
        hx = cc - hx;
        tx = self.hi - hx;
        hy = c - yhi;
        cc = self.hi * yhi;
        hy = c - hy;
        ty = yhi - hy;
        c = ((((hx * hy - cc) + hx * ty) + tx * hy) + tx * ty) + (self.hi * ylo + self.lo * yhi);
        let zhi: f64 = cc + c;
        hx = cc - zhi;
        let zlo: f64 = c + hx;
        self.hi = zhi;
        self.lo = zlo;
    }

    /**
     * Computes a new DoubleDouble whose value is <tt>(this / y)</tt>.
     *
     * @param y the divisor
     * @return a new object with the value <tt>(this / y)</tt>
     */
    pub fn divide_dd(&self, y: &DD) -> DD {
        let mut hc: f64;
        let tc: f64;
        let mut hy: f64;
        let ty: f64;
        let cc: f64;
        let mut c: f64;
        let uu: f64;
        let mut u: f64;

        cc = self.hi / y.hi;
        c = DD::SPLIT * cc;
        hc = c - cc;
        u = DD::SPLIT * y.hi;
        hc = c - hc;
        tc = cc - hc;
        hy = u - y.hi;
        uu = cc * y.hi;
        hy = u - hy;
        ty = y.hi - hy;
        u = (((hc * hy - uu) + hc * ty) + tc * hy) + tc * ty;
        c = ((((self.hi - uu) - u) + self.lo) - cc * y.lo) / y.hi;
        u = cc + c;

        let zhi: f64 = u;
        let zlo: f64 = (cc - u) + c;
        return DD::new_hi_lo(zhi, zlo);
    }

    /**
     * Computes a new DoubleDouble whose value is <tt>(this / y)</tt>.
     *
     * @param y the divisor
     * @return a new object with the value <tt>(this / y)</tt>
     */
    pub fn divide_f64(&self, y: f64) -> DD {
        if f64::is_nan(y) {
            return DD::create_nan();
        }
        let mut copy = DD::copy(self);
        copy.self_divide_hi_lo(y, 0.0);
        return copy;
    }

    /**
     * Divides this object by the argument, returning <tt>this</tt>.
     * To prevent altering constants,
     * this method <b>must only</b> be used on values known to
     * be newly created.
     *
     * @param y the value to divide by
     * @return this object, divided by y
     */
    pub fn self_divide_dd(&mut self, y: &DD) {
        return self.self_divide_hi_lo(y.hi, y.lo);
    }

    /**
     * Divides this object by the argument, returning <tt>this</tt>.
     * To prevent altering constants,
     * this method <b>must only</b> be used on values known to
     * be newly created.
     *
     * @param y the value to divide by
     * @return this object, divided by y
     */
    pub fn self_divide_f64(&mut self, y: f64) {
        return self.self_divide_hi_lo(y, 0.0);
    }

    pub fn self_divide_hi_lo(&mut self, yhi: f64, ylo: f64) {
        let mut hc: f64;
        let tc: f64;
        let mut hy: f64;
        let ty: f64;
        let cc: f64;
        let mut c: f64;
        let uu: f64;
        let mut u: f64;

        cc = self.hi / yhi;
        c = DD::SPLIT * cc;
        hc = c - cc;
        u = DD::SPLIT * yhi;
        hc = c - hc;
        tc = cc - hc;
        hy = u - yhi;
        uu = cc * yhi;
        hy = u - hy;
        ty = yhi - hy;
        u = (((hc * hy - uu) + hc * ty) + tc * hy) + tc * ty;
        c = ((((self.hi - uu) - u) + self.lo) - cc * ylo) / yhi;
        u = cc + c;

        self.hi = u;
        self.lo = (cc - u) + c;
    }

    /**
     * Returns a DoubleDouble whose value is  <tt>1 / this</tt>.
     *
     * @return the reciprocal of this value
     */
    pub fn reciprocal(&self) -> DD {
        let mut hc: f64;
        let tc: f64;
        let mut hy: f64;
        let ty: f64;
        let cc: f64;
        let mut c: f64;
        let uu: f64;
        let mut u: f64;

        cc = 1.0 / self.hi;
        c = DD::SPLIT * cc;
        hc = c - cc;
        u = DD::SPLIT * self.hi;
        hc = c - hc;
        tc = cc - hc;
        hy = u - self.hi;
        uu = cc * self.hi;
        hy = u - hy;
        ty = self.hi - hy;
        u = (((hc * hy - uu) + hc * ty) + tc * hy) + tc * ty;
        c = (((1.0 - uu) - u) - cc * self.lo) / self.hi;

        let zhi: f64 = cc + c;
        let zlo: f64 = (cc - zhi) + c;
        return DD::new_hi_lo(zhi, zlo);
    }

    /**
     * Returns the largest (closest to positive infinity)
     * value that is not greater than the argument
     * and is equal to a mathematical integer.
     * Special cases:
     * <ul>
     * <li>If this value is NaN, returns NaN.
     * </ul>
     *
     * @return the largest (closest to positive infinity)
     * value that is not greater than the argument
     * and is equal to a mathematical integer.
     */
    pub fn floor(&self) -> DD {
        if self.is_nan() {
            return DD::create_nan();
        }
        let fhi = f64::floor(self.hi);
        let mut flo = 0.0;
        // Hi is already integral.  Floor the low word
        if fhi == self.hi {
            flo = f64::floor(self.lo);
        }
        // do we need to renormalize here?
        return DD::new_hi_lo(fhi, flo);
    }

    /**
     * Returns the smallest (closest to negative infinity) value
     * that is not less than the argument and is equal to a mathematical integer.
     * Special cases:
     * <ul>
     * <li>If this value is NaN, returns NaN.
     * </ul>
     *
     * @return the smallest (closest to negative infinity) value
     * that is not less than the argument and is equal to a mathematical integer.
     */
    pub fn ceil(&self) -> DD {
        if self.is_nan() {
            return DD::create_nan();
        }
        let fhi = f64::ceil(self.hi);
        let mut flo = 0.0;
        // Hi is already integral.  Ceil the low word
        if fhi == self.hi {
            flo = f64::ceil(self.lo);
            // do we need to renormalize here?
        }
        return DD::new_hi_lo(fhi, flo);
    }

    /**
     * Returns an integer indicating the sign of this value.
     * <ul>
     * <li>if this value is &gt; 0, returns 1
     * <li>if this value is &lt; 0, returns -1
     * <li>if this value is = 0, returns 0
     * <li>if this value is NaN, returns 0
     * </ul>
     *
     * @return an integer indicating the sign of this value
     */
    pub fn signum(&self) -> i32 {
        if self.hi > 0. {
            return 1;
        }
        if self.hi < 0. {
            return -1;
        }
        if self.lo > 0. {
            return 1;
        }
        if self.lo < 0. {
            return -1;
        }
        return 0;
    }

    /**
     * Rounds this value to the nearest integer.
     * The value is rounded to an integer by adding 1/2 and taking the floor of the result.
     * Special cases:
     * <ul>
     * <li>If this value is NaN, returns NaN.
     * </ul>
     *
     * @return this value rounded to the nearest integer
     */
    pub fn rint(&mut self) -> DD {
        if self.is_nan() {
            return self.copy_self();
        }
        // may not be 100% correct
        self.self_add_f64(0.5);
        return self.floor();
    }

    /**
     * Returns the integer which is largest in absolute value and not further
     * from zero than this value.  
     * Special cases:
     * <ul>
     * <li>If this value is NaN, returns NaN.
     * </ul>
     *  
     * @return the integer which is largest in absolute value and not further from zero than this value
     */
    pub fn trunc(&self) -> DD {
        if self.is_nan() {
            return DD::create_nan();
        }
        if self.is_positive() {
            return self.floor();
        } else {
            return self.ceil();
        }
    }

    /**
     * Returns the absolute value of this value.
     * Special cases:
     * <ul>
     * <li>If this value is NaN, it is returned.
     * </ul>
     *
     * @return the absolute value of this value
     */
    pub fn abs(&self) -> DD {
        if self.is_nan() {
            return DD::create_nan();
        }
        if self.is_negative() {
            return self.negate();
        }
        return DD::new_from_dd(self);
    }

    /**
     * Computes the square of this value.
     *
     * @return the square of this value.
     */
    pub fn sqr(&self) -> DD {
        return self.multiply_dd(self);
    }

    /**
     * Squares this object.
     * To prevent altering constants,
     * this method <b>must only</b> be used on values known to
     * be newly created.
     *
     * @return the square of this value.
     */
    pub fn self_sqr(&mut self) {
        let copy = self.copy_self();
        return self.self_multiply_dd(&copy);
    }

    /**
     * Computes the square of this value.
     *
     * @return the square of this value.
     */
    pub fn sqr_f64(&self, x: f64) -> DD {
        let mut copy = DD::value_of_f64(x);
        copy.self_multiply_f64(x);
        return copy;
    }

    /**
     * Computes the positive square root of this value.
     * If the number is NaN or negative, NaN is returned.
     *
     * @return the positive square root of this number.
     * If the argument is NaN or less than zero, the result is NaN.
     */
    pub fn sqrt_dd(&self) -> DD {
        /* Strategy:  Use Karp's trick:  if x is an approximation
          to sqrt(a), then

             sqrt(a) = a*x + [a - (a*x)^2] * x / 2   (approx)

          The approximation is accurate to twice the accuracy of x.
          Also, the multiplication (a*x) and [-]*x can be done with
          only half the precision.
        */

        if self.is_zero() {
            return DD::value_of_f64(0.0);
        }

        if self.is_negative() {
            return DD::create_nan();
        }

        let x = 1.0 / f64::sqrt(self.hi);
        let ax = self.hi * x;

        let axdd = DD::value_of_f64(ax);
        let diff_sq = self.subtract_dd(&axdd.sqr());
        let d2 = diff_sq.hi * (x * 0.5);

        return axdd.add_f64(d2);
    }

    pub fn sqrt_f64(&self, x: f64) -> DD {
        return DD::value_of_f64(x).sqrt_dd();
    }

    /**
     * Computes the value of this number raised to an integral power.
     * Follows semantics of Java Math.pow as closely as possible.
     *
     * @param exp the integer exponent
     * @return x raised to the integral power exp
     */
    pub fn pow(&self, exp: i32) -> DD {
        if exp == 0 {
            return DD::value_of_f64(1.0);
        }

        let mut r = DD::new_from_dd(self);
        let mut s = DD::value_of_f64(1.0);
        let mut n = i32::abs(exp);

        if n > 1 {
            /* Use binary exponentiation */
            while n > 0 {
                if n % 2 == 1 {
                    s.self_multiply_dd(&r);
                }
                n /= 2;
                if n > 0 {
                    r = r.sqr();
                }
            }
        } else {
            s = r;
        }

        /* Compute the reciprocal if n is negative. */
        if exp < 0 {
            return s.reciprocal();
        }
        return s;
    }

    /**
     * Computes the determinant of the 2x2 matrix with the given entries.
     *
     * @param x1 a double value
     * @param y1 a double value
     * @param x2 a double value
     * @param y2 a double value
     * @return the determinant of the values
     */
    pub fn determinant_xy_f64(x1: f64, y1: f64, x2: f64, y2: f64) -> DD {
        return DD::determinant_xy_dd(
            &DD::value_of_f64(x1),
            &DD::value_of_f64(y1),
            &DD::value_of_f64(x2),
            &DD::value_of_f64(y2),
        );
    }

    /**
     * Computes the determinant of the 2x2 matrix with the given entries.
     *
     * @param x1 a matrix entry
     * @param y1 a matrix entry
     * @param x2 a matrix entry
     * @param y2 a matrix entry
     * @return the determinant of the matrix of values
     */
    pub fn determinant_xy_dd(x1: &DD, y1: &DD, x2: &DD, y2: &DD) -> DD {
        let mut copy = x1.multiply_dd(&y2);
        copy.self_subtract_dd(&y1.multiply_dd(&x2));
        return copy;
    }

    /*------------------------------------------------------------
     *   Ordering Functions
     *------------------------------------------------------------
     */

    /**
     * Computes the minimum of this and another DD number.
     *
     * @param x a DD number
     * @return the minimum of the two numbers
     */
    pub fn min(&self, x: &DD) -> DD {
        if self.le(x) {
            return self.copy_self();
        } else {
            return x.copy_self();
        }
    }

    /**
     * Computes the maximum of this and another DD number.
     *
     * @param x a DD number
     * @return the maximum of the two numbers
     */
    pub fn max(&self, x: &DD) -> DD {
        if self.ge(x) {
            return self.copy_self();
        } else {
            return x.copy_self();
        }
    }

    /*------------------------------------------------------------
     *   Conversion Functions
     *------------------------------------------------------------
     */

    /**
     * Converts this value to the nearest double-precision number.
     *
     * @return the nearest double-precision number to this value
     */
    pub fn double_value(&self) -> f64 {
        return self.hi + self.lo;
    }

    /**
     * Converts this value to the nearest integer.
     *
     * @return the nearest integer to this value
     */
    pub fn int_value(&self) -> i32 {
        return self.hi as i32;
    }

    /*------------------------------------------------------------
     *   Predicates
     *------------------------------------------------------------
     */

    /**
     * Tests whether this value is equal to 0.
     *
     * @return true if this value is equal to 0
     */
    pub fn is_zero(&self) -> bool {
        return self.hi == 0.0 && self.lo == 0.0;
    }

    /**
     * Tests whether this value is less than 0.
     *
     * @return true if this value is less than 0
     */
    pub fn is_negative(&self) -> bool {
        return self.hi < 0.0 || (self.hi == 0.0 && self.lo < 0.0);
    }

    /**
     * Tests whether this value is greater than 0.
     *
     * @return true if this value is greater than 0
     */
    pub fn is_positive(&self) -> bool {
        return self.hi > 0.0 || (self.hi == 0.0 && self.lo > 0.0);
    }

    /**
     * Tests whether this value is NaN.
     *
     * @return true if this value is NaN
     */
    pub fn is_nan(&self) -> bool {
        return f64::is_nan(self.hi);
    }

    /**
     * Tests whether this value is equal to another <tt>DoubleDouble</tt> value.
     *
     * @param y a DoubleDouble value
     * @return true if this value = y
     */
    pub fn equals(&self, y: &DD) -> bool {
        return self.hi == y.hi && self.lo == y.lo;
    }

    /**
     * Tests whether this value is greater than another <tt>DoubleDouble</tt> value.
     * @param y a DoubleDouble value
     * @return true if this value &gt; y
     */
    pub fn gt(&self, y: &DD) -> bool {
        return (self.hi > y.hi) || (self.hi == y.hi && self.lo > y.lo);
    }
    /**
     * Tests whether this value is greater than or equals to another <tt>DoubleDouble</tt> value.
     * @param y a DoubleDouble value
     * @return true if this value &gt;= y
     */
    pub fn ge(&self, y: &DD) -> bool {
        return (self.hi > y.hi) || (self.hi == y.hi && self.lo >= y.lo);
    }
    /**
     * Tests whether this value is less than another <tt>DoubleDouble</tt> value.
     * @param y a DoubleDouble value
     * @return true if this value &lt; y
     */
    pub fn lt(&self, y: &DD) -> bool {
        return (self.hi < y.hi) || (self.hi == y.hi && self.lo < y.lo);
    }
    /**
     * Tests whether this value is less than or equal to another <tt>DoubleDouble</tt> value.
     * @param y a DoubleDouble value
     * @return true if this value &lt;= y
     */
    pub fn le(&self, y: &DD) -> bool {
        return (self.hi < y.hi) || (self.hi == y.hi && self.lo <= y.lo);
    }

    /**
     * Compares two DoubleDouble objects numerically.
     *
     * @return -1,0 or 1 depending on whether this value is less than, equal to
     * or greater than the value of <tt>o</tt>
     */
    pub fn compare_to(&self, other: &DD) -> i32 {
        if self.hi < other.hi {
            return -1;
        }
        if self.hi > other.hi {
            return 1;
        }
        if self.lo < other.lo {
            return -1;
        }
        if self.lo > other.lo {
            return 1;
        }
        return 0;
    }

    /**
     * Dumps the components of this number to a string.
     *
     * @return a string showing the components of the number
     */
    pub fn dump(&self) -> String {
        return format!("DD<{}, {}>", self.hi, self.lo);
    }

    // /**
    //  * Returns a string representation of this number, in either standard or scientific notation.
    //  * If the magnitude of the number is in the range [ 10<sup>-3</sup>, 10<sup>8</sup> ]
    //  * standard notation will be used.  Otherwise, scientific notation will be used.
    //  *
    //  * @return a string representation of this number
    //  */
    // pub fn toString(&self) -> String
    // {
    //   let mag = self.magnitude(self.hi);
    //   if mag >= -3 && mag <= 20 {
    //     return self.toStandardNotation();
    //   }
    //   return self.toSciNotation();
    // }

    // /**
    //  * Returns the string representation of this value in standard notation.
    //  *
    //  * @return the string representation in standard notation
    //  */
    // pub fn toStandardNotation(&self) -> String {
    //   let specialStr = self.getSpecialNumberString();
    //   if specialStr.is_some() {
    //     return specialStr;
    //   }

    //   let magnitude: [i32;1] = [1];
    //   let sigDigits = self.extractSignificantDigits(true, magnitude);
    //   let decimalPointPos = magnitude[0] + 1;

    //   let num = sigDigits;
    //   // add a leading 0 if the decimal point is the first char
    //   if sigDigits.charAt(0) == '.' {
    //     num = "0" + sigDigits;
    //   }
    //   else if decimalPointPos < 0 {
    //     num = "0." + self.stringOfChar('0', -decimalPointPos) + sigDigits;
    //   }
    //   else if sigDigits.indexOf('.') == -1 {
    //     // no point inserted - sig digits must be smaller than magnitude of number
    //     // add zeroes to end to make number the correct size
    //     let numZeroes = decimalPointPos - sigDigits.length();
    //     let zeroes = self.stringOfChar('0', numZeroes);
    //     num = sigDigits + zeroes + ".0";
    //   }

    //   if self.isNegative() {
    //     return "-" + num;
    //   }
    //   return num;
    // }

    // /**
    //  * Returns the string representation of this value in scientific notation.
    //  *
    //  * @return the string representation in scientific notation
    //  */
    // pub fn toSciNotation(&self) -> String {
    //   // special case zero, to allow as
    //   if self.isZero() {
    //     return DD::SCI_NOT_ZERO.to_string();
    //   }

    //   let specialStr = self.getSpecialNumberString();
    //   if specialStr.is_some() {
    //     return specialStr;
    //   }

    //   let magnitude: [i32;1] = [1];
    //   let digits = self.extractSignificantDigits(false, magnitude);
    //   let expStr = format!("{}{}", DD::SCI_NOT_EXPONENT_CHAR.to_string(), magnitude[0]);

    // //   // should never have leading zeroes
    // //   // MD - is this correct?  Or should we simply strip them if they are present?
    // //   if (digits.charAt(0) == '0') {
    // //     throw new IllegalStateException("Found leading zero: " + digits);
    // //   }

    //   // add decimal point
    //   let mut trailingDigits = "";
    //   if digits.length() > 1 {
    //     trailingDigits = digits.substring(1);
    //   }
    //   let digitsWithDecimal = digits.charAt(0) + "." + trailingDigits;

    //   if self.isNegative() {
    //     return format!("-{}{}", digitsWithDecimal, expStr);
    //   }
    //   return digitsWithDecimal + expStr;
    // }

    // /**
    //  * Extracts the significant digits in the decimal representation of the argument.
    //  * A decimal point may be optionally inserted in the string of digits
    //  * (as long as its position lies within the extracted digits
    //  * - if not, the caller must prepend or append the appropriate zeroes and decimal point).
    //  *
    //  * @param y the number to extract ( >= 0)
    //  * @param decimalPointPos the position in which to insert a decimal point
    //  * @return the string containing the significant digits and possibly a decimal point
    //  */
    // pub fn extractSignificantDigits(&self, insertDecimalPoint: bool, magnitude: [i32;1]) -> String {
    //   let y = self.abs();
    //   // compute *correct* magnitude of y
    //   let mag = magnitude(y.hi);
    //   DD scale = TEN.pow(mag);
    //   y = y.divide(scale);

    //   // fix magnitude if off by one
    //   if (y.gt(TEN)) {
    //     y = y.divide(TEN);
    //     mag += 1;
    //   }
    //   else if (y.lt(ONE)) {
    //     y = y.multiply(TEN);
    //     mag -= 1;
    //   }

    //   int decimalPointPos = mag + 1;
    //   StringBuffer buf = new StringBuffer();
    //   int numDigits = MAX_PRINT_DIGITS - 1;
    //   for (int i = 0; i <= numDigits; i++) {
    //     if (insertDecimalPoint && i == decimalPointPos) {
    //       buf.append('.');
    //     }
    //     int digit = (int) y.hi;
    // //      System.out.println("printDump: [" + i + "] digit: " + digit + "  y: " + y.dump() + "  buf: " + buf);

    //     /**
    //      * This should never happen, due to heuristic checks on remainder below
    //      */
    //     if (digit < 0 || digit > 9) {
    // //        System.out.println("digit > 10 : " + digit);
    // //        throw new IllegalStateException("Internal errror: found digit = " + digit);
    //     }
    //     /**
    //      * If a negative remainder is encountered, simply terminate the extraction.
    //      * This is robust, but maybe slightly inaccurate.
    //      * My current hypothesis is that negative remainders only occur for very small lo components,
    //      * so the inaccuracy is tolerable
    //      */
    //     if (digit < 0) {
    //       break;
    //       // throw new IllegalStateException("Internal errror: found digit = " + digit);
    //     }
    //     boolean rebiasBy10 = false;
    //     char digitChar = 0;
    //     if (digit > 9) {
    //       // set flag to re-bias after next 10-shift
    //       rebiasBy10 = true;
    //       // output digit will end up being '9'
    //       digitChar = '9';
    //     }
    //     else {
    //      digitChar = (char) ('0' + digit);
    //     }
    //     buf.append(digitChar);
    //     y = (y.subtract(DD.valueOf(digit))
    //         .multiply(TEN));
    //     if (rebiasBy10)
    //       y.selfAdd(TEN);

    //     boolean continueExtractingDigits = true;
    //     /**
    //      * Heuristic check: if the remaining portion of
    //      * y is non-positive, assume that output is complete
    //      */
    // //      if (y.hi <= 0.0)
    // //        if (y.hi < 0.0)
    // //        continueExtractingDigits = false;
    //     /**
    //      * Check if remaining digits will be 0, and if so don't output them.
    //      * Do this by comparing the magnitude of the remainder with the expected precision.
    //      */
    //     int remMag = magnitude(y.hi);
    //     if (remMag < 0 && Math.abs(remMag) >= (numDigits - i))
    //       continueExtractingDigits = false;
    //     if (! continueExtractingDigits)
    //       break;
    //   }
    //   magnitude[0] = mag;
    //   return buf.toString();
    // }

    // /**
    //  * Creates a string of a given length containing the given character
    //  *
    //  * @param ch the character to be repeated
    //  * @param len the len of the desired string
    //  * @return the string
    //  */
    // private static String stringOfChar(char ch, int len)
    // {
    //   StringBuffer buf = new StringBuffer();
    //   for (int i = 0; i < len; i++) {
    //     buf.append(ch);
    //   }
    //   return buf.toString();
    // }

    // /**
    //  * Returns the string for this value if it has a known representation.
    //  * (E.g. NaN or 0.0)
    //  *
    //  * @return the string for this special number
    //  * or null if the number is not a special number
    //  */
    // private String getSpecialNumberString()
    // {
    //   if (isZero()) return "0.0";
    //   if (isNaN())  return "NaN ";
    //   return null;
    // }

    /**
     * Determines the decimal magnitude of a number.
     * The magnitude is the exponent of the greatest power of 10 which is less than
     * or equal to the number.
     *
     * @param x the number to find the magnitude of
     * @return the decimal magnitude of x
     */
    pub fn magnitude(&self, x: f64) -> i32 {
        let x_abs = f64::abs(x);
        let x_log10 = f64::ln(x_abs) / f64::ln(10.);
        let mut x_mag: i32 = f64::floor(x_log10) as i32;

        // Since log computation is inexact, there may be an off-by-one error
        // in the computed magnitude.
        // Following tests that magnitude is correct, and adjusts it if not

        let x_approx = i32::pow(10, x_mag as u32);
        if x_approx * 10 <= x_abs as i32 {
            x_mag += 1;
        }

        return x_mag;
    }

    // /*------------------------------------------------------------
    //  *   Input
    //  *------------------------------------------------------------
    //  */
    // /**
    //  * Converts a string representation of a real number into a DoubleDouble value.
    //  * The format accepted is similar to the standard Java real number syntax.
    //  * It is defined by the following regular expression:
    //  * <pre>
    //  * [<tt>+</tt>|<tt>-</tt>] {<i>digit</i>} [ <tt>.</tt> {<i>digit</i>} ] [ ( <tt>e</tt> | <tt>E</tt> ) [<tt>+</tt>|<tt>-</tt>] {<i>digit</i>}+
    //  * </pre>
    //  *
    //  * @param str the string to parse
    //  * @return the value of the parsed number
    //  * @throws NumberFormatException if <tt>str</tt> is not a valid representation of a number
    //  */
    // public static DD parse(String str)
    //   throws NumberFormatException
    // {
    //   int i = 0;
    //   int strlen = str.length();

    //   // skip leading whitespace
    //   while (Character.isWhitespace(str.charAt(i)))
    //     i++;

    //   // check for sign
    //   boolean isNegative = false;
    //   if (i < strlen) {
    //     char signCh = str.charAt(i);
    //     if (signCh == '-' || signCh == '+') {
    //       i++;
    //       if (signCh == '-') isNegative = true;
    //     }
    //   }

    //   // scan all digits and accumulate into an integral value
    //   // Keep track of the location of the decimal point (if any) to allow scaling later
    //   DD val = new DD();

    //   int numDigits = 0;
    //   int numBeforeDec = 0;
    //   int exp = 0;
    //   boolean hasDecimalChar = false;
    //   while (true) {
    //     if (i >= strlen)
    //       break;
    //     char ch = str.charAt(i);
    //     i++;
    //     if (Character.isDigit(ch)) {
    //       double d = ch - '0';
    //       val.selfMultiply(TEN);
    //       // MD: need to optimize this
    //       val.selfAdd(d);
    //       numDigits++;
    //       continue;
    //     }
    //     if (ch == '.') {
    //       numBeforeDec = numDigits;
    //       hasDecimalChar = true;
    //       continue;
    //     }
    //     if (ch == 'e' || ch == 'E') {
    //       String expStr = str.substring(i);
    //       // this should catch any format problems with the exponent
    //       try {
    //         exp = Integer.parseInt(expStr);
    //       }
    //       catch (NumberFormatException ex) {
    //         throw new NumberFormatException("Invalid exponent " + expStr + " in string " + str);
    //       }
    //       break;
    //     }
    //     throw new NumberFormatException("Unexpected character '" + ch
    //         + "' at position " + i
    //         + " in string " + str);
    //   }
    //   DD val2 = val;

    //   // correct number of digits before decimal sign if we don't have a decimal sign in the string
    //   if (!hasDecimalChar) numBeforeDec = numDigits;

    //   // scale the number correctly
    //   int numDecPlaces = numDigits - numBeforeDec - exp;
    //   if (numDecPlaces == 0) {
    //     val2 = val;
    //   }
    //   else if (numDecPlaces > 0) {
    //     DD scale = TEN.pow(numDecPlaces);
    //     val2 = val.divide(scale);
    //   }
    //   else if (numDecPlaces < 0) {
    //     DD scale = TEN.pow(-numDecPlaces);
    //     val2 = val.multiply(scale);
    //   }
    //   // apply leading sign, if any
    //   if (isNegative) {
    //     return val2.negate();
    //   }
    //   return val2;

    // }
}
