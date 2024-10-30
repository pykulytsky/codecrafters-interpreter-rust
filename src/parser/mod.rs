#![allow(dead_code, unused)]

use crate::{
    lexer::{Lexer, Token, TokenKind, RESERVED_WORDS},
    parser::expr::{BinaryKind, UnaryKind},
};
pub use expr::Expr;
pub use literal::Literal;

pub mod expr;
pub mod literal;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    current_precedence: u8,
}

const ARITHMETIC_OPERATIONS: &[TokenKind] = &[
    TokenKind::Plus,
    TokenKind::Minus,
    TokenKind::Star,
    TokenKind::Slash,
];

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let tokens = lexer.parse_to_end();
        Self {
            tokens,
            cursor: 0,
            current_precedence: 0,
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
            TokenKind::Plus | TokenKind::Minus => 5,
            _ => 0,
        }
    }

    fn parse_expression(&mut self, precedence: u8) -> Option<Expr> {
        let mut left = self.parse_primary()?;

        let next_token = self.peek_token()?;
        if ARITHMETIC_OPERATIONS.contains(&next_token.kind) {
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
                    _ => unreachable!(),
                };

                let right = self.parse_expression(op_precedence)?;

                left = Expr::Binary {
                    op: operator,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
        }

        Some(left)
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
                    todo!("Unmatched parens");
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
            t => unimplemented!("{:?}", t),
        }
    }
}

impl Iterator for Parser {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_expression(0)
    }
}
