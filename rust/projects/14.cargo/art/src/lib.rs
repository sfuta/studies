//! # Art
//! 
//! The Library is modeling for artistic concept

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    pub enum PrimaryColor { Red, Yellow, Blue }
    pub enum SecondaryColor { Orange, Green, Purple }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Yellow) => SecondaryColor::Orange,
            (PrimaryColor::Yellow, PrimaryColor::Blue) => SecondaryColor::Green,
            (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
            _ => SecondaryColor::Purple
        }
        
    }
    
}
