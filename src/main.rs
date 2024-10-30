#![allow(dead_code, unused)]

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
            for expr in parser.by_ref() {
                match expr {
                    parser::Expr::Literal(literal) => match literal {
                        parser::Literal::Str(s) => println!("{s}"),
                        parser::Literal::Number(n) => println!("{n}"),
                        parser::Literal::Logical(l) => println!("{l}"),
                        parser::Literal::Nil => println!("nil"),
                    },
                    parser::Expr::Unary(unary_kind, expr) => todo!(),
                    parser::Expr::Binary { op, left, right } => todo!(),
                    parser::Expr::Group(vec) => todo!(),
                }
            }
            if parser.result.is_err() {
                exit(65);
            }
        }
    }
    Ok(())
}
