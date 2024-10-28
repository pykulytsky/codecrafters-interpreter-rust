use std::process::exit;

use clap::Parser;
use cli::*;
use lexer::Scanner;

mod cli;
mod lexer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Commands::Tokenize { filename } => {
            let mut scanner = Scanner::new(&filename).await?;
            let res = scanner.parse_sourse();
            for token in scanner.tokens {
                println!("{:?}", token);
            }
            if res.is_err() {
                exit(65);
            }
        }
    }
    Ok(())
}
