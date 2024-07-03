#[cfg(test)]
mod dd_compute_tests {
    use crate::core::math::dd::DD;

    #[test]
    fn test_e_by_taylor_series() {
        let test_e = compute_e_by_taylor_series();
        let err = f64::abs(test_e.subtract_dd(&DD::new_e()).double_value());
        assert!(err < 64. * DD::EPS);
    }

    /**
     * Uses Taylor series to compute e
     *
     * e = 1 + 1 + 1/2! + 1/3! + 1/4! + ...
     *
     * @return an approximation to e
     */
    fn compute_e_by_taylor_series() -> DD {
        let mut s = DD::value_of_f64(2.0);
        let mut t = DD::value_of_f64(1.0);
        let mut n = 1.0;

        while t.double_value() > DD::EPS {
            n += 1.0;
            t = t.divide_dd(&DD::value_of_f64(n));
            s = s.add_dd(&t);
        }
        return s;
    }

    #[test]
    fn test_pi_by_machin() {
        let test_e = compute_pi_by_machin();
        let err = f64::abs(test_e.subtract_dd(&DD::new_pi()).double_value());
        assert!(err < 8. * DD::EPS);
    }

    /**
     * Uses Machin's arctangent formula to compute Pi:
     *
     *    Pi / 4  =  4 arctan(1/5) - arctan(1/239)
     *    
     * @return an approximation to Pi
     */
    fn compute_pi_by_machin() -> DD {
        let t1 = DD::value_of_f64(1.0).divide_dd(&DD::value_of_f64(5.0));
        let t2 = DD::value_of_f64(1.0).divide_dd(&DD::value_of_f64(239.0));

        let pi4 = (DD::value_of_f64(4.0).multiply_dd(&arctan(&t1))).subtract_dd(&arctan(&t2));
        let pi = DD::value_of_f64(4.0).multiply_dd(&pi4);
        return pi;
    }

    /**
     * Computes the arctangent based on the Taylor series expansion
     *
     *    arctan(x) = x - x^3 / 3 + x^5 / 5 - x^7 / 7 + ...
     *    
     * @param x the argument
     * @return an approximation to the arctangent of the input
     */
    fn arctan(x: &DD) -> DD {
        let mut t = DD::new_from_dd(x);
        let t2 = t.sqr();
        let mut at = DD::new_x(0.0);
        let two = DD::new_x(2.0);
        let mut d = DD::new_x(1.0);
        let mut sign = 1;
        while t.double_value() > DD::EPS {
            if sign < 0 {
                at = at.subtract_dd(&t.divide_dd(&d));
            } else {
                at = at.add_dd(&t.divide_dd(&d));
            }

            d = d.add_dd(&two);
            t = t.multiply_dd(&t2);
            sign = -sign;
        }
        return at;
    }
}
