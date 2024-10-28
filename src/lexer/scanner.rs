use crate::lexer::{LexerResult, Token};

#[derive(Debug)]
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub async fn new(filename: &str) -> LexerResult<Self> {
        let source = tokio::fs::read_to_string(filename).await?;
        Ok(Scanner {
            source,
            tokens: vec![],
        })
    }
    pub fn parse_sourse(&mut self) {
        while !self.source.is_empty() {
            let ch = self.source.remove(0);
            match ch {
                '(' => self.tokens.push(Token::new_left_paren()),
                ')' => self.tokens.push(Token::new_right_paren()),
                '{' => self.tokens.push(Token::new_left_brace()),
                '}' => self.tokens.push(Token::new_right_brace()),
                '*' => self.tokens.push(Token::new_star()),
                '.' => self.tokens.push(Token::new_dot()),
                ',' => self.tokens.push(Token::new_comma()),
                '+' => self.tokens.push(Token::new_plus()),
                '-' => self.tokens.push(Token::new_minus()),
                ';' => self.tokens.push(Token::new_semicolon()),
                '/' => self.tokens.push(Token::new_slash()), // TODO: handle comments
                '\n' => {}
                _ => todo!("Unexpected token: {}", ch),
            }
        }
        self.tokens.push(Token::EOF);
    }
}
