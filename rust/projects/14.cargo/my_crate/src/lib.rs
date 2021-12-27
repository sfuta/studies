// doc comment for my_crate
//! # My Crate
//! 
//! `my_crate` is a collection of utilities to make performing certain
//! caluculations more convert.

// doc comment for add_one
/// Adds one to the number given
/// 
/// # Examples
/// 
/// ```rust
/// let file = 5;
/// 
/// assert_eq!(6, sandbox::add_one(5));
/// ```
/// 
/// # Panics
/// 
/// `Panic conditioins`
/// 
/// # Errors
/// 
/// `Error conditioins`
/// 
/// # Safety
/// 
/// `Unsafe detail`
pub fn add_one(x: i32) -> i32 {
    x + 1
}