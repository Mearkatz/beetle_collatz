//! A collection of functions relating to the Collatz conjecture

/// This has the effect of dividing a number by 2 until it is odd.
/// Odd numbers are simply returned.
pub fn divide_while_even(n: u128) -> u128 {
    n >> n.trailing_zeros()
}

/// Same as divide_while_even, but also returns how many times the number was divided by 2 before becoming odd.
pub fn divide_while_even_and_trailing_zeros(n: u128) -> (u128, u32) {
    let zeros = n.trailing_zeros();    
    (n >> zeros, zeros)
}

/// Contains functions that apply the rules of the collatz conjecture in more performant ways
pub mod rules {
    /// Applies the rules of the collatz conjecture to a number N, and returns the result.
    /// If N is ODD: returns 3n + 1,
    /// If N is EVEN: returns n / 2.
    /// All other functions in this module are faster than this one.
    /// Should only be used when benchmarking other functions in this module.
    pub fn basic(n: u128) -> u128 {
        if n & 1 == 1 {
            // N is ODD
            3 * n + 1
        } else {
            // N is EVEN
            n / 2
        }
    }

    // Same as the `basic` function,
    // except if N is odd, it also divides it by 2 before returning it.
    // for use with the `fall` function
    /// Do not use if the precise number of steps needed to reach 1 is important.
    pub fn halve_odds(n: u128) -> u128 {
        match n & 1 {
            1 => (3 * n + 1) / 2, // ODD
            _ => n / 2,           // EVEN
        }
    }

    /// In theory faster than halve_odds, in practice, seems about the same.    
    pub fn trailing_zeros(n: u128) -> u128 {
        crate::divide_while_even(basic(n))
    }

    /// same as rules::trailing_zeros, but we know for sure that N is ODD.
    pub fn trailing_zeros_num_is_odd(n: u128) -> u128 {
        crate::divide_while_even(3 * n + 1)
    }

    /// same as rules::trailing_zeros, but we know for sure that N is EVEN
    pub fn trailing_zeros_num_is_even(n: u128) -> u128 {
        crate::divide_while_even(n)
    }
}

/// Contains functions that apply the rules of the collatz conjecture until a number reaches one
/// Functions herein with no return value are meant for benchmarking -- and because return values aren't strictly necessary.
/// If needed there are also versions of each function that return a boolean value if they succeed.
pub mod fall {
    /// Applies the rules of the collatz conjecture until a number reaches one
    /// This exists for benchmarking other faster functions' speed relative this one.
    /// This aims to always be a correct implementation, but not very fast.
    /// Do not use if performance is important to you.
    pub fn alpha(mut n: u128) {
        while n != 1 {
            n = crate::rules::basic(n);
        }
    }

    /// fall::alpha but MUCH FASTER.    
    pub fn omega(mut n: u128) {
        // If n is even, return immediately,
        // because the number will decrease,
        // which also means it will reach 1.
        // if n & 1 == 1 {
        //     omega_n_is_odd(n);
        // }

        loop {
            let odd = n & 1 == 1;

            // If N is even it decreases, thus it will reach 1.
            if !odd {
                return;
            }

            // If N is odd, and has more than 2 trailing zeros, it is about to decrease, thus it will reach 1.
            let next_n = 3 * n + 1;
            if odd && (next_n.trailing_zeros() > 1) {
                return;
            }

            /*
            (SHOULD BE) Equivalent to the following:
            ```rust
                n = next_if_odd / 2;
                n = 3 * n + 1;
            ```
            */
            n = (9 * n + 5) / 2;
        }
    }

    /// Same as Omega, but faster than Omega when N is known to be odd, since it bypasses an if-statement.
    pub fn omega_n_is_odd(mut n: u128) {
        loop {
            let m = 3 * n + 1;
            if m.trailing_zeros() > 1 {
                return;
            }
            n = m / 2;
        }
    }
}

/// Functions for counting how many steps a number takes to reach 1
pub mod steps {
    use crate::divide_while_even_and_trailing_zeros;

    /// Counts how many steps N takes to reach 1.
    /// Probably slower than other functions in this module.
    pub fn alpha(mut n: u128) -> u32 {
        let mut steps = 0;
        while n != 1 {
            if n & 1 == 1 {
                n = (3 * n) + 1;
                steps += 1;
            }
            n /= 2;
            steps += 1;
        }
        steps
    }

    /// Ideally far faster than steps::basic. Further testing needed.
    pub fn omega(n: u128) -> u32 {
        // let mut steps = 0;
        // while n != 1 {
        //     // See rules_super_speed for an explanation
        //     let m = match n & 1 {
        //         1 => 3 * n + 1,
        //         _ => n,
        //     };
        //     let num_zeroes = m.trailing_zeros();
        //     n = m / (1 << num_zeroes);
        //     steps += num_zeroes;
        // }
        // steps

        /*
        Big brain:
        If N is Even, simply make it odd!
        */
        if n & 1 != 1 {
            omega_n_is_even(n)
        }
        else {
            omega_n_is_odd(n)
        }
    }


    // Makes N odd, then passes it to omega_n_is_odd
    pub fn omega_n_is_even(n: u128) -> u32 {
        let (n, steps) = divide_while_even_and_trailing_zeros(n);            
        steps + omega_n_is_odd(n)
    }

    /// Same as steps::omega, but N is known to be odd, saving some computations
    pub fn omega_n_is_odd(mut n: u128) -> u32 {
        let mut steps = 0;
        while n != 1 {
            // See rules_super_speed for an explanation
            let m = 3 * n + 1;
            let zeros = m.trailing_zeros();
            n = m >> zeros;
            steps += zeros + 1;
        }
        steps
    }
}

/// Functions for mapping integers to the number of steps they take to reach 1.
pub mod steps_range {
    use std::ops::Range;

    /// Maps each number N in the range `nums` to its steps to reach 1 using steps::basic.
    /// Performance should be pretty good, but consider using steps_range::omega for better performance.
    pub fn basic(nums: Range<u128>) -> impl Iterator<Item = u32> {
        nums.map(crate::steps::alpha)
    }

    /// Ideally much faster than steps_range::basic, by use of steps::omega instea of steps::basic.
    ///
    /// Potentially less stable as a result, and may panic or overflow more often, I'm not sure yet.
    pub fn omega(nums: Range<u128>) -> impl Iterator<Item = u32> {
        nums.map(crate::steps::omega)
    }
}

/// For checking to see if ranges of numbers fall to 1
pub mod check_range {
    use std::{hint::black_box, ops::Range};

    /// Checks a range of numbers to ensure they all fall to 1.
    pub fn check_range_unoptimized(mut nums: Range<u128>) -> bool {
        nums.all(|x| {
            crate::fall::alpha(x);
            true
        })
    }

    /// Same as check_range_unoptimized but uses fall::omega_boolean instead of fall::standard_boolean
    pub fn check_range_omega(mut nums: Range<u128>) -> bool {
        nums.all(|x| {
            crate::fall::omega(x);
            true
        })
    }

    /// Same as check_range_omega, but takes advantage of knowing all the numbers in the range are odd first
    pub fn check_range_omega_all_odds(start: u128, end: u128, step: usize) -> bool {
        assert!(start % 2 != 0); // start must be odd, since it's the first number we check
        assert!(step % 2 == 0); // step must be even

        (start..end).step_by(step).all(|x| {
            crate::fall::omega_n_is_odd(x);
            black_box(());
            true
        })
    }
}

/// Functions for finding numbers who take the most steps to reach 1, given the rules.
pub mod bouncy_numbers {

    /// Finds a number N that takes the most steps S to reach 1 in a given range
    /// Returns (N, S)
    /// Note: the range provided must be ascending
    pub fn basic(start: u128, end: u128) -> (u128, u32) {
        let mut record_number: u128 = 0;
        let mut record_steps: u32 = 0;
        assert!((start > 0) && (start < end)); // preventing weirdness

        for i in start..end {
            let steps = crate::steps::omega(i);
            if record_steps < steps {
                record_number = i;
                record_steps = steps;
            }
        }
        (record_number, record_steps)
    }

    /// Same as `bouncy_numbers::basic`, but ideally faster
    pub fn optimized(start: u128, end: u128) -> (u128, u32) {
        assert!((start > 0) && (start < end)); // preventing weirdness

        (start..end)
            .map(|n| (n, crate::steps::omega(n)))
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
    pub fn calculate_bouncy_sequence(start: u128, end: u128) -> Vec<(u128, u32)> {
        let mut retval = vec![];
        let mut record_steps = 0;
        for n in start..end {
            let steps = crate::steps::omega(n);
            if steps > record_steps {
                record_steps = steps;
                retval.push((n, steps));
            }
        }
        retval
    }
}
