//! Contains functions that apply the rules of the collatz conjecture until a number reaches one
use crate::{Collatz, NonZero};

/**
Applies the rules of the collatz conjecture until a number reaches one
This exists for benchmarking other faster functions' speed relative this one.
This aims to always be a correct implementation, but not very fast.
Do not use if performance is important to you.
*/
pub fn alpha<T: Collatz>(mut n: NonZero<T>) {
    while !n.0.is_one() {
        n = crate::rules::basic(n).unwrap();
    }
}

/**
fall::alpha but MUCH FASTER.
Assumes you've not checked all numbers < START
*/
pub fn omega<T: Collatz>(start: NonZero<T>) {
    let start = start.0;
    let mut n = start;
    while n >= start {
        n = n >> n.trailing_zeros().try_into().unwrap();
        n = n + n + n + T::one();
    }
}
