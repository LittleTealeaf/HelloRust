//! # Cargo
//! `cargo` is a collection of utilities to make performing certain actions more convenient

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo::add_one(arg);
///
/// assert_eq!(6,answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Commonly use sections:
// # Panics
// # Errors
// # Safety

// Code blocks in documentations are automatically called as tests

/// # Art
///
/// A library for modeling artistic concepts
pub mod Art {

    pub use self::kinds::PrimaryColor;
    pub use self::kinds::SecondaryColor;
    pub use self::utils::mix;

    pub mod kinds {
        /// The primary colors according to the RYB color model
        pub enum PrimaryColor {
            Red,
            Yellow,
            Blue
        }

        /// The secondary colors according to the RYB color model.
        pub enum SecondaryColor {
            Orange,
            Green,
            Purple
        }
    }

    pub mod utils {
        use crate::Art::kinds::*;

        /// Combines two primary colors in equal amounts to create a secondary color
        pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
            match c1 {
                Red => match c2 {
                    Red => panic!("Please provide two different colors"),
                    Yellow => SecondaryColor::Orange,
                    Blue => SecondaryColor::Purple
                },
                Yellow => match c2 {
                    Red => SecondaryColor::Orange,
                    Yellow => panic!("Please provide two different colors"),
                    Blue => SecondaryColor::Green,
                },
                Blue => match c2 {
                    Red => SecondaryColor::Purple,
                    Yellow => SecondaryColor::Green,
                    Blue => panic!("Please provide two different colors")
                }
            }
        }
    }
}
