//! Contains functions that apply the rules of the collatz conjecture until a number reaches one
use beetle_nonzero::NonZeroUnchecked;

use crate::Collatz;

/**
Applies the rules of the collatz conjecture until a number reaches one,
returning whether it succeeds or not.
Success is marked by a number reaching one, and no errors along the way.

This exists for benchmarking other faster functions' speed relative this one.
This aims to always be a correct implementation, but not very fast.
Do not use if performance is important to you.
*/
pub fn alpha<T: Collatz>(mut n: NonZeroUnchecked<T>) -> bool {
    while !n.value.is_one() {
        // If rules::basic returns None,
        // return false immediately,
        // because it means something has gone wrong.
        if let Some(m) = crate::rules::basic(n) {
            n = m;
        } else {
            return false;
        }
    }
    true
}

/**
fall::alpha but MUCH FASTER.
Assumes you have already checked all numbers < START
*/
pub fn omega<T: Collatz>(start: NonZeroUnchecked<T>) -> bool {
    let start = start.value;
    let mut n = start;
    while n >= start {
        let trailing_zeros: usize;
        if let Ok(tz) = usize::try_from(n.trailing_zeros()) {
            trailing_zeros = tz;
        } else {
            return false;
        }
        n = n >> trailing_zeros;
        n = n + n + n + T::one();
    }
    true
}
