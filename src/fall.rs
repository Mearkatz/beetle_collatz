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
pub fn omega(start: NonZeroU128) {
    let start: u128 = start.into();
    let mut n = start;    

    if n & 1 != 1 {
        n >>= n.trailing_zeros();
    }
    omega_n_is_odd(n)
}

/// Same as Omega, but faster than Omega when N is known to be odd, since it bypasses an if-statement.
pub fn omega_n_is_odd(start: u128) {
    let mut n = start;
    while n >= start {
        n = 3 * n + 1;
        n >>= n.trailing_zeros();
    }
}
