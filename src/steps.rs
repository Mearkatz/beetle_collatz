//! Functions for counting how many steps a number takes to reach 1

use crate::{divide_while_even, Collatz, NonZero};

/// Counts how many steps N takes to reach 1.
/// Probably slower than other functions in this module.
pub fn alpha<T: Collatz>(n: NonZero<T>) -> u32 {
    let mut steps = 0;
    let mut n = n.0;
    while !n.is_one() {
        if n.is_odd() {
            n = n + n + n + T::one();
            steps += 1;
        }
        n = n / (T::one() + T::one());
        steps += 1;
    }
    steps
}

/// Ideally far faster than steps::basic. Further testing needed.
pub fn omega<T: Collatz>(n: NonZero<T>) -> Option<u32> {
    /*
    Big brain:
    If N is Even, simply make it odd!
    */
    if n.0.is_odd() {
        omega_n_is_odd(n)
    } else {
        omega_n_is_even(n)
    }
}

/// Makes N odd, then passes it to omega_n_is_odd
pub fn omega_n_is_even<T: Collatz>(n: NonZero<T>) -> Option<u32> {
    let mut steps = n.0.trailing_zeros();
    let n = divide_while_even(n)?;

    steps += omega_n_is_odd(n)?;
    Some(steps)
}

/// Same as steps::omega, but N is known to be odd, saving some computations
pub fn omega_n_is_odd<T: Collatz>(n: NonZero<T>) -> Option<u32> {
    let mut steps = 0;
    let mut n = n.0;
    while !n.is_one() {
        // See rules_super_speed for an explanation
        let m = n + n + n + T::one();
        let zeros = m.trailing_zeros();
        n = m >> zeros.try_into().ok()?;
        steps += zeros + 1;
    }
    Some(steps)
}
