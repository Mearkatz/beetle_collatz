//! Contains functions that apply the rules of the collatz conjecture in more performant ways

use no_panic::no_panic;

use crate::{Collatz, NonZero};

/// Returns 3 * n + 1.
/// If there is an overflow, this returns None.
#[no_panic]
pub fn odd_rule<T: Collatz>(n: NonZero<T>) -> Option<NonZero<T>> {
    let one = T::one();
    let three = one + one + one;
    let three_times_n = n.0.checked_mul(&three)?; // Make sure `3 * n` doesn't overflow.
    let three_times_n_plus_one = three_times_n.checked_add(&one)?; // Make sure `three_times_n + 1` doesn't overflow
    Some(NonZero(three_times_n_plus_one))
}

/// Returns n / 2.
/// This should never panic in the contexts where it's used.
#[no_panic]
pub fn even_rule<T: Collatz>(n: NonZero<T>) -> NonZero<T> {
    let two = T::one() + T::one();
    NonZero(n.0 / two)
}

/// Applies the rules of the collatz conjecture to a number N, and returns the result.
/// If N is ODD: returns 3n + 1,
/// If N is EVEN: returns n / 2.
/// All other functions in this module are faster than this one.
/// Should only be used when benchmarking other functions in this module.
#[no_panic]
pub fn basic<T: Collatz>(n: NonZero<T>) -> Option<NonZero<T>> {
    if n.0.is_odd() {
        odd_rule(n)
    } else {
        Some(even_rule(n))
    }
}

/// Same as the `basic` function,
/// except if N is odd, it also divides it by 2 before returning it.
/// for use with the `fall` function
/// Do not use if the precise number of steps needed to reach 1 is important.
#[no_panic]
pub fn halve_odds<T: Collatz>(n: NonZero<T>) -> NonZero<T> {
    let two = T::one() + T::one();

    let n = n.0;
    if n.is_odd() {
        NonZero((n + n + n + T::one()) / two)
    } else {
        NonZero(n / two)
    }
}

/// In theory faster than halve_odds  
#[no_panic]
pub fn trailing_zeros<T: Collatz>(n: NonZero<T>) -> Option<NonZero<T>> {
    let applied_rules: NonZero<T> = basic(n)?;
    let ans = crate::divide_while_even(applied_rules)?;
    Some(ans)
}

/// same as rules::trailing_zeros, but we know for sure that N is ODD.
#[no_panic]
pub fn trailing_zeros_num_is_odd<T: Collatz>(n: NonZero<T>) -> Option<NonZero<T>> {
    let n = n.0 * T::from_u8(3).unwrap() + T::one();
    let n = NonZero(n); // will always succeed
    crate::divide_while_even(n)
}

/// same as rules::trailing_zeros, but we know for sure that N is EVEN
#[no_panic]
pub fn trailing_zeros_num_is_even<T: Collatz>(n: NonZero<T>) -> Option<NonZero<T>> {
    crate::divide_while_even(n)
}
