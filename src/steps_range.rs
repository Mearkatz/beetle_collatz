//! Functions for mapping integers to the number of steps they take to reach 1.

use crate::{Collatz, NonZero};

/// Maps each number N in the range `nums` to its steps to reach 1 using steps::basic.
/// Performance should be pretty good, but consider using steps_range::omega for better performance.

pub fn alpha<T: Collatz>(start: NonZero<T>, stop: NonZero<T>) -> impl Iterator<Item = u32> {
    let (start, stop) = (start.0, stop.0);
    num::iter::range(start, stop)
        .map(NonZero)
        .map(crate::steps::alpha)
}

/// Ideally much faster than steps_range::alpha, by use of steps::omega instea of steps::alpha.
///
/// Potentially less stable as a result, and may panic or overflow more often, I'm not sure yet.
pub fn omega<T: Collatz>(start: NonZero<T>, stop: NonZero<T>) -> impl Iterator<Item = Option<u32>> {
    let (start, stop) = (start.0, stop.0);
    num::iter::range(start, stop)
        .map(NonZero)
        .map(crate::steps::omega)
}
