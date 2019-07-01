// TODO: use a macro like `Real!(n)` for literals of `Positive`, `Negative`, and `Zero` in code.
// TODO: Implement `from` or `parse` for other instantiations.

//use std::marker::PhantomData;
use std::ops::Add;
use std::ops::Sub;

//enum OptionReal {
//    NaN,
//    Real
//}
//
//enum Real {
//    Negative(f64),
//    Positive(f64),
//    Zero(f64),
//}
//
//use Real::*;

#[derive(Debug, PartialEq)]
struct Positive(f64);

#[derive(Debug, PartialEq)]
struct Negative(f64);

#[derive(Debug, PartialEq)]
struct Zero(f64);

impl Add for Positive {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Positive(self.0 + other.0)
    }
}

impl Add<Zero> for Positive {
    type Output = Self;

    fn add(self, _other: Zero) -> Self::Output {
        self
    }
}

impl Sub<Zero> for Positive {
    type Output = Self;

    fn sub(self, _other: Zero) -> Self::Output {
        self
    }
}

impl Sub<Negative> for Positive {
    type Output = Self;

    fn sub(self, other: Negative) -> Self::Output {
        Positive(self.0 - other.0)
    }
}

impl Add<Positive> for Zero {
    type Output = Positive;

    fn add(self, other: Positive) -> Self::Output {
       other
    }
}

impl Add<Zero> for Zero {
    type Output = Zero;

    fn add(self, other: Zero) -> Self::Output {
       other
    }
}

impl Add<Negative> for Zero {
    type Output = Negative;

    fn add(self, other: Negative) -> Self::Output {
       other
    }
}

impl Sub<Positive> for Zero {
    type Output = Negative;

    fn sub(self, other: Positive) -> Self::Output {
       Negative(self.0 - other.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod positive {
        use super::*;

        #[test]
        fn plus_positive() {
            let r0 = Positive(2.0);
            let r1 = Positive(3.5);
            let expected = Positive(2.0 + 3.5);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_zero() {
            let r0 = Positive(2.0);
            let r1 = Zero(0.0);
            let expected = Positive(2.0 + 0.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_negative() {
            unimplemented!();
        }

        #[test]
        fn minus_positive() {
            unimplemented!();
        }

        #[test]
        fn minus_zero() {
            let r0 = Positive(2.0);
            let r1 = Zero(0.0);
            let expected = Positive(2.0 - 0.0);
            assert_eq!(r0 - r1, expected);
        }

        #[test]
        fn minus_negative() {
            let r0 = Positive(2.0);
            let r1 = Negative(-3.5);
            let expected = Positive(2.0 - -3.5);
            assert_eq!(r0 - r1, expected);
        }
    }

    mod zero {
        use super::*;

        #[test]
        fn plus_positive() {
            let r0 = Zero(0.0);
            let r1 = Positive(2.0);
            let expected = Positive(0.0 + 2.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_zero() {
            let r0 = Zero(0.0);
            let r1 = Zero(0.0);
            let expected = Zero(0.0 + 0.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_negative() {
            let r0 = Zero(0.0);
            let r1 = Negative(-1.0);
            let expected = Negative(0.0 + -1.0);
            assert_eq!(r0 + r1, expected);
        }

//        #[test]
//        fn minus_positive() {
//            let r0 = Zero(0.0);
//            let r1 = Positive(2.0);
//            let expected = Negative(0.0 - 2.0);
//            assert_eq!(r0 - r1, expected);
//        }
    }
}