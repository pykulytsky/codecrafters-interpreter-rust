use crate::lexer::{tokens::RESERVED_WORDS, LexerError, LexerResult, Token};

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
                '(' => self.tokens.push(Token::left_paren()),
                ')' => self.tokens.push(Token::right_paren()),
                '{' => self.tokens.push(Token::left_brace()),
                '}' => self.tokens.push(Token::right_brace()),
                '*' => self.tokens.push(Token::star()),
                '.' => self.tokens.push(Token::dot()),
                ',' => self.tokens.push(Token::comma()),
                '+' => self.tokens.push(Token::plus()),
                '-' => self.tokens.push(Token::minus()),
                ';' => self.tokens.push(Token::semicolon()),
                '/' if self.source.starts_with("/") => {
                    if let Some(pos) = self.source.find('\n') {
                        self.source = self.source[pos + 1..].to_string();
                        line += 1;
                    } else {
                        self.source.clear();
                    }
                }
                '/' => self.tokens.push(Token::slash()),
                '=' if self.source.starts_with("=") => {
                    self.source.remove(0);
                    self.tokens.push(Token::equal_equal())
                }
                '=' => self.tokens.push(Token::equal()),
                '!' if self.source.starts_with("=") => {
                    self.source.remove(0);
                    self.tokens.push(Token::bang_equal())
                }
                '!' => self.tokens.push(Token::bang()),

                '>' if self.source.starts_with("=") => {
                    self.source.remove(0);
                    self.tokens.push(Token::greater_equal())
                }
                '>' => self.tokens.push(Token::greater()),
                '<' if self.source.starts_with("=") => {
                    self.source.remove(0);
                    self.tokens.push(Token::less_equal())
                }
                '<' => self.tokens.push(Token::less()),
                '"' => {
                    if let Some(pos) = self.source.find('"') {
                        let (s, rest) = self.source.split_at(pos + 1);
                        let mut s = s.to_string();
                        s.insert(0, '"');
                        self.source = rest.to_string();
                        self.tokens.push(Token::string_literal(s));
                    } else {
                        self.source.clear();
                        res = Err(LexerError::UnterminatedString(line));
                        eprintln!("[line {line}] Error: Unterminated string.")
                    }
                }
                ch if ch.is_whitespace() => {
                    if ch == '\n' {
                        line += 1;
                    }
                }
                ch if ch.is_ascii_digit() => {
                    if let Some(pos) = self.source.find(|ch| !is_number(ch)) {
                        let (s, rest) = self.source.split_at(pos);
                        let mut s = s.to_string();
                        s.insert(0, ch);
                        self.source = rest.to_string();
                        self.tokens.push(Token::number_literal(s));
                    } else {
                        let mut s = self.source.clone();
                        s.insert(0, ch);
                        self.tokens.push(Token::number_literal(s));
                        self.source.clear()
                    }
                }
                ch if is_alpha(ch) => {
                    if let Some(pos) = self
                        .source
                        .find(|ch: char| !is_alpha(ch) && !ch.is_alphanumeric())
                    {
                        let (s, rest) = self.source.split_at(pos);
                        let mut s = s.to_string();
                        s.insert(0, ch);
                        self.source = rest.to_string();
                        if RESERVED_WORDS.contains(&s.as_str()) {
                            self.tokens.push(Token::reserved(s));
                        } else {
                            self.tokens.push(Token::identifier(s));
                        }
                    } else {
                        let mut s = self.source.clone();
                        s.insert(0, ch);
                        if RESERVED_WORDS.contains(&s.as_str()) {
                            self.tokens.push(Token::reserved(s));
                        } else {
                            self.tokens.push(Token::identifier(s));
                        }
                        self.source.clear()
                    }
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

fn is_number(ch: char) -> bool {
    ch.is_ascii_digit() || ch == '.'
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
}
