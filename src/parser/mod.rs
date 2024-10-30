#![allow(dead_code, unused)]

use crate::{
    lexer::{Lexer, LexerResult, Token, TokenKind, RESERVED_WORDS},
    parser::{
        error::{ParserError, ParserResult},
        expr::{BinaryKind, UnaryKind},
    },
};
pub use expr::Expr;
pub use literal::Literal;

pub mod error;
pub mod expr;
pub mod literal;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    current_precedence: u8,
    pub result: ParserResult<()>,
}

fn is_binary_op(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual
            | TokenKind::EqualEqual
            | TokenKind::BangEqual
    )
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let tokens = lexer.parse_to_end();
        Self {
            tokens,
            cursor: 0,
            current_precedence: 0,
            result: lexer_to_parser_result(lexer.result),
        }
    }
    fn peek_token(&self) -> Option<Token> {
        // TODO: maybe return reference
        self.tokens[self.cursor..].iter().next().cloned()
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }

    fn get_precedence(&self, kind: &TokenKind) -> u8 {
        match kind {
            TokenKind::Star | TokenKind::Slash => 10,
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual
            | TokenKind::EqualEqual
            | TokenKind::BangEqual => 5,
            _ => 0,
        }
    }

    fn parse_expression(&mut self, precedence: u8) -> Option<Expr> {
        let mut primary = self.parse_primary()?;

        let next_token = self.peek_token()?;
        if is_binary_op(next_token.kind) {
            while let Some(next_token) = self.peek_token() {
                let op_precedence = self.get_precedence(&next_token.kind);

                if op_precedence <= precedence {
                    break;
                }

                self.advance();
                let operator = match next_token.kind {
                    TokenKind::Plus => BinaryKind::Addition,
                    TokenKind::Minus => BinaryKind::Subtraction,
                    TokenKind::Star => BinaryKind::Multiplication,
                    TokenKind::Slash => BinaryKind::Division,
                    TokenKind::Less => BinaryKind::Less,
                    TokenKind::LessEqual => BinaryKind::LessEqual,
                    TokenKind::Greater => BinaryKind::Greater,
                    TokenKind::GreaterEqual => BinaryKind::GreaterEqual,
                    TokenKind::EqualEqual => BinaryKind::Equality,
                    TokenKind::BangEqual => BinaryKind::NotEquality,
                    _ => unreachable!(),
                };

                let right = self.parse_expression(op_precedence);
                if right.is_none() {
                    let lexeme = &self.tokens[self.cursor - 1].lexeme;
                    eprintln!("[line 1] Error at '{}': Expect expression.", lexeme);
                    self.result = Err(ParserError::ExpectedExpression {
                        line: 1,
                        lexeme: lexeme.to_string(),
                    });
                    return None;
                }

                primary = Expr::Binary {
                    op: operator,
                    left: Box::new(primary),
                    right: Box::new(right?),
                };
            }
        }
        if self.result.is_err() {
            return None;
        }

        Some(primary)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        let token = self.peek_token()?;
        self.advance();

        match token.kind {
            TokenKind::NIL => Some(Expr::NIL),
            TokenKind::TRUE => Some(Expr::Literal(Literal::Logical(true))),
            TokenKind::FALSE => Some(Expr::Literal(Literal::Logical(false))),
            TokenKind::NumberLiteral => {
                Some(Expr::Literal(Literal::Number(token.literal?.parse().ok()?)))
            }
            TokenKind::StringLiteral => Some(Expr::Literal(Literal::Str(token.literal?))),
            TokenKind::LeftParen => {
                if let Some(pos) = self.tokens[self.cursor..]
                    .iter()
                    .position(|tk| tk.kind == TokenKind::RightParen)
                {
                    let mut group_tokens = vec![];
                    for expr in self.by_ref() {
                        group_tokens.push(expr);
                    }
                    let group = Expr::Group(group_tokens);
                    Some(group)
                } else {
                    eprintln!("[line 1] Error: Unmatched parens");
                    self.result = Err(ParserError::UnmatchedParens(1));
                    None
                }
            }
            TokenKind::Bang => {
                let operand = self.parse_expression(10)?;
                Some(Expr::Unary(UnaryKind::LogicalNot, Box::new(operand)))
            }
            TokenKind::Minus => {
                let operand = self.parse_expression(10)?;
                Some(Expr::Unary(UnaryKind::Negation, Box::new(operand)))
            }
            TokenKind::Eof => None,
            TokenKind::RightParen => None,
            t => {
                eprintln!("[line 1] Error: Unexpected token");
                self.result = Err(ParserError::UnexpectedToken(1));
                None
            } // t => unimplemented!("{:?}", t),
        }
    }
}

fn lexer_to_parser_result(lexer_result: LexerResult<()>) -> ParserResult<()> {
    Ok(lexer_result?)
}

impl Iterator for Parser {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_expression(0)
    }
}
