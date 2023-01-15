//! Functions for finding numbers who take the most steps to reach 1, given the rules.
use std::num::NonZeroU128;

/// Finds a number N that takes the most steps S to reach 1 in a given range
/// Returns (N, S)
/// Note: the range provided must be ascending
pub fn basic(start: NonZeroU128, end: NonZeroU128) -> (u128, u32) {
    let mut record_number: u128 = 0;
    let mut record_steps: u32 = 0; 

    let start: u128 = start.into();
    let end: u128 = end.into();

    for i in start..end {
        let steps = crate::steps::omega(i.try_into().unwrap());
        if record_steps < steps {
            record_number = i;
            record_steps = steps;
        }
    }
    (record_number, record_steps)
}

/// Same as `bouncy_numbers::basic`, but ideally faster
pub fn optimized(start: NonZeroU128, end: NonZeroU128) -> (u128, u32) {
    let start: u128 = start.into();
    let end: u128 = end.into();

    (start..end)
        .map(|n| (n, crate::steps::omega(n.try_into().unwrap())))
        .reduce(|(num1, steps1), (num2, steps2)| {
            if steps2 > steps1 {
                (num2, steps2)
            } else {
                (num1, steps1)
            }
        })
        .unwrap()
}

/// Finds every number N, which takes more steps to reach 1 than all numbers before it.
/// Returns this as a sequence starting at START, and ending at END, with every number N paired with its corresponding number of steps S
pub fn calculate_bouncy_sequence(start: NonZeroU128, end: NonZeroU128) -> Vec<(u128, u32)> {
    let mut retval = vec![];
    let mut record_steps = 0;
    let start: u128 = start.into();
    let end: u128 = end.into();

    for n in start..end {
        let steps = crate::steps::omega(n.try_into().unwrap());
        if steps > record_steps {
            record_steps = steps;
            retval.push((n, steps));
        }
    }
    retval
}
