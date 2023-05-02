//! For checking how many steps a number takes to decrease.

use beetle_nonzero::NonZeroUnchecked;
use no_panic::no_panic;

use crate::{rules, Collatz};

/// Returns how many steps 'n' takes to decreaseb below its starting value
#[no_panic]
pub fn alpha<T: Collatz>(mut n: NonZeroUnchecked<T>) -> Option<u32> {
    let starting_value = n.value;

    let mut steps = 0;
    while n.value >= starting_value {
        n = rules::basic(n)?;
        steps += 1;
    }

    Some(steps)
}
