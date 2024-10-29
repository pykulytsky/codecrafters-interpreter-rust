use clap::Parser as ClapParser;
use std::process::exit;

use cli::*;
use lexer::Lexer;

use crate::parser::Parser;

mod cli;
mod lexer;
mod parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Commands::Tokenize { filename } => {
            let mut lexer = Lexer::new(&filename).await?;
            for token in lexer.by_ref() {
                println!("{:?}", token);
            }
            if lexer.result.is_err() {
                exit(65);
            }
        }
        Commands::Parse { filename } => {
            let lexer = Lexer::new(&filename).await?;
            let parser = Parser::new(lexer);
            for expr in parser {
                println!("{:?}", expr);
            }
        }
    }
    Ok(())
}
