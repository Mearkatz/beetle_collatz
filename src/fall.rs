//! Contains functions that apply the rules of the collatz conjecture until a number reaches one

use std::num::NonZeroU128;

/// Applies the rules of the collatz conjecture until a number reaches one
/// This exists for benchmarking other faster functions' speed relative this one.
/// This aims to always be a correct implementation, but not very fast.
/// Do not use if performance is important to you.
pub fn alpha(mut n: NonZeroU128) {
    while n != NonZeroU128::new(1).unwrap() {
        n = crate::rules::basic(n).try_into().unwrap();
    }
}

/// fall::alpha but MUCH FASTER.
/// Assumes you've not checked all numbers < START
pub fn omega(start: NonZeroU128) {
    let start: u128 = start.into();
    let mut n = start;
    while n >= start {
        n >>= n.trailing_zeros();
        n = 3 * n + 1;
    }
}

/// fall::omega (but faster in the case that you've checked all numbers < START)
pub fn omega_checked_lesser_numbers(start: NonZeroU128) {
    let start: u128 = start.into();
    let mut n: u128 = start;
    while n > start {
        n >>= n.trailing_zeros();
        n = 3 * n + 1;
    }
}
