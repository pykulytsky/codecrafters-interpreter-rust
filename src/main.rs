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
            loop {
                let Some(stmt) = parser.next() else {
                    break;
                };
                if let Stmt::Expr(expr) = stmt {
                    match expr.evaluate(&parser.global_variables) {
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
            loop {
                let Some(stmt) = parser.next() else {
                    break;
                };
                match stmt.run(&parser.global_variables) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{}", err);
                        exit(70);
                    }
                }
            }

            if let Err(err) = parser.result {
                match err {
                    parser::error::ParserError::UndefinedVariable(_) => {
                        exit(70);
                    }
                    _ => {
                        exit(65);
                    }
                }
            }
        }
    }
    Ok(())
}
