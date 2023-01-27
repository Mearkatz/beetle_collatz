//! A collection of functions relating to the Collatz conjecture

#![deny(missing_docs)]

use std::num::NonZeroU128;

/// This has the effect of dividing a number by 2 until it is odd.
/// Odd numbers are simply returned.
pub fn divide_while_even(n: NonZeroU128) -> u128 {
    let m: u128 = n.into();
    m >> m.trailing_zeros()
}

/// Same as divide_while_even, but also returns how many times the number was divided by 2 before becoming odd.
pub fn divide_while_even_and_trailing_zeros(n: NonZeroU128) -> (u128, u32) {
    let zeros = n.trailing_zeros();
    let m: u128 = n.into();
    (m >> zeros, zeros)
}

/// Returns all the numbers N becomes on its way to falling to one.
pub fn transformations(n: NonZeroU128) -> Vec<u128> {
    let mut n: u128 = n.into();
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
