//! Traits related to the Collatz Conjecture and the rules of it

/// Types implementing this can be passed to the most if not all functions in this library
// trait Collatz: Unsigned + Integer + PrimInt + From<u8> {}

// impl<T> Collatz for T where T: Unsigned + Integer + PrimInt + From<u8> {}

pub trait Collatz: Rules + Steps + WithoutTrailingZeros + Bouncy + Transformations {}

/// Gives access to functions related to the rules of the Collatz Conjecture
pub trait Rules: Sized {
    /// Returns `3N + 1`.
    fn odd_rule(&self) -> Self;
    /// Returns `N / 2`.
    fn even_rule(&self) -> Self;

    /// If N is odd, this returns `3N + 1`, otherwise `N / 2`
    fn rules(&self) -> Self;

    /// If N is odd, this returns `(3N + 1) / 2`, otherwise `N / 2`.    
    fn rules_halve_odds(&self) -> Self;

    /// If N is odd, this returns `3N + 1`, otherwise `N (with any trailing zeros removed)`
    fn rules_remove_trailing_zeros(&self) -> Self;
}

/// Gives access to functions for counting the steps a number takes to reach 1,
/// given the rules of the Collatz Conjecture
pub trait Steps: Sized {
    /// Returns how many steps a number takes to reach 1,
    /// or None if there is an overflow
    fn steps_to_one(&self) -> u64;

    /// Returns how many steps a number takes to decrease,
    /// or None if there is an overflow
    fn steps_to_decrease(&self) -> u64;

    /// Same as steps_to_one but faster for even numbers
    fn steps_to_one_for_even_number(&self) -> u64;

    /// Same as steps_to_one but faster for odd numbers
    fn steps_to_one_for_odd_number(&self) -> u64;
}

/// Gives access to a function that returns a copy of a number without trailing zeros
pub trait WithoutTrailingZeros: Sized {
    /// Return type for `without_trailing_zeros`
    type R;
    /// Returns a copy of a number with its trailing zeros removed.
    /// This also is effectively the same as dividing the number by 2 until it is odd.
    fn without_trailing_zeros(&self) -> Self::R;
}

/// Gives access to a function that determines if a number if 'bouncy'.
/// I made the term up, but I'm sure there's a proper term for it somewhere.
pub trait Bouncy: Steps {
    /// Returns whether this number takes more steps to fall to one than all the previous numbers.
    /// Returns None if an overflow happens when calling `steps_to_one` on any integer along the way.
    fn is_bouncy(&self) -> bool;
}

/// Gives access to a function that shows the transformations of a number as it falls to one
pub trait Transformations: Rules {
    /// Returns every number that this number becomes as it falls to one
    fn transformations_to_one(&self) -> Vec<Self>;
}
