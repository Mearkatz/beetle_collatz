//! A collection of functions relating to the Collatz conjecture

#![deny(missing_docs, unused_imports)]
#![warn(clippy::unwrap_used)]

mod traits;
pub use traits::*;

mod impl_traits {
    use std::ops::AddAssign;

    use crate::traits::*;
    use beetle_nonzero::{NonZero, RangeNonZeroUnsigned};
    use num::One;

    impl<T: Collatz> Rules for NonZero<T> {
        fn odd_rule(self) -> Self {
            let one = NonZero::one();
            let three = one + one + one;
            self * three + one
        }

        fn even_rule(self) -> Self {
            self >> 1
        }

        fn rules(self) -> Self {
            if self.get().is_even() {
                self.even_rule()
            } else {
                self.odd_rule()
            }
        }

        fn rules_halve_odds(self) -> Self {
            if self.get().is_odd() {
                self.odd_rule() >> 1
            } else {
                self >> 1
            }
        }

        fn rules_remove_trailing_zeros(self) -> Option<Self> {
            let partial_ans = if self.get().is_odd() {
                self.odd_rule()
            } else {
                self
            };
            partial_ans.without_trailing_zeros()
        }
    }

    impl<T: Collatz> Steps for NonZero<T> {
        fn steps_to_one(mut self) -> Option<u64> {
            let mut steps = 0;

            while !self.is_one() {
                self = self.rules();
                steps += 1;
            }
            Some(steps)
        }

        fn steps_to_decrease(mut self) -> Option<u64> {
            let mut steps = 0;
            let starting_value = self;

            while self > starting_value {
                self = self.rules();
                steps += 1;
            }

            Some(steps)
        }

        fn steps_to_one_for_even_number(self) -> Option<u64> {
            let steps_to_become_odd: u64 = self.get().trailing_zeros().into();
            let without_zeros = self.without_trailing_zeros()?;
            Some(steps_to_become_odd + without_zeros.steps_to_one_for_odd_number()?)
        }

        fn steps_to_one_for_odd_number(mut self) -> Option<u64> {
            let mut steps = 0;
            while !self.is_one() {
                // Number is known to be odd here,
                // so apply odd rule,
                self = self.odd_rule();
                steps += 1;

                // After the odd rule is applied, the resulting number is always even,
                // so remove trailing zeros,
                // and count each trailing zeros as a step
                let tz = self.get().trailing_zeros();
                self = self.without_trailing_zeros()?;
                steps += tz as u64;
            }
            Some(steps)
        }
    }

    // Impl for Primitive Integers (and BigInts apparently ??? What the fuck ???)
    impl<T: Collatz> WithoutTrailingZeros for NonZero<T> {
        type R = Option<Self>;
        fn without_trailing_zeros(&self) -> Self::R {
            let zeros: usize = self.get().trailing_zeros().try_into().ok()?;
            Some(*self >> zeros)
        }
    }

    impl<T: Collatz + Steps + AddAssign> Bouncy for NonZero<T> {
        fn is_bouncy(&self) -> Option<bool> {
            let steps: u64 = self.get().steps_to_one()?;
            // let range: Range<T> = num::iter::range(T::zero(), value);
            let range = RangeNonZeroUnsigned::new(NonZero::one(), *self);
            for x in range {
                let x_steps: u64 = x.steps_to_one()?;
                if x_steps >= steps {
                    return Some(false);
                }
            }
            Some(true)
        }
    }

    impl<T: Collatz + Rules> Transformations for NonZero<T> {
        fn transformations_to_one(mut self) -> Option<Vec<Self>> {
            let mut v = Vec::new();
            while self.get().is_one() {
                self = self.rules();
                v.push(self);
            }
            Some(v)
        }
    }
}

#[allow(clippy::unwrap_used)]
mod tests {

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
        assert_eq!(one.steps_to_one(), Some(0));

        // u16
        let one: NonZero<u8> = NonZero::one();
        assert_eq!(one.steps_to_one(), Some(0));

        // u32
        let one: NonZero<u32> = NonZero::one();
        assert_eq!(one.steps_to_one(), Some(0));

        // u64
        let one: NonZero<u64> = NonZero::one();
        assert_eq!(one.steps_to_one(), Some(0));

        // u128
        let one: NonZero<u128> = NonZero::one();
        assert_eq!(one.steps_to_one(), Some(0));
    }

    #[test]
    fn step_counts_for_ranges_are_correct() {
        use crate::traits::Steps;
        use beetle_nonzero::RangeNonZeroUnsigned;
        let start: u32 = 1;
        let stop: u32 = 73;
        let steps: Vec<u64> = RangeNonZeroUnsigned::from_primitives(start, stop)
            .expect("Failed to produce a RangeNonZeroUnsigned from two primitives")
            .map(|n| {
                n.steps_to_one()
                    .expect("Failed to count how many steps a number took to fall")
            })
            .collect();
        assert_eq!(steps, OEIS_STEPS.to_vec());
    }
}
