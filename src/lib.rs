//! A collection of functions relating to the Collatz conjecture

#![deny(missing_docs, unused_imports)]
#![warn(clippy::unwrap_used)]

use std::fmt::{Debug, Display};

use beetle_nonzero::NonZeroUnchecked;
use no_panic::no_panic;
use num::{FromPrimitive, Integer, PrimInt, Unsigned};

/// Types implementing this can be passed to the most if not all functions in this library
pub trait Collatz: Unsigned + Integer + PrimInt + FromPrimitive + Debug + Display {}

impl<T> Collatz for T where T: Unsigned + Integer + PrimInt + FromPrimitive + Debug + Display {}

/// This has the effect of dividing a number by 2 until it is odd.
/// Odd numbers are simply returned.
pub fn divide_while_even<T: Collatz>(n: NonZeroUnchecked<T>) -> Option<NonZeroUnchecked<T>> {
    let n = n.value;
    let trailing_zeros: usize = n.trailing_zeros().try_into().ok()?;
    Some(NonZeroUnchecked::new(n >> trailing_zeros))
}

/// Returns all the numbers N becomes on its way to falling to one.
#[no_panic]
pub fn transformations<T: Collatz>(n: NonZeroUnchecked<T>) -> Option<Vec<T>> {
    let mut n = n;
    let mut trans: Vec<T> = vec![n.value];
    while !n.value.is_one() {
        n = rules::basic(n)?;
        trans.push(n.value);
    }
    Some(trans)
}

pub mod bouncy_numbers;
pub mod check_range;
pub mod decrease_steps;
pub mod fall;
pub mod rules;
pub mod steps;
pub mod steps_range;

mod tests {

    // Number of steps to reach 1 for integers 1..=72 (according to the Online Encyclopedia of Integer Sequences)
    // For some reason this is counted as dead code currently, but it isn't dead code, I just convert it to a vec, or iter when using it.
    #[allow(dead_code)]
    const OEIS_STEPS: [u32; 72] = [
        0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, 20, 7, 7, 15, 15, 10, 23, 10,
        111, 18, 18, 18, 106, 5, 26, 13, 13, 21, 21, 21, 34, 8, 109, 8, 29, 16, 16, 16, 104, 11,
        24, 24, 24, 11, 11, 112, 112, 19, 32, 19, 32, 19, 19, 107, 107, 6, 27, 27, 27, 14, 14, 14,
        102, 22,
    ];

    // Make sure the steps returned by steps::omega conforms with the OEIS STEPS
    #[test]
    fn steps_module_works() {
        use crate::NonZeroUnchecked;

        // Unwrap here is fine, since if it panics during this test then there's a bug.
        #[allow(clippy::unwrap_used)]
        let step_counts: Vec<u32> = (1u32..=72u32)
            .map(|n| crate::steps::omega(NonZeroUnchecked::new(n)).unwrap())
            .collect();
        assert_eq!(OEIS_STEPS.to_vec(), step_counts);
    }

    #[test]
    fn steps_range_module_works() {
        use crate::NonZeroUnchecked;
        let start = NonZeroUnchecked::new(1u32);
        let end = NonZeroUnchecked::new(73u32);
        #[allow(clippy::unwrap_used)]
        let step_counts: Vec<u32> = crate::steps_range::omega(start, end)
            .map(|x| x.unwrap())
            .collect();

        assert_eq!(OEIS_STEPS.to_vec(), step_counts);
    }
}
