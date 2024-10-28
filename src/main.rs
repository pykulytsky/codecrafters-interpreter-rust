use clap::Parser;
use cli::*;
use std::fs;

mod cli;
mod lexer;

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Tokenize { filename } => {
            let file_contents = fs::read_to_string(&filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", &filename);
                String::new()
            });

            if !file_contents.is_empty() {
                panic!("Scanner not implemented");
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
    }
}
