use rand::Rng;

pub struct MathUtil {}

impl MathUtil {
    /**
     * Clamps a <tt>double</tt> value to a given range.
     * @param x the value to clamp
     * @param min the minimum value of the range
     * @param max the maximum value of the range
     * @return the clamped value
     */
    pub fn clamp_f64(x: f64, min: f64, max: f64) -> f64 {
        if x < min {
            return min;
        }
        if x > max {
            return max;
        }
        return x;
    }

    /**
     * Clamps an <tt>int</tt> value to a given range.
     * @param x the value to clamp
     * @param min the minimum value of the range
     * @param max the maximum value of the range
     * @return the clamped value
     */
    pub fn clamp_i32(x: i32, min: i32, max: i32) -> i32 {
        if x < min {
            return min;
        }
        if x > max {
            return max;
        }
        return x;
    }

    /**
     * Clamps an integer to a given maximum limit.
     *
     * @param x the value to clamp
     * @param max the maximum value
     * @return the clamped value
     */
    pub fn clamp_max(x: i32, max: i32) -> i32 {
        if x > max {
            return max;
        }
        return x;
    }

    /**
     * Computes the ceiling function of the dividend of two integers.
     *
     * @param num the numerator
     * @param denom the denominator
     * @return the ceiling of num / denom
     */
    pub fn ceil(num: i32, denom: i32) -> i32 {
        let div = num / denom;
        if div * denom >= num {
            return div;
        } else {
            return div + 1;
        }
    }

    /**
     * Computes an index which wraps around a given maximum value.
     * For values &gt;= 0, this is equals to <tt>val % max</tt>.
     * For values &lt; 0, this is equal to <tt>max - (-val) % max</tt>
     *
     * @param index the value to wrap
     * @param max the maximum value (or modulus)
     * @return the wrapped index
     */
    pub fn wrap(index: i32, max: i32) -> i32 {
        if index < 0 {
            return max - ((index * -1) % max);
        }
        return index % max;
    }

    /**
     * Computes the average of two numbers.
     *
     * @param x1 a number
     * @param x2 a number
     * @return the average of the inputs
     */
    pub fn average(x1: f64, x2: f64) -> f64 {
        return (x1 + x2) / 2.0;
    }

    pub fn max_3(v1: f64, v2: f64, v3: f64) -> f64 {
        let mut max = v1;
        if v2 > max {
            max = v2;
        }
        if v3 > max {
            max = v3;
        }
        return max;
    }

    pub fn max_4(v1: f64, v2: f64, v3: f64, v4: f64) -> f64 {
        let mut max = v1;
        if v2 > max {
            max = v2;
        }
        if v3 > max {
            max = v3;
        }
        if v4 > max {
            max = v4;
        }
        return max;
    }

    pub fn min(v1: f64, v2: f64, v3: f64, v4: f64) -> f64 {
        let mut min = v1;
        if v2 < min {
            min = v2;
        }
        if v3 < min {
            min = v3;
        }
        if v4 < min {
            min = v4;
        }
        return min;
    }

    /**
     * Generates a quasi-random sequence of numbers in the range [0,1].
     * They are produced by an additive recurrence with 1/&phi; as the constant.
     * This produces a low-discrepancy sequence which is more evenly
     * distribute than random numbers.
     * <p>
     * See <a href='https://en.wikipedia.org/wiki/Low-discrepancy_sequence#Additive_recurrence'>Wikipedia: Low-discrepancy Sequences - Additive Recurrence</a>.
     * <p>
     * The sequence is initialized by calling it
     * with any positive fractional number; 0 works well for most uses.
     *
     * @param curr the current number in the sequence
     * @return the next value in the sequence
     */
    pub fn quasirandom_curr(curr: f64) -> f64 {
        let phi_inv = (f64::sqrt(5.) - 1.0) / 2.0;
        return MathUtil::quasirandom_curr_alpha(curr, phi_inv);
    }

    /**
     * Generates a quasi-random sequence of numbers in the range [0,1].
     * They are produced by an additive recurrence with constant &alpha;.
     * <pre>
     *     R(&alpha;) :  t<sub>n</sub> = { t<sub>0</sub> + n&alpha; },  n = 1,2,3,...   
     * </pre>
     * When &alpha; is irrational this produces a
     * <a href='https://en.wikipedia.org/wiki/Low-discrepancy_sequence#Additive_recurrence'>Low discrepancy sequence</a>
     *  which is more evenly
     * distributed than random numbers.
     * <p>
     * The sequence is initialized by calling it
     * with any positive fractional number. 0 works well for most uses.
     *
     * @param curr the current number in the sequence
     * @param alpha the sequence additive constant
     * @return the next value in the sequence
     */
    pub fn quasirandom_curr_alpha(curr: f64, alpha: f64) -> f64 {
        let next = curr + alpha;
        if next < 1. {
            return next;
        }
        return next - f64::floor(next);
    }

    /**
     * Generates a randomly-shuffled list of the integers from [0..n-1].
     * <p>
     * One use is to randomize points inserted into a {@link KDtree}.
     *
     * @param n the number of integers to shuffle
     * @return the shuffled array
     */
    pub fn shuffle(n: usize) -> Vec<i32> {
        let mut ints: Vec<i32> = vec![];
        for i in 0..n {
            ints[i] = i as i32;
        }

        let mut i = n - 1;
        while i >= 1 {
            let j = rand::thread_rng().gen_range(0..i + 1);
            let last = ints[i];
            ints[i] = ints[j];
            ints[j] = last;
            i = i - 1;
        }

        return ints;
    }
}
