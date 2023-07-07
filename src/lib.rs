//! A collection of functions relating to the Collatz conjecture

#![deny(missing_docs, unused_imports)]
#![warn(clippy::unwrap_used)]

mod traits;
pub use traits::*;

mod impl_nonzero {
    use crate::traits::*;
    use beetle_nonzero::{ranges::RangeNonZeroUnsigned, traits::*, NonZero};
    use num::{traits::Pow, One};

    impl<T: PrimUint> Rules for NonZero<T> {
        fn odd_rule(&self) -> Self {
            let value = *self;
            (value + value + value) + Self::one()
        }

        fn even_rule(&self) -> Self {
            let two = Self::one() + Self::one();
            *self / two
        }

        fn rules(&self) -> Self {
            if self.is_even() {
                self.even_rule()
            } else {
                self.odd_rule()
            }
        }

        fn rules_halve_odds(&self) -> Self {
            if self.is_odd() {
                self.odd_rule().even_rule()
            } else {
                self.even_rule()
            }
        }

        fn rules_remove_trailing_zeros(&self) -> Self {
            if self.is_odd() {
                self.odd_rule().without_trailing_zeros()
            } else {
                self.without_trailing_zeros()
            }
        }
    }

    impl<T: PrimUint> Steps for NonZero<T> {
        fn steps_to_one(&self) -> u64 {
            let mut n: NonZero<T> = *self;
            let mut steps = 0;

            while !n.is_one() {
                n = n.rules();
                steps += 1;
            }
            steps
        }

        fn steps_to_decrease(&self) -> u64 {
            if self.is_even() {
                return 1;
            }

            let mut n = *self;
            let mut steps = 0;

            // Apply rules once beforehand for a potential performance gain.
            n = n.rules();
            steps += 1;

            let starting_value = *self;
            while n > starting_value {
                n = n.rules();
                steps += 1;
            }

            steps
        }

        fn steps_to_one_for_even_number(&self) -> u64 {
            let steps_to_become_odd: u64 = self.get().trailing_zeros().into();
            let without_zeros = self.without_trailing_zeros();
            steps_to_become_odd + without_zeros.steps_to_one_for_odd_number()
        }

        fn steps_to_one_for_odd_number(&self) -> u64 {
            let mut steps = 0;
            let mut n = *self;
            while !n.is_one() {
                // Number is known to be odd here,
                // so apply odd rule,
                n = n.odd_rule();
                steps += 1;

                // After the odd rule is applied, the resulting number is always even,
                // so remove trailing zeros,
                // and count each trailing zeros as a step
                let tz = n.trailing_zeros() as u64;
                n = n.without_trailing_zeros();
                steps += tz;
            }
            steps
        }
    }

    impl<T: PrimUint> WithoutTrailingZeros for NonZero<T> {
        type R = Self;
        fn without_trailing_zeros(&self) -> Self::R {
            let zeros: u32 = self.trailing_zeros();
            let two = Self::one() + Self::one();
            let power_of_two = two.pow(zeros);
            *self / power_of_two
        }
    }

    impl<T: PrimUint> Bouncy for NonZero<T> {
        fn is_bouncy(&self) -> bool {
            let steps: u64 = self.steps_to_one();
            // let range: Range<T> = num::iter::range(T::zero(), value);
            let range = RangeNonZeroUnsigned::new(NonZero::one(), *self);
            for x in range {
                let x_steps: u64 = x.steps_to_one();
                if x_steps >= steps {
                    return false;
                }
            }
            true
        }
    }

    impl<T: PrimUint> Transformations for NonZero<T> {
        fn transformations_to_one(&self) -> Vec<Self> {
            let mut v: Vec<NonZero<T>> = Vec::new();
            let mut n: NonZero<T> = *self;
            v.push(n);
            while !n.is_one() {
                n = n.rules();
                v.push(n);
            }
            v
        }
    }
}

mod impl_nonzero_biguint {
    use crate::traits::*;
    use beetle_nonzero::{ranges::RangeNonZeroBigUint, NonZeroBigUint};
    use num::{traits::Pow, One};

    impl Rules for NonZeroBigUint {
        fn odd_rule(&self) -> Self {
            let value = self.clone();
            let one = Self::one();
            let three = one.clone() + one.clone() + one.clone();
            value * three + one
        }

        fn even_rule(&self) -> Self {
            let two = Self::one() + Self::one();
            self.clone() / two
        }

        fn rules(&self) -> Self {
            if self.is_even() {
                self.even_rule()
            } else {
                self.odd_rule()
            }
        }

        fn rules_halve_odds(&self) -> Self {
            if self.is_odd() {
                self.odd_rule().even_rule()
            } else {
                self.even_rule()
            }
        }

        fn rules_remove_trailing_zeros(&self) -> Self {
            if self.is_odd() {
                self.odd_rule().without_trailing_zeros()
            } else {
                self.without_trailing_zeros()
            }
        }
    }

    impl Steps for NonZeroBigUint {
        fn steps_to_one(&self) -> u64 {
            let mut n = self.clone();
            let mut steps = 0;

            while !n.is_one() {
                n = n.rules();
                steps += 1;
            }
            steps
        }

        fn steps_to_decrease(&self) -> u64 {
            let mut n = self.clone();
            let mut steps = 0;
            let starting_value = self.clone();

            while n >= starting_value {
                n = n.rules();
                steps += 1;
            }

            steps
        }

        fn steps_to_one_for_even_number(&self) -> u64 {
            let steps_to_become_odd: u64 = self.trailing_zeros();
            let without_zeros = self.without_trailing_zeros();
            steps_to_become_odd + without_zeros.steps_to_one_for_odd_number()
        }

        fn steps_to_one_for_odd_number(&self) -> u64 {
            let mut n = self.clone();
            let mut steps = 0;
            while !self.is_one() {
                // Number is known to be odd here,
                // so apply odd rule,
                n = n.odd_rule();
                steps += 1;

                // After the odd rule is applied, the resulting number is always even,
                // so remove trailing zeros,
                // and count each trailing zeros as a step
                n = n.without_trailing_zeros();
                steps += n.trailing_zeros();
            }
            steps
        }
    }

    impl WithoutTrailingZeros for NonZeroBigUint {
        type R = Self;
        fn without_trailing_zeros(&self) -> Self::R {
            let mut n = self.clone();

            while !n.is_odd() {
                let zeros: u32 = self.trailing_zeros() as u32;
                let two = Self::one() + Self::one();
                let power_of_two = two.pow(zeros);
                n = self.clone() / power_of_two;
            }
            n
        }
    }

    impl Bouncy for NonZeroBigUint {
        fn is_bouncy(&self) -> bool {
            let steps: u64 = self.steps_to_one();
            // let range: Range<T> = num::iter::range(T::zero(), value);
            // let range = RangeNonZeroUnsigned::new(Self::one(), self.clone());
            let range = RangeNonZeroBigUint::new(Self::one(), self.clone());
            for x in range {
                let x_steps: u64 = x.steps_to_one();
                if x_steps >= steps {
                    return false;
                }
            }
            true
        }
    }

    impl Transformations for NonZeroBigUint {
        fn transformations_to_one(&self) -> Vec<Self> {
            let mut n = self.clone();
            let mut v = vec![n.clone()];
            while !n.is_one() {
                n = n.rules();
                v.push(n.clone());
            }
            v
        }
    }
}

#[allow(clippy::unwrap_used, unused_imports)]
mod tests {
    use beetle_nonzero::{
        ranges::{RangeNonZeroBigUint, RangeNonZeroUnsigned},
        traits::ToNonZero,
        NonZero, NonZeroBigUint,
    };

    use crate::Steps;

    // Number of steps to reach 1 for integers 1..=72 (according to the Online Encyclopedia of Integer Sequences)
    // For some reason this is counted as dead code currently, but it isn't dead code, I just convert it to a vec, or iter when using it.
    #[allow(dead_code)]
    const OEIS_STEPS: [u64; 72] = [
        0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, 20, 7, 7, 15, 15, 10, 23, 10,
        111, 18, 18, 18, 106, 5, 26, 13, 13, 21, 21, 21, 34, 8, 109, 8, 29, 16, 16, 16, 104, 11,
        24, 24, 24, 11, 11, 112, 112, 19, 32, 19, 32, 19, 19, 107, 107, 6, 27, 27, 27, 14, 14, 14,
        102, 22,
    ];

    #[test]
    fn step_counts_for_integers_are_correct() {
        use crate::traits::Steps;
        use beetle_nonzero::NonZero;
        use num::One;

        // u8
        let one: NonZero<u8> = NonZero::one();
        assert_eq!(one.steps_to_one(), 0);

        // u16
        let one: NonZero<u16> = NonZero::one();
        assert_eq!(one.steps_to_one(), 0);

        // u32
        let one: NonZero<u32> = NonZero::one();
        assert_eq!(one.steps_to_one(), 0);

        // u64
        let one: NonZero<u64> = NonZero::one();
        assert_eq!(one.steps_to_one(), 0);

        // u128
        let one: NonZero<u128> = NonZero::one();
        assert_eq!(one.steps_to_one(), 0);

        // BigUint
        let one: NonZeroBigUint = NonZeroBigUint::one();
        assert_eq!(one.steps_to_one(), 0);
    }

    #[test]
    fn step_counts_for_ranges_are_correct() {
        use beetle_nonzero::{ranges::RangeNonZeroUnsigned, NonZero};

        // U32
        let start: u32 = 1;
        let stop: u32 = 73;
        let steps: Vec<u64> = RangeNonZeroUnsigned::from_primitives(start, stop)
            .unwrap()
            .map(|n: NonZero<u32>| n.steps_to_one())
            .collect();
        assert_eq!(steps, OEIS_STEPS.to_vec());

        // BigUint
        let start: u32 = 1;
        let stop: u32 = 73;
        let steps: Vec<u64> = RangeNonZeroBigUint::from_biguints(start.into(), stop.into())
            .unwrap()
            .map(|n| n.steps_to_one())
            .collect();
        assert_eq!(steps, OEIS_STEPS.to_vec());
    }

    #[test]
    fn transformations_for_numbers_are_correct() {
        use crate::Transformations;
        let n: NonZero<u32> = 4u32.to_nonzero().unwrap();
        let transforms = n.transformations_to_one();
        let expected_transformations: Vec<NonZero<u32>> = [4, 2, 1]
            .into_iter()
            .map(|x: u32| x.to_nonzero().unwrap())
            .collect();
        assert_eq!(transforms, expected_transformations);
    }
}
