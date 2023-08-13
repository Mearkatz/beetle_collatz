//! A collection of functions relating to the Collatz conjecture

#![deny(missing_docs, unused, clippy::unwrap_used)]

mod traits;
use std::ops::Add;

use beetle_nonzero::{
    ranges::RangeNonZero,
    traits::{TrailingZeros, Uint, WithoutTrailingZeros},
    NonZero,
};
use num::One;
pub use traits::*;

#[inline(always)]
pub(crate) fn one<T>() -> T
where
    T: num::One,
{
    T::one()
}

#[inline(always)]
pub(crate) fn two<T>() -> T
where
    T: num::One + Add<Output = T>,
{
    T::one() + T::one()
}

#[inline(always)]
pub(crate) fn three<T>() -> T
where
    T: num::One + Add<Output = T>,
{
    T::one() + T::one() + T::one()
}
impl<T: Uint> Rules for NonZero<T>
where
    NonZero<T>: WithoutTrailingZeros,
{
    #[inline(always)]
    fn odd_rule(&self) -> Self {
        self.clone() * three() + one()
    }

    #[inline(always)]
    fn even_rule(&self) -> Self {
        self.clone() / two()
    }

    #[inline]
    fn rules(&self) -> Self {
        if self.is_even() {
            self.even_rule()
        } else {
            self.odd_rule()
        }
    }

    #[inline]
    fn rules_halve_odds(&self) -> Self {
        if self.is_odd() {
            self.odd_rule().even_rule()
        } else {
            self.even_rule()
        }
    }

    #[inline]
    fn rules_remove_trailing_zeros(&self) -> Self {
        if self.is_odd() {
            self.odd_rule().without_trailing_zeros()
        } else {
            self.without_trailing_zeros()
        }
    }
}

impl<T> Steps for NonZero<T>
where
    T: Uint,
    NonZero<T>: WithoutTrailingZeros,
{
    fn steps_to_one(&self) -> u64 {
        let mut n: NonZero<T> = self.clone();
        let mut steps = 0;

        while !n.is_one() {
            n = n.rules();
            steps += 1;
        }
        steps
    }

    fn steps_to_decrease(&self) -> u64 {
        if self.is_even() {
            1
        } else {
            self.steps_to_decrease_for_odd_number()
        }
    }

    fn steps_to_decrease_for_odd_number(&self) -> u64 {
        let mut n = self.clone();
        let mut steps = 0;
        // Apply rules once beforehand for a potential performance gain.
        n = n.odd_rule();
        steps += n.trailing_zeros() + 1;
        n = n.without_trailing_zeros();

        let starting_value = self.clone();
        while n > starting_value {
            n = n.odd_rule();
            steps += n.trailing_zeros() + 1;
            n = n.without_trailing_zeros();
        }

        steps
    }

    fn steps_to_one_for_even_number(&self) -> u64 {
        let steps_to_become_odd: u64 = self.trailing_zeros();
        let without_zeros = self.without_trailing_zeros();
        steps_to_become_odd + without_zeros.steps_to_one_for_odd_number()
    }

    fn steps_to_one_for_odd_number(&self) -> u64 {
        let mut steps = 0;
        let mut n = self.clone();
        while !n.is_one() {
            // Number is known to be odd here,
            // so apply odd rule,
            n = n.odd_rule();
            steps += 1;

            // After the odd rule is applied, the resulting number is always even,
            // so remove trailing zeros,
            // and count each trailing zeros as a step
            let tz = n.trailing_zeros();
            n = n.without_trailing_zeros();
            steps += tz;
        }
        steps
    }
}

impl<T> Bouncy for NonZero<T>
where
    T: Uint,
    NonZero<T>: TrailingZeros + Steps,
{
    fn is_bouncy(&self) -> bool {
        let steps: u64 = self.steps_to_one();
        // let range: Range<T> = num::iter::range(T::zero(), value);
        let range = RangeNonZero::new(NonZero::one(), self.clone());
        for x in range {
            let x_steps: u64 = x.steps_to_one();
            if x_steps >= steps {
                return false;
            }
        }
        true
    }
}

impl<T> Transformations for NonZero<T>
where
    NonZero<T>: WithoutTrailingZeros,
    T: Uint,
{
    fn transformations_to_one(&self) -> Vec<Self> {
        let mut v: Vec<NonZero<T>> = Vec::new();
        let mut n: NonZero<T> = self.clone();
        v.push(n.clone());
        while !n.is_one() {
            n = n.rules();
            v.push(n.clone());
        }
        v
    }
}

#[allow(clippy::unwrap_used, unused_imports)]
mod tests {
    use beetle_nonzero::{ranges::RangeNonZero, NonZero};
    use num::BigUint;

    use crate::{one, Steps};

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
        let one: NonZero<BigUint> = NonZero::one();
        assert_eq!(one.steps_to_one(), 0);
    }

    #[test]
    fn step_counts_for_ranges_are_correct() {
        use beetle_nonzero::ranges::RangeNonZero;

        // U32
        let start: u32 = 1;
        let stop: u32 = 73;
        let steps: Vec<u64> = RangeNonZero::from_primitives(start, stop)
            .unwrap()
            .map(|n: NonZero<u32>| n.steps_to_one())
            .collect();
        assert_eq!(steps, OEIS_STEPS.to_vec());

        // BigUint
        let start: u32 = 1;
        let stop: u32 = 73;
        let big_range: RangeNonZero<BigUint> =
            RangeNonZero::from_primitives(start.into(), stop.into()).unwrap();
        let steps: Vec<u64> = big_range.map(|n| n.steps_to_one()).collect();
        assert_eq!(steps, OEIS_STEPS.to_vec());
    }

    #[test]
    fn transformations_for_numbers_are_correct() {
        use crate::Transformations;
        let n: NonZero<u32> = NonZero::new(4u32).unwrap();
        let transforms = n.transformations_to_one();
        let expected_transformations: Vec<NonZero<u32>> = [4, 2, 1]
            .into_iter()
            .map(|x: u32| NonZero::new(x).unwrap())
            .collect();
        assert_eq!(transforms, expected_transformations);
    }
}
