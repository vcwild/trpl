//! Utility functions for working with colors.

use crate::kinds::*;

use crate::kinds::PrimaryColor::*;
use crate::kinds::SecondaryColor::*;

/// Combines two primary colors in equal amounts to create
/// a secondary color.
///
/// # Panics
///
/// This function will panic if the two colors cannot be combined.
///
/// # Examples
///
/// ```
/// use crate::utils::mix;
/// use crate::kinds::PrimaryColor::*;
/// use crate::kinds::SecondaryColor::*;
///
/// assert_eq!(mix(Red, Yellow), Orange);
/// ```
pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
    match (c1, c2) {
        (Red, Yellow) => Orange,
        (Yellow, Red) => Orange,
        (Yellow, Blue) => Green,
        (Blue, Yellow) => Green,
        (Blue, Red) => Purple,
        (Red, Blue) => Purple,
        _ => panic!("Color combination not supported"),
    }
}
