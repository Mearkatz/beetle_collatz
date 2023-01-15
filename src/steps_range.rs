//! Functions for mapping integers to the number of steps they take to reach 1.
use std::num::NonZeroU128;

/// Maps each number N in the range `nums` to its steps to reach 1 using steps::basic.
/// Performance should be pretty good, but consider using steps_range::omega for better performance.
pub fn alpha(start: NonZeroU128, end: NonZeroU128) -> impl Iterator<Item = u32> {
    let start: u128 = start.into();
    let end: u128 = end.into();
    (start..end).map(|n| crate::steps::alpha(n.try_into().unwrap()))
}

/// Ideally much faster than steps_range::basic, by use of steps::omega instea of steps::basic.
///
/// Potentially less stable as a result, and may panic or overflow more often, I'm not sure yet.
pub fn omega(start: NonZeroU128, end: NonZeroU128) -> impl Iterator<Item = u32> {
    let start: u128 = start.into();
    let end: u128 = end.into();
    (start..end).map(|n| crate::steps::omega(n.try_into().unwrap()))
}
