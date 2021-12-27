/// 与えられた数値に1を足す
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