// TODO: use a macro like `Real!(n)` for literals of `Positive`, `Negative`, and `Zero` in code.
// TODO: Implement `from` or `parse` for other instantiations.

//use std::marker::PhantomData;
use std::ops::Add;
use std::ops::Sub;


#[macro_export]
macro_rules! r64 {
    (0) => {Zero(0.0)};
    (-0) => {Zero(-0.0)};
    (0.0) => {Zero(0.0)};
    (-0.0) => {Zero(-0.0)};
    (-$r:literal) => {Negative(-$r)};
    ($r:literal) => {Positive($r)};
    //    (-0.$(0)+) => {Zero(-0.0)};
}

//enum OptionReal {
//    NaN,
//    Real
//}
//
#[derive(Debug, PartialEq)]
struct Positive(f64);

#[derive(Debug, PartialEq)]
struct Negative(f64);

#[derive(Debug, PartialEq)]
struct Zero(f64);

#[derive(Debug, PartialEq)]
struct GenericReal(f64);

#[derive(Debug, PartialEq)]
enum Real {
    Negative(f64),
    Positive(f64),
    Zero(f64),
    GenericReal(f64),
}
use Real::*;


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

impl Add<Real> for Real {
    type Output = Self;

    fn add(self, other: Real) -> Real {
       Real(self.0 + other.0)
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

impl Add for Negative {
    type Output = Negative;

    fn add(self, other: Self) -> Self::Output {
        Negative(self.0 + other.0)
    }
}

impl Add<Zero> for Negative {
    type Output = Negative;

    fn add(self, _other: Zero) -> Self::Output {
        self
    }
}

impl Add<Positive> for Negative {
    type Output = Negative;

    fn add(self, _other: Positive) -> Self::Output {
        unimplemented!();
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    mod positive {
        use super::*;

        #[test]
        fn plus_positive() {
            let r0 = r64!(2.0);
            let r1 = r64!(3.5);
            let expected = Positive(2.0 + 3.5);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_zero() {
            let r0 = r64!(2.0);
            let r1 = r64!(0.0);
            let expected = Positive(2.0 + 0.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_negative() {
           let r0 = r64!(1.0);
           let r1 = r64!(-2.0);
           let expected = Real(1.0 + -2.0);
           assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn minus_positive() {
            unimplemented!();
        }

        #[test]
        fn minus_zero() {
            let r0 = r64!(2.0);
            let r1 = r64!(0.0);
            let expected = Positive(2.0 - 0.0);
            assert_eq!(r0 - r1, expected);
        }

        #[test]
        fn minus_negative() {
            let r0 = r64!(2.0);
            let r1 = r64!(-3.5);
            let expected = Positive(2.0 - -3.5);
            assert_eq!(r0 - r1, expected);
        }

    }

    mod zero {
        use super::*;

        #[test]
        fn plus_positive() {
            let r0 = r64!(0.0);
            let r1 = r64!(2.0);
            let expected = Positive(0.0 + 2.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_zero() {
            let r0 = r64!(0.0);
            let r1 = r64!(0.0);
            let expected = Zero(0.0 + 0.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_negative() {
            let r0 = r64!(0.0);
            let r1 = r64!(-1.0);
            let expected = Negative(0.0 + -1.0);
            assert_eq!(r0 + r1, expected);
        }
    }

    mod negative {
        use super::*;

        #[test]
        fn plus_positive() {
            unimplemented!();
           // let r0 = r64!(-1.0);
           // let r1 = r64!(2.0);
           // let expected = Real(-1.0 + 2.0);
           // assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_zero() {
            let r0 = r64!(-1.0);
            let r1 = r64!(0.0);
            let expected = Negative(-1.0 + 0.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_negative() {
            let r0 = r64!(-2.0);
            let r1 = r64!(-1.0);
            let expected = Negative(-2.0 + -1.0);
            assert_eq!(r0 + r1, expected);
        }

    }

    mod macro_tests {
        use super::*;

        #[test]
        fn one_zero() {
            assert_eq!(Zero(0.0), r64!(0));
        }

        #[test]
        fn one_zero_negative() {
            assert_eq!(Zero(-0.0), r64!(-0));
        }

        #[test]
        fn negative_zero() {
            assert_eq!(Zero(-0.0), r64!(-0.0));
        }

        #[test]
        fn one_trailing_zero() {
            assert_eq!(Zero(0.0), r64!(0.0));
        }

        #[test]
        fn positive_literal() {
            assert_eq!(Positive(1.5), r64!(1.5));
        }

        #[test]
        fn negative_literal() {
            assert_eq!(Negative(-1.5), r64!(-1.5));
        }

        #[test]
        fn nontrivial_literal() {
            assert_eq!(Positive(123_456_3e1_2), r64!(123_456_3e1_2));
        }

        //#[test]
        //fn trailing_zeros() {
        //    assert_eq!(Zero(0.0), r64!(0.00000));
        //}
    }
}
