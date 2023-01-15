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
pub fn omega(n: NonZeroU128) {
    let mut n: u128 = n.into();
    loop {
        let odd = n & 1 == 1;

        // If N is even it decreases, thus it will reach 1.
        if !odd {
            return;
        }

        // If N is odd, and has more than 2 trailing zeros, it is about to decrease, thus it will reach 1.
        let next_n = 3 * n + 1;
        if odd && (next_n.trailing_zeros() > 1) {
            return;
        }

        /*
        (SHOULD BE) Equivalent to the following:
        ```rust
            n = next_if_odd / 2;
            n = 3 * n + 1;
        ```
        */
        n = (9 * n + 5) / 2;
    }
}

/// Same as Omega, but faster than Omega when N is known to be odd, since it bypasses an if-statement.
pub fn omega_n_is_odd(n: NonZeroU128) {
    let mut n: u128 = n.into();
    loop {
        let m = 3 * n + 1;
        if m.trailing_zeros() > 1 {
            return;
        }
        n = m / 2;
    }
}
