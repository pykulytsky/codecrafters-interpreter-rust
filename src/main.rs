#![allow(dead_code, unused)]

use clap::Parser as ClapParser;
use std::process::exit;

use cli::*;
use lexer::Lexer;

use crate::parser::{Expr, Parser, Stmt};

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
            let mut parser = Parser::new(lexer);
            for expr in parser.by_ref() {
                println!("{:?}", expr);
            }
            if parser.result.is_err() {
                exit(65);
            }
        }

        Commands::Evaluate { filename } => {
            let lexer = Lexer::new(&filename).await?;
            let mut parser = Parser::new(lexer);
            for stmt in parser.by_ref() {
                if let Stmt::Expr(expr) = stmt {
                    match expr.evaluate() {
                        Ok(res) => {
                            println!("{:?}", res);
                        }
                        Err(err) => {
                            eprintln!("{}", err);
                            exit(70);
                        }
                    }
                }
            }
            if parser.result.is_err() {
                exit(65);
            }
        }
        Commands::Run { filename } => {
            let lexer = Lexer::new(&filename).await?;
            let mut parser = Parser::new(lexer);
            for stmt in parser.by_ref() {
                stmt.run();
            }

            if parser.result.is_err() {
                exit(65);
            }
        }
    }
    Ok(())
}
