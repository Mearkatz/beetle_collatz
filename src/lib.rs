//! A collection of functions relating to the Collatz conjecture

#![deny(missing_docs)]

use std::{fmt::{Debug, Display}, num::NonZeroU128};

use color_eyre::eyre::{self, eyre};

use num::{Integer, Unsigned, PrimInt};
use no_panic::no_panic;

/// Types implementing this can be passed to the most if not all functions in this library
pub trait Collatz: Unsigned + Integer + PrimInt + Debug + Display {}

impl<T> Collatz for T
where T: Unsigned + Integer + PrimInt + Debug + Display {}


struct NonZero<T: Collatz>(T);

impl<T: Collatz> NonZero<T> {
    fn try_new(n: T) -> Option<Self> {
        if n.is_zero() {None} else {Some(Self(n))}
    }

    fn into(self) -> T {
        self.0
    }
}


/// This has the effect of dividing a number by 2 until it is odd.
/// Odd numbers are simply returned.
#[no_panic]
pub fn divide_while_even<T: Collatz>(n: NonZero<T>) -> eyre::Result<T> {
    let n = n.into();
    let odd = n >> n.trailing_zeros().try_into()?;
    Ok(odd)
}

/// Same as divide_while_even, but also returns how many times the number was divided by 2 before becoming odd.
#[no_panic]
pub fn divide_while_even_and_trailing_zeros<T: Collatz>(n: NonZero<T>) -> eyre::Result<(T, usize)> {
    let n = n.into();
    let trailing_zeros: usize = n.trailing_zeros().try_into()?;
    let odd = n >> trailing_zeros;
    Ok((odd, trailing_zeros))
}

/// Returns all the numbers N becomes on its way to falling to one.
pub fn transformations<T: Collatz>(n: NonZero<T>) -> Vec<T> {
    let mut n = n.into();
    let mut trans: Vec<u128> = vec![n];
    while n != 1 {
        n = rules::basic(n.try_into().unwrap());
        trans.push(n);
    }

    trans
}

pub mod bouncy_numbers;
pub mod check_range;
pub mod fall;
pub mod rules;
pub mod steps;
pub mod steps_range;

mod tests {

    // Make sure the steps returned by steps::omega
    #[test]
    fn steps_range_conforms_to_oeis() {
        // Number of steps to reach 1 for integers 1..=72
        let oeis_steps: Vec<u32> = vec![
            0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, 20, 7, 7, 15, 15, 10, 23,
            10, 111, 18, 18, 18, 106, 5, 26, 13, 13, 21, 21, 21, 34, 8, 109, 8, 29, 16, 16, 16,
            104, 11, 24, 24, 24, 11, 11, 112, 112, 19, 32, 19, 32, 19, 19, 107, 107, 6, 27, 27, 27,
            14, 14, 14, 102, 22,
        ];

        // println!("OEIS STEPS HAS LENGTH {}", oeis_steps.len());

        let r = 1..(oeis_steps.len() + 1);

        let step_counts: Vec<u32> = r
            .map(|n| -> u128 { n.try_into().unwrap() })
            .map(|n| crate::steps::omega(n.try_into().unwrap()))
            .collect();

        // println!("STEP COUNTS HAS LENGTH {}", step_counts.len());

        for i in 0..oeis_steps.len() {
            println!(
                "{} => OEIS: {}, LIB: {}",
                i + 1,
                oeis_steps[i],
                step_counts[i]
            );
            assert_eq!(oeis_steps[i], step_counts[i]);
        }
    }
}
