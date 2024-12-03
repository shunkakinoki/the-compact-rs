//! The arbiter binary for the compact-rs project.

use clap::Parser;
use tokio::runtime::Runtime;

/// Command line arguments for the arbiter binary.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The address to listen on
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,

    /// The port to listen on  
    #[clap(short, long, default_value = "8080")]
    port: u16,
}

/// Main entry point for the arbiter binary.
fn main() {
    let args = Args::parse();

    let rt = Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(async_main(args));
}

/// Async main function that runs the arbiter.
async fn async_main(args: Args) {
    // TODO: Implement arbiter logic here
    println!("Running arbiter on {}:{}", args.address, args.port);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        Args::parse();
    }
}
