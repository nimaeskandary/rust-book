//! # docs example
//! 
//! run `cargo doc --open` to see
//! 

/// Adds one to the number given
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = docs::add_one(arg);
/// 
/// assert_eq!(answer, 6);
/// ```
pub fn add_one(input: u32) -> u32 {
    return input + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
