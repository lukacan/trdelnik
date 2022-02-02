
  
use anyhow::Error;
use fehler::throws;
use clap::Parser;

mod command;

/// Trdelnik CLI
#[derive(Parser, Debug)]
enum Args {
    /// Run program tests
    Test { 
        /// Anchor project root
        #[clap(short, long, default_value = "./")]
        root: String 
    }
}

#[throws]
#[tokio::main]
async fn main() {
    match Args::parse() {
        Args::Test { root } => command::test(root).await?,
    }
}
