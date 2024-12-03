//! The axum crate for the compact-rs project.

/// Main entry point for the axum crate.
pub fn main() {
    println!("Hello from the axum crate!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
