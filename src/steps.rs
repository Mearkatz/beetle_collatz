//! Functions for counting how many steps a number takes to reach 1

use crate::divide_while_even_and_trailing_zeros;
use std::num::NonZeroU128;

/// Counts how many steps N takes to reach 1.
/// Probably slower than other functions in this module.
pub fn alpha(n: NonZeroU128) -> u32 {
    let mut steps = 0;
    let mut n: u128 = n.into();
    while n != 1 {
        if n & 1 == 1 {
            n = 3 * n + 1;
            steps += 1;
        }
        n /= 2;
        steps += 1;
    }
    steps
}

/// Ideally far faster than steps::basic. Further testing needed.
pub fn omega(n: NonZeroU128) -> u32 {
    /*
    Big brain:
    If N is Even, simply make it odd!
    */
    if Into::<u128>::into(n) & 1 != 1 {
        omega_n_is_even(n)
    } else {
        omega_n_is_odd(n)
    }
}

/// Makes N odd, then passes it to omega_n_is_odd
pub fn omega_n_is_even(n: NonZeroU128) -> u32 {
    let (n, steps) = divide_while_even_and_trailing_zeros(n);
    steps + omega_n_is_odd(n.try_into().unwrap())
}

/// Same as steps::omega, but N is known to be odd, saving some computations
pub fn omega_n_is_odd(n: NonZeroU128) -> u32 {
    let mut steps = 0;
    let mut n: u128 = n.into();
    while n != 1 {
        // See rules_super_speed for an explanation
        let m = 3 * n + 1;
        let zeros = m.trailing_zeros();
        n = m >> zeros;
        steps += zeros + 1;
    }
    steps
}
