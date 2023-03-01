//! The kinds of colors.

/// The primary colors according to the RYB color model.
#[derive(Debug, PartialEq)]
pub enum PrimaryColor {
    Red,
    Yellow,
    Blue,
}

/// The secondary colors according to the RYB color model.
#[derive(Debug, PartialEq)]
pub enum SecondaryColor {
    Orange,
    Green,
    Purple,
}
