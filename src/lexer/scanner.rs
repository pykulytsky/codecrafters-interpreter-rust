use crate::lexer::{tokens::RESERVED_WORDS, LexerError, LexerResult, Token};

#[derive(Debug)]
pub struct Lexer {
    pub source: String,
    pub current_line: usize,
    pub result: LexerResult<()>,
    cursor: usize,
    done: bool,
}

impl Lexer {
    pub async fn new(filename: &str) -> LexerResult<Self> {
        let source = tokio::fs::read_to_string(filename).await?;
        Ok(Lexer {
            source,
            cursor: 0,
            current_line: 1,
            result: Ok(()),
            done: false,
        })
    }

    fn peek_char(&self) -> char {
        self.source[self.cursor..].chars().next().unwrap()
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }

    fn starts_with(&self, s: &str) -> bool {
        self.source[self.cursor..].starts_with(s)
    }

    pub fn parse_to_end(&mut self) -> Vec<Token> {
        self.by_ref().collect()
    }
}

fn is_number(ch: char) -> bool {
    ch.is_ascii_digit() || ch == '.'
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
}

pub const UNEXPECTED: &[char] = &['#', '$', '%', '@'];

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.source.len() {
            if !self.done {
                self.done = true;
                return Some(Token::EOF);
            }
            return None;
        }

        let mut ch = self.peek_char();
        self.advance();
        while UNEXPECTED.contains(&ch) {
            self.result = Err(LexerError::UnexpectedCharacter {
                line: self.current_line,
                ch,
            });
            eprintln!(
                "[line {}] Error: Unexpected character: {}",
                self.current_line, ch
            );

            if self.source[self.cursor..].is_empty() {
                self.done = true;
                return Some(Token::EOF);
            }
            ch = self.peek_char();
            self.advance();
        }

        match ch {
            '(' => Some(Token::left_paren()),
            ')' => Some(Token::right_paren()),
            '{' => Some(Token::left_brace()),
            '}' => Some(Token::right_brace()),
            '*' => Some(Token::star()),
            '.' => Some(Token::dot()),
            ',' => Some(Token::comma()),
            '+' => Some(Token::plus()),
            '-' => Some(Token::minus()),
            ';' => Some(Token::semicolon()),
            '/' if self.starts_with("/") => {
                if let Some(pos) = self.source[self.cursor..].find('\n') {
                    self.cursor += pos + 1;
                    self.current_line += 1;
                    self.next()
                } else {
                    self.cursor = self.source.len();
                    self.done = true;
                    Some(Token::EOF)
                }
            }
            '/' => Some(Token::slash()),
            '=' if self.starts_with("=") => {
                self.advance();
                Some(Token::equal_equal())
            }
            '=' => Some(Token::equal()),
            '!' if self.starts_with("=") => {
                self.advance();
                Some(Token::bang_equal())
            }
            '!' => Some(Token::bang()),
            '>' if self.starts_with("=") => {
                self.advance();
                Some(Token::greater_equal())
            }
            '>' => Some(Token::greater()),
            '<' if self.starts_with("=") => {
                self.advance();
                Some(Token::less_equal())
            }
            '<' => Some(Token::less()),
            '"' => {
                if let Some(pos) = self.source[self.cursor..].find('"') {
                    let end_pos = self.cursor + pos + 1;
                    let s = &self.source[self.cursor - 1..end_pos];
                    self.cursor = end_pos;
                    Some(Token::string_literal(s.to_string()))
                } else {
                    self.cursor = self.source.len();
                    self.result = Err(LexerError::UnterminatedString(self.current_line));
                    eprintln!("[line {}] Error: Unterminated string.", self.current_line);
                    self.done = true;
                    Some(Token::EOF)
                }
            }
            ch if ch.is_whitespace() => {
                if ch == '\n' {
                    self.current_line += 1;
                }
                self.next()
            }
            ch if ch.is_ascii_digit() => {
                if let Some(pos) = self.source[self.cursor..].find(|c| !is_number(c)) {
                    let end_pos = self.cursor + pos;
                    let s = &self.source[self.cursor - 1..end_pos];
                    self.cursor = end_pos;
                    Some(Token::number_literal(s.to_string()))
                } else {
                    let s = &self.source[self.cursor - 1..];
                    self.cursor = self.source.len();
                    Some(Token::number_literal(s.to_string()))
                }
            }

            ch if is_alpha(ch) => {
                if let Some(pos) =
                    self.source[self.cursor..].find(|c: char| !is_alpha(c) && !c.is_alphanumeric())
                {
                    let end_pos = self.cursor + pos;
                    let s = &self.source[self.cursor - 1..end_pos];
                    self.cursor = end_pos;
                    if RESERVED_WORDS.contains(&s) {
                        Some(Token::reserved(s.to_string()))
                    } else {
                        Some(Token::identifier(s.to_string()))
                    }
                } else {
                    let s = &self.source[self.cursor - 1..];
                    self.cursor = self.source.len();
                    if RESERVED_WORDS.contains(&s) {
                        Some(Token::reserved(s.to_string()))
                    } else {
                        Some(Token::identifier(s.to_string()))
                    }
                }
            }
            _ => {
                self.result = Err(LexerError::UnexpectedCharacter {
                    line: self.current_line,
                    ch,
                });
                eprintln!(
                    "[line {}] Error: Unexpected character: {}",
                    self.current_line, ch
                );
                None
            }
        }
    }
}
