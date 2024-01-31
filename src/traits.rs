//! Traits related to the Collatz Conjecture and the rules of it

use std::ops::Add;

use num::One;

/// Gives access to functions related to the rules of the Collatz Conjecture
pub trait Rules: Sized {
    /// Returns `3N + 1`.
    #[must_use]
    fn odd_rule(self) -> Self;
    /// Returns `N / 2`.
    #[must_use]
    fn even_rule(self) -> Self;

    /// If N is odd, this returns `3N + 1`, otherwise `N / 2`
    #[must_use]
    fn rules(self) -> Self;

    /// If N is odd, this returns `(3N + 1) / 2`, otherwise `N / 2`.    
    #[must_use]
    fn rules_halve_odds(self) -> Self;

    /// If N is odd, this returns `3N + 1`, otherwise `N (with any trailing zeros removed)`
    #[must_use]
    fn rules_remove_trailing_zeros(self) -> Self;
}

/// Gives access to functions for counting the steps a number takes to reach 1,
/// given the rules of the Collatz Conjecture
pub trait Steps: Sized {
    /// Returns how many steps a number takes to reach 1,    
    fn steps_to_one(self) -> u64;

    /// Same as `steps_to_one` but faster for even numbers
    fn steps_to_one_for_even_number(self) -> u64;

    /// Same as `steps_to_one` but faster for odd numbers
    fn steps_to_one_for_odd_number(self) -> u64;

    /// Returns how many steps a number takes to decrease,    
    fn steps_to_decrease(self) -> u64;

    /// Steps for an odd number to decrease
    fn steps_to_decrease_for_odd_number(self) -> u64;
}

/// Gives access to a function that shows the transformations of a number as it falls to one
pub trait Transformations: Rules {
    /// Returns every number that this number becomes as it falls to one
    fn transformations_to_one(&self) -> Vec<Self>;
}

/// Extends the `num::One` trait to the integers Two and Three
pub trait TwoThree: One + Add<Output = Self> {
    #[must_use]
    fn two() -> Self {
        Self::one() + Self::one()
    }

    #[must_use]
    fn three() -> Self {
        Self::one() + Self::one() + Self::one()
    }
}
