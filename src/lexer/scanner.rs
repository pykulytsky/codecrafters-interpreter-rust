use crate::lexer::{LexerError, LexerResult, Token};

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
    pub fn parse_sourse(&mut self) -> LexerResult<()> {
        let mut line = 1;
        let mut res = Ok(());
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
                '=' if self.source.starts_with("=") => {
                    self.source.remove(0);
                    self.tokens.push(Token::new_equal_equal())
                }
                '=' => self.tokens.push(Token::new_equal()), // TODO: handle comments
                '\n' => {
                    line += 1;
                }
                _ => {
                    res = Err(LexerError::UnexpectedCharacter { line, ch });
                    eprintln!("[line {line}] Error: Unexpected character: {}", ch)
                }
            }
        }
        self.tokens.push(Token::EOF);
        res
    }
}
