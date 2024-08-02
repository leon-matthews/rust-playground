
/// Q7 fixed-point format, as seen in *Rust in Action*.
/// Can represent numbers in inclusive range [1.0, -1.0]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Q7(i8);


impl Q7 {
    pub const EPSILON: f64 = 0.0078125;
}


impl From<f32> for Q7 {
    fn from(f: f32) -> Q7 {
        Q7::from(f as f64)
    }
}

impl From<f64> for Q7 {
    fn from(f: f64) -> Self {
        debug_assert!(f >= -1.0);
        debug_assert!(f <= 1.0);
        if f >= 1.0 {
            Q7(127)
        } else if f <= -1.0{
            Q7(-128)
        } else {
            Q7((f * 128.0) as i8)
        }
    }
}

impl From<Q7> for f32 {
    fn from(q: Q7) -> f32 {
        f64::from(q) as f32
    }
}


impl From<Q7> for f64 {
    fn from(q: Q7) -> f64 {
        (q.0 as f64) * 2f64.powf(-7.0)
        //~ (q.0) as f64 * Q7::EPSILON
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    /// Round-trip conversions
    #[test]
    fn from_f64_round_trip() {
        let zero = Q7::from(0.0);
        assert_eq!(f64::from(zero), 0.0);

        let minus_one = Q7::from(-1.0);
        assert_eq!(f64::from(minus_one), -1.0);
    }

    /// Peek behind curtains
    #[test]
    fn q7_from_f64_internals() {
        assert_eq!(Q7::from(0.0), Q7(0));
        assert_eq!(Q7::from(0.5), Q7(64));
        assert_eq!(Q7::from(-0.5), Q7(-64));
        assert_eq!(Q7::from(1.0), Q7(127));
        assert_eq!(Q7::from(-1.0), Q7(-128));
    }

    #[test]
    fn f64_from_q7_internals() {
        assert_eq!(f64::from(Q7(-128)), -1.0);
        assert_eq!(f64::from(Q7(-64)), -0.5);
        assert_eq!(f64::from(Q7(0)), 0.0);
        assert_eq!(f64::from(Q7(64)), 0.5);

        // Note that the
        assert!(f64::from(Q7(127)) + Q7::EPSILON == 1.0);
    }
}
