#![allow(dead_code, unused)]

use crate::{
    lexer::{Lexer, Token, TokenKind, RESERVED_WORDS},
    parser::expr::UnaryKind,
};
pub use expr::Expr;
pub use literal::Literal;

pub mod expr;
pub mod literal;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let tokens = lexer.parse_to_end();
        Self { tokens, cursor: 0 }
    }
    fn peek_token(&self) -> Token {
        // TODO: maybe return reference
        self.tokens[self.cursor..].iter().next().unwrap().clone()
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }
}

impl Iterator for Parser {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.tokens.len() {
            return None;
        }
        let token = self.peek_token();
        self.advance();
        use crate::lexer::TokenKind::*;
        match token.kind {
            NIL => Some(Expr::NIL),
            TRUE => Some(Expr::Literal(Literal::Logical(true))),
            FALSE => Some(Expr::Literal(Literal::Logical(false))),
            NumberLiteral => Some(Expr::Literal(Literal::Number(token.literal?.parse().ok()?))),
            StringLiteral => Some(Expr::Literal(Literal::Str(token.literal?))),
            LeftParen => {
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
            Bang => {
                let operand = self.next()?;
                Some(Expr::Unary(UnaryKind::LogicalNot, Box::new(operand)))
            }
            Minus => {
                let operand = self.next()?;
                Some(Expr::Unary(UnaryKind::Negation, Box::new(operand)))
            }
            Eof => None,
            RightParen => None,
            t => unimplemented!("{:?}", t),
        }
    }
}
