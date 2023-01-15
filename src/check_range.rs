//! For checking to see if ranges of numbers fall to 1
use std::{
    hint::black_box,
    num::{NonZeroU128, NonZeroUsize},
};

/// Checks a range of numbers to ensure they all fall to 1.
pub fn alpha(start: NonZeroU128, end: NonZeroU128) -> bool {
    let start: u128 = start.into();
    let end: u128 = end.into();
    let mut nums = start..end;

    nums.all(|x| {
        crate::fall::alpha(x.try_into().unwrap());
        true
    })
}

/// Same as check_range_unoptimized but uses fall::omega_boolean instead of fall::standard_boolean
pub fn omega(start: NonZeroU128, end: NonZeroU128) -> bool {
    let start: u128 = start.into();
    let end: u128 = end.into();
    let mut nums = start..end;
    nums.all(|x| {
        crate::fall::omega(x.try_into().unwrap());
        true
    })
}

/// Same as check_range_omega, but takes advantage of knowing all the numbers in the range are odd first
pub fn omega_all_odds(start: NonZeroU128, end: NonZeroU128, step: NonZeroUsize) -> bool {
    let start: u128 = start.into();
    let end: u128 = end.into();
    let step: usize = step.into();

    assert_eq!(step & 1, 1);
    assert!(start < end);

    (start..end).step_by(step).all(|x| {
        crate::fall::omega_n_is_odd(x.try_into().unwrap());
        black_box(());
        true
    })
}

/// Multi-threaded version of check_range::alpha
#[cfg(feature = "threaded")]
pub fn alpha_threaded(nums: Range<u128>) -> bool {
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};
    nums.into_par_iter().all(|n| {
        crate::fall::alpha(n.try_into().unwrap());
        true
    })
}

/// Multi-threaded version of check_range::omega
#[cfg(feature = "threaded")]
pub fn omega_threaded(nums: Range<u128>) -> bool {
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};
    nums.into_par_iter().all(|n| {
        crate::fall::omega(n.try_into().unwrap());
        true
    })
}
