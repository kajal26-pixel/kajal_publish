//! # Publish
//! 
//! A library for modelling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;



pub mod kinds {
    /// the primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    ///combines two primary colors in equal amounts to create a secondary color.
    pub fn mix(c1:PrimaryColor, c2:PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}