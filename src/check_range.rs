//! For checking to see if ranges of numbers fall to 1

use beetle_nonzero::NonZeroUnchecked;
use no_panic::no_panic;

use crate::Collatz;
use std::hint::black_box;

// trait PrimitiveUnsignedInteger {}
// impl PrimitiveUnsignedInteger for u8 {}
// impl PrimitiveUnsignedInteger for u16 {}
// impl PrimitiveUnsignedInteger for u32 {}
// impl PrimitiveUnsignedInteger for u64 {}
// impl PrimitiveUnsignedInteger for u128 {}

// /// Marks all primitive unsigned integers for use in parallel-ized stuff.
// /// Mainly for use with the parallel iterators of the rayon crate
// trait CollatzParallel: PrimitiveUnsignedInteger + Collatz {}
// impl<T> CollatzParallel for T where T: PrimitiveUnsignedInteger + Collatz {}

// struct RangeNonZeroUnsignedIntegers<T: CollatzParallel> {
//     start: NonZero<T>,
//     stop: NonZero<T>,
// }

// impl<T: CollatzParallel> RangeNonZeroUnsignedIntegers<T> {
//     fn new(start: NonZero<T>, stop: NonZero<T>) -> Self {
//         Self { start, stop }
//     }

//     fn to_range(self) -> Range<T> {
//         let (start, stop) = (self.start.0, self.stop.0);
//         range(start, stop)
//     }
// }

/// Checks a range of numbers to ensure they all fall to 1.
#[no_panic]
pub fn alpha<T: Collatz>(start: NonZeroUnchecked<T>, stop: NonZeroUnchecked<T>) -> bool {
    let (start, stop) = (start.value, stop.value);
    for i in num::iter::range(start, stop) {
        crate::fall::alpha(NonZeroUnchecked::new(i));
    }
    true
}

/// Same as check_range_unoptimized but uses fall::omega_boolean instead of fall::standard_boolean
#[no_panic]
pub fn omega<T: Collatz>(start: NonZeroUnchecked<T>, stop: NonZeroUnchecked<T>) -> bool {
    let (start, stop) = (start.value, stop.value);
    for i in num::iter::range(start, stop) {
        crate::fall::omega(NonZeroUnchecked::new(i));
    }
    true
}

/// Same as check_range_omega, but takes advantage of knowing all the numbers in the range are odd first
#[no_panic]
pub fn omega_all_odds<T: Collatz>(start: NonZeroUnchecked<T>, stop: NonZeroUnchecked<T>) -> bool {
    let (start, stop) = (start.value, stop.value);

    num::iter::range(start, stop).step_by(2).for_each(|x| {
        crate::fall::omega(NonZeroUnchecked::new(x));
        black_box(());
    });
    true
}

// /// Multi-threaded version of check_range::alpha
// #[no_panic]
// // #[cfg(feature = "threaded")]
// pub fn alpha_threaded(start: NonZero<u128>, stop: NonZero<u128>) -> bool {
//     use rayon::{iter::IntoParallelIterator, prelude::ParallelIterator};

//     let (start, stop) = (start.0, stop.0);

//     (start..stop).into_par_iter().all(|n| {
//         crate::fall::alpha(NonZero(n));
//         true
//     })
// }

// /// Multi-threaded version of check_range::omega
// #[no_panic]
// pub fn omega_threaded(start: u128, stop: u128) -> Option<bool> {
//     use rayon::{iter::IntoParallelIterator, prelude::ParallelIterator};
//     if start.is_zero() || stop.is_zero() {
//         None
//     } else {
//         Some((start..stop).into_par_iter().all(|n| {
//             crate::fall::omega(NonZero(n));
//             true
//         }))
//     }
// }
