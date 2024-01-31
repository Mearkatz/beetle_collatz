//! A collection of functions relating to the Collatz conjecture

pub mod traits;
use std::ops::{Add, Shr};

use beetle_nonzero::NonZero;
use num::{Integer, One, PrimInt};
pub use traits::*;

impl<T> TwoThree for T where T: PrimInt + Integer + One + Add<Output = T> {}

impl<T> Rules for NonZero<T>
where
    T: PrimInt + Integer + Shr<u32, Output = T>,
{
    fn odd_rule(self) -> Self {
        unsafe { self.map_unchecked(|x| x * T::three() + T::one()) }
    }

    fn even_rule(self) -> Self {
        unsafe { self.map_unchecked(|x| x / T::two()) }
    }

    fn rules(self) -> Self {
        if self.is_odd() {
            self.odd_rule()
        } else {
            self.even_rule()
        }
    }

    fn rules_halve_odds(self) -> Self {
        if self.is_odd() {
            self.odd_rule().even_rule()
        } else {
            self.even_rule()
        }
    }

    fn rules_remove_trailing_zeros(self) -> Self {
        let x = if self.is_odd() { self.odd_rule() } else { self };
        x.without_trailing_zeros()
    }
}

impl<T> Steps for NonZero<T>
where
    T: PrimInt + Integer + Shr<u32, Output = T>,
{
    fn steps_to_one(self) -> u64 {
        if self.is_even() {
            self.steps_to_one_for_even_number()
        } else {
            self.steps_to_one_for_odd_number()
        }
    }

    fn steps_to_one_for_even_number(self) -> u64 {
        let steps_to_become_odd: u64 = self.trailing_zeros().into();
        let without_zeros = self.without_trailing_zeros();
        steps_to_become_odd + without_zeros.steps_to_one_for_odd_number()
    }

    fn steps_to_one_for_odd_number(self) -> u64 {
        let mut steps: u64 = 0;
        let mut n = self;
        while !n.get().is_one() {
            // Number is known to be odd here,
            // so apply odd rule,
            n = n.odd_rule();
            steps += 1;

            // After the odd rule is applied, the resulting number is always even,
            // so remove trailing zeros,
            // and count each trailing zeros as a step
            let tz: u64 = n.trailing_zeros().into();
            n = n.without_trailing_zeros();
            steps += tz;
        }
        steps
    }

    fn steps_to_decrease(self) -> u64 {
        if self.is_even() {
            1
        } else {
            self.steps_to_decrease_for_odd_number()
        }
    }

    fn steps_to_decrease_for_odd_number(self) -> u64 {
        let mut n = self;
        let mut steps: u64 = 0;

        n = n.odd_rule();
        steps += u64::from(n.trailing_zeros()) + 1;
        n = n.without_trailing_zeros();

        let starting_value = self;
        while n > starting_value {
            n = n.odd_rule();
            steps += u64::from(n.trailing_zeros()) + 1;
            n = n.without_trailing_zeros();
        }

        steps
    }
}

impl<T> Transformations for NonZero<T>
where
    T: Integer + PrimInt + Shr<u32, Output = T>,
{
    fn transformations_to_one(&self) -> Vec<Self> {
        let mut v: Vec<Self> = Vec::new();
        let mut n: Self = *self;
        v.push(n);
        while !n.get().is_one() {
            n = n.rules();
            v.push(n);
        }
        v
    }
}

#[allow(clippy::unwrap_used, unused_imports)]
mod tests {
    use beetle_nonzero::NonZero;

    use crate::{Steps, TwoThree};

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
        let one: NonZero<u8> = unsafe { NonZero::new_unchecked(1) };
        assert_eq!(one.steps_to_one(), 0);

        // u16
        let one: NonZero<u16> = unsafe { NonZero::new_unchecked(1) };
        assert_eq!(one.steps_to_one(), 0);

        // u32
        let one: NonZero<u32> = unsafe { NonZero::new_unchecked(1) };
        assert_eq!(one.steps_to_one(), 0);

        // u64
        let one: NonZero<u64> = unsafe { NonZero::new_unchecked(1) };
        assert_eq!(one.steps_to_one(), 0);

        // u128
        let one: NonZero<u128> = unsafe { NonZero::new_unchecked(1) };
        assert_eq!(one.steps_to_one(), 0);
    }

    #[test]
    fn step_counts_for_ranges_are_correct() {
        let start: u32 = 1;
        let stop: u32 = 73;
        let steps: Vec<u64> = (start..stop)
            .map(|n| unsafe { NonZero::new_unchecked(n).steps_to_one() })
            .collect();
        assert_eq!(steps, OEIS_STEPS.to_vec());

        // BigUint
        let start: u32 = 1;
        let stop: u32 = 73;

        let steps: Vec<u64> = (start..stop)
            .map(|n| unsafe { NonZero::new_unchecked(n).steps_to_one() })
            .collect();
        assert_eq!(steps, OEIS_STEPS.to_vec());
    }

    #[test]
    fn transformations_for_numbers_are_correct() {
        use crate::Transformations;
        let n: NonZero<u32> = unsafe { NonZero::new_unchecked(4u32) };
        let transforms = n.transformations_to_one();
        let expected_transformations: Vec<NonZero<u32>> = [4, 2, 1]
            .into_iter()
            .map(|x: u32| unsafe { NonZero::new_unchecked(x) })
            .collect();
        assert_eq!(transforms, expected_transformations);
    }
}
