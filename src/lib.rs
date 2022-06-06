// TODO: Implement `from` or `parse` for other instantiations.

//use std::marker::PhantomData;

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
//enum Real {
//    Negative(f64),
//    Positive(f64),
//    Zero(f64),
//}
//
//use Real::*;


trait Float {}
impl Float for f32 {}
impl Float for f64 {}

// A real of unkown sign
#[derive(Debug, PartialEq)]
struct Real<T:Float>(T);

#[derive(Debug, PartialEq)]
struct Positive<T:Float>(T);

#[derive(Debug, PartialEq)]
struct Negative<T:Float>(T);

#[derive(Debug, PartialEq)]
struct Zero<T:Float>(T);


macro_rules! op_rules {
    ( $op_trait:ident $op_fun:ident $op_sym:tt [$( $x:ident $y:ident -> $result:ident)+]) => {
        $(
        impl<T> std::ops::$op_trait<$y<T>> for $x<T>
          where
            T: Float + std::ops::$op_trait<Output = T>
        {
            type Output = $result<T>;
            fn $op_fun(self, other: $y<T>) -> Self::Output {
                $result::<T>(self.0 $op_sym other.0)
            }
        }
        )+
    };
}

op_rules!(Add add + [
  Positive Positive -> Positive
  Positive Negative -> Real
  Positive Zero -> Positive
  Positive Real -> Real
  Negative Positive -> Real
  Negative Negative -> Negative
  Negative Zero -> Negative
  Negative Real -> Real
  Zero Positive -> Positive
  Zero Negative -> Negative
  Zero Zero -> Zero
  Zero Real -> Real
  Real Positive -> Real
  Real Negative -> Real
  Real Zero -> Real
  Real Real -> Real
]);

op_rules!(Sub sub - [
  Positive Positive -> Real
  Positive Negative -> Positive
  Positive Zero -> Positive
  Positive Real -> Real
  Negative Positive -> Negative
  Negative Negative -> Real
  Negative Zero -> Negative
  Negative Real -> Real
  Zero Positive -> Negative
  Zero Negative -> Positive
  Zero Zero -> Zero
  Zero Real -> Real
  Real Positive -> Real
  Real Negative -> Real
  Real Zero -> Real
  Real Real -> Real
]);

op_rules!(Mul mul * [
  Positive Positive -> Positive
  Positive Negative -> Negative
  Positive Zero -> Zero
  Positive Real -> Real
  Negative Positive -> Negative
  Negative Negative -> Positive
  Negative Zero -> Zero
  Negative Real -> Real
  Zero Positive -> Zero
  Zero Negative -> Zero
  Zero Zero -> Zero
  Zero Real -> Zero
  Real Positive -> Real
  Real Negative -> Real
  Real Zero -> Zero
  Real Real -> Real
]);

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
            let r0 = r64!(21.0);
            let r1 = r64!(-1.0);
            let expected = Real(21.0 + -1.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_real() {
            let r0 = r64!(21.0);
            let r1 = r64!(-1.0);
            let expected = Real(21.0 + -1.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn minus_positive() {
            let r0 = r64!(1.0);
            let r1 = r64!(8.0);
            let expected = Real(1.0 - 8.0);
            assert_eq!(r0 - r1, expected);
        }

        #[test]
        fn minus_real() {
            let r0 = r64!(1.0);
            let r1 = Real(-8.0);
            let expected = Real(1.0 - -8.0);
            assert_eq!(r0 - r1, expected);
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

        #[test]
        fn plus_real() {
            let r0 = r64!(0.0);
            let r1 = Real(1.0);
            let expected = Real(0.0 + 1.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn minus_positive() {
            let r0 = r64!(0.0);
            let r1 = r64!(2.0);
            let expected = Negative(0.0 - 2.0);
            assert_eq!(r0 - r1, expected);
        }

        #[test]
        fn minus_zero() {
            let r0 = r64!(0.0);
            let r1 = r64!(0.0);
            let expected = Zero(0.0 - 0.0);
            assert_eq!(r0 - r1, expected);
        }

        #[test]
        fn minus_negative() {
            let r0 = r64!(0.0);
            let r1 = r64!(-1.0);
            let expected = Positive(0.0 - -1.0);
            assert_eq!(r0 - r1, expected);
        }

        #[test]
        fn minus_real() {
            let r0 = r64!(0.0);
            let r1 = Real(1.0);
            let expected = Real(0.0 - 1.0);
            assert_eq!(r0 - r1, expected);
        }
    }

    mod negative {
        use super::*;

        #[test]
        fn plus_positive() {
            let r0 = r64!(-1.0);
            let r1 = r64!(21.0);
            let expected = Real(-1.0 + 21.0);
            assert_eq!(r0 + r1, expected);
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

        #[test]
        fn plus_real() {
            let r0 = r64!(-2.0);
            let r1 = r64!(-1.0);
            let expected = Negative(-2.0 + -1.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn minus_zero() {
            let r0 = r64!(-2.0);
            let r1 = r64!(0.0);
            let expected = Negative(-2.0 - 0.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn minus_positive() {
            let r0 = r64!(-2.0);
            let r1 = r64!(1.0);
            let expected = Real(-2.0 + 1.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn minus_negative() {
            let r0 = r64!(-2.0);
            let r1 = r64!(-1.0);
            let expected = Negative(-2.0 + -1.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn minus_real() {
            let r0 = r64!(-2.0);
            let r1 = Real(-1.0);
            let expected = Real(-2.0 + -1.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn times_negative() {
            let r0 = r64!(-2.0);
            let r1 = r64!(-2.0);
            let expected = Positive(-2.0 * -2.0);
            assert_eq!(r0 * r1, expected);
        }
    }

    mod real {
        use super::*;

        #[test]
        fn plus_real() {
            let r0 = Real(-2.0);
            let r1 = Real(-1.0);
            let expected = Real(-2.0 + -1.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_positive() {
            let r0 = Real(-1.0);
            let r1 = r64!(21.0);
            let expected = Real(-1.0 + 21.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_zero() {
            let r0 = Real(-1.0);
            let r1 = r64!(0.0);
            let expected = Real(-1.0 + 0.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn plus_negative() {
            let r0 = Real(-2.0);
            let r1 = r64!(-1.0);
            let expected = Real(-2.0 + -1.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn minus_zero() {
            let r0 = Real(-2.0);
            let r1 = r64!(0.0);
            let expected = Real(-2.0 - 0.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn minus_positive() {
            let r0 = Real(-2.0);
            let r1 = r64!(1.0);
            let expected = Real(-2.0 + 1.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn minus_negative() {
            let r0 = Real(-2.0);
            let r1 = r64!(-1.0);
            let expected = Real(-2.0 + -1.0);
            assert_eq!(r0 + r1, expected);
        }

        #[test]
        fn minus_real() {
            let r0 = Real(-2.0);
            let r1 = Real(-1.0);
            let expected = Real(-2.0 + -1.0);
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

        #[test]
        #[ignore]
        fn trailing_zeros() {
            // won't compile
            // assert_eq!(Zero(0.0), r64!(0.00000));
        }
    }
}
