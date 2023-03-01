//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Adds two to the number given.
///
/// # Examples
///
/// ```
/// let a = 5;
/// let b = 2;
/// let answer = my_crate::add_two_numbers(a, b);
/// assert_eq!(7, answer);
/// ```
///
pub fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_with_two_numbers() {
        let result = add_two_numbers(2, 3);
        assert_eq!(result, 5);
    }
}
