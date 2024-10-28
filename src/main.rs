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
            scanner.parse_sourse();
            for token in scanner.tokens {
                println!("{:?}", token);
            }
        }
    }
    Ok(())
}
