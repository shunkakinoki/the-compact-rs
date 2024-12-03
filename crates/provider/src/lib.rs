//! The provider crate for the compact-rs project.

/// Main entry point for the provider crate.
pub fn main() {
    println!("Hello from the provider crate!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
