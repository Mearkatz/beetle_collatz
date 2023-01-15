//! Contains functions that apply the rules of the collatz conjecture in more performant ways

use std::num::NonZeroU128;

/// Applies the rules of the collatz conjecture to a number N, and returns the result.
/// If N is ODD: returns 3n + 1,
/// If N is EVEN: returns n / 2.
/// All other functions in this module are faster than this one.
/// Should only be used when benchmarking other functions in this module.
pub fn basic(n: NonZeroU128) -> u128 {
    let n: u128 = n.into();
    if n & 1 == 1 {
        // N is ODD
        3 * n + 1
    } else {
        // N is EVEN
        n / 2
    }
}

// Same as the `basic` function,
// except if N is odd, it also divides it by 2 before returning it.
// for use with the `fall` function
/// Do not use if the precise number of steps needed to reach 1 is important.
pub fn halve_odds(n: NonZeroU128) -> u128 {
    let n: u128 = n.into();
    match n & 1 {
        1 => (3 * n + 1) / 2, // ODD
        _ => n / 2,           // EVEN
    }
}

/// In theory faster than halve_odds, in practice, seems about the same.    
pub fn trailing_zeros(n: NonZeroU128) -> u128 {
    let next_n = basic(n);
    next_n >> next_n.trailing_zeros()
}

/// same as rules::trailing_zeros, but we know for sure that N is ODD.
pub fn trailing_zeros_num_is_odd(n: NonZeroU128) -> u128 {
    let m: u128 = (Into::<u128>::into(n) * 3) + 1;
    let m: NonZeroU128 = m.try_into().unwrap(); // will always succeed
    crate::divide_while_even(m)
}

/// same as rules::trailing_zeros, but we know for sure that N is EVEN
pub fn trailing_zeros_num_is_even(n: NonZeroU128) -> u128 {
    crate::divide_while_even(n)
}
