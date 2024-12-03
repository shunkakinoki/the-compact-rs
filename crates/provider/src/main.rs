//! The provider crate for the compact-rs project.

fn main() {
    println!("Hello from the provider crate!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        main();
    }
}
