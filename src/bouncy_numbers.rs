//! Functions for finding numbers who take the most steps to reach 1, given the rules.

use beetle_nonzero::NonZeroUnchecked;

use crate::Collatz;

/// Finds a number N that takes the most steps S to reach 1 in a given range
/// Returns (N, S)
/// Note: the range provided must be ascending
pub fn alpha<T: Collatz>(start: NonZeroUnchecked<T>, end: NonZeroUnchecked<T>) -> Option<(T, u32)> {
    let mut record_number = T::zero();
    let mut record_steps = 0;

    let (start, end) = (start.value, end.value);

    for i in num::iter::range(start, end) {
        let steps = crate::steps::omega(NonZeroUnchecked::new(i))?;
        if record_steps < steps {
            record_number = i;
            record_steps = steps;
        }
    }
    Some((record_number, record_steps))
}

/// Faster (hopefully) version of `bouncy_numbers::alpha`
// pub fn omega<T: Collatz>(range: num::iter::Range<T>) -> Option<(T, u32)> {
//     range.map(|n| (n, crate::steps::omega(NonZero(n)))).reduce(
//         |(num1, steps1), (num2, steps2)| -> Option<(T, u32)> {
//             // let steps1 = steps1.ok()?;
//             // let steps2 = steps2.ok()?;
//             match (steps1, steps2) {
//                 (Ok(s1), Ok(s2)) => {
//                     if s2 > s1 {
//                         Some((num2, s2))
//                     } else {
//                         Some((num1, s1))
//                     }
//                 }
//                 _ => None,
//             }
//             // if steps2 > steps1 {
//             //     (num2, Ok(steps2))
//             // } else {
//             //     (num1, Ok(steps1))
//             // }
//         },
//     )
// }

/// Finds a number N that takes the most steps S to reach 1 in a given range
/// Returns (N, S)
/// Note: the range provided must be ascending
/// /// Same as `beetle_collatz::bouncy_numbers::optimized`, but is multi-threaded and probably way faster
// #[cfg(feature = "threaded")]
// pub fn omega_threaded<T: Collatz>(start: NonZero<T>, end: NonZero<T>) -> eyre::Result<(u128, u32)> {
//     use rayon::prelude::{IntoParallelIterator, ParallelIterator};
//     let (start, end) = (start.0, end.0);

//     // preventing weirdness
//     if start >= end {
//         Err(eyre!(
//             "bouncy_numbers::omega_threaded expects `start` to be less than `end`"
//         ));
//     }

//     let ans = (start..end)
//         .into_par_iter()
//         .map(|n| (n, crate::steps::omega(NonZero(n)?)))
//         .reduce(
//             || (0_u128, 0_u32),
//             |(a, a_steps), (b, b_steps)| -> (u128, u32) {
//                 if a_steps > b_steps {
//                     (a, a_steps)
//                 } else {
//                     (b, b_steps)
//                 }
//             },
//         );
// }

/// Finds every number N, which takes more steps to reach 1 than all numbers before it.
/// Returns this as a sequence starting at START, and ending at END, with every number N paired with its corresponding number of steps S
pub fn calculate_bouncy_sequence<T: Collatz>(
    start: NonZeroUnchecked<T>,
    stop: NonZeroUnchecked<T>,
) -> Option<Vec<(T, u32)>> {
    let mut retval = Vec::new();
    let mut record_steps = 0;

    for n in num::iter::range(start.value, stop.value) {
        let steps = crate::steps::omega(NonZeroUnchecked::new(n))?;
        if steps > record_steps {
            record_steps = steps;
            retval.push((n, steps));
        }
    }
    Some(retval)
}
