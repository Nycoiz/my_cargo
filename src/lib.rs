//! #Art
//! 
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix_colors;

pub mod kinds
{
    ///The primary colors according to the RYB color model
    pub enum PrimaryColor
    {
        Red,
        Yellow,
        Blue,
    }

    ///The secondary colors according to the RYB color model
    pub enum SecondaryColor
    {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils
{
    use crate::kinds::*;
    pub fn mix_colors(color1: PrimaryColor, color2: PrimaryColor) -> SecondaryColor
    {
        SecondaryColor::Green
    }
}