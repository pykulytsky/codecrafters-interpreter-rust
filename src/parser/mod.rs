#![allow(dead_code, unused)]

use std::collections::BTreeMap;

use crate::{
    lexer::{Lexer, LexerResult, Token, TokenKind, RESERVED_WORDS},
    parser::{
        error::{EvaluationError, EvaluationResult, ParserError, ParserResult},
        expr::{BinaryKind, EvaluationValue, Ident, UnaryKind},
        stmt::Scope,
    },
};
pub use expr::Expr;
pub use literal::Literal;
pub use stmt::Stmt;

pub mod error;
pub mod expr;
pub mod literal;
pub mod stmt;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    current_precedence: u8,
    pub result: ParserResult<()>,
    pub global_scope: Scope,
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
            global_scope: BTreeMap::new(),
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
            TokenKind::Plus | TokenKind::Minus | TokenKind::Less => 5,
            TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual
            | TokenKind::EqualEqual
            | TokenKind::BangEqual => 2,
            _ => 0,
        }
    }

    fn expect_expression_err(&mut self) -> Option<Stmt> {
        self.result = Err(ParserError::ExpectedExpression {
            line: 1,
            lexeme: "print".to_string(),
        });
        // self.result = Err(ParserError::UndefinedVariable("test".to_string()));
        None
    }

    fn expect_print_stmt(&mut self) -> Option<Stmt> {
        let expr = self.parse_expression(0);
        match expr {
            Some(expr) => Some(Stmt::Print(expr)),
            None => self.expect_expression_err(),
        }
    }

    fn expect_assignment_stmt(
        &mut self,
        ident: Ident,
        scope: Option<&mut Scope>,
        outer_scope: &mut Option<&mut Scope>,
        is_decl: bool,
    ) -> Option<Stmt> {
        match self.parse_expression(0) {
            Some(expr) => {
                let ass_expr = expr.evaluate(&self.global_scope);
                if let Err(EvaluationError::UndefinedVariable(var)) = ass_expr {
                    self.result = Err(ParserError::UndefinedVariable(var));
                    return None;
                }
                let ass_expr = ass_expr.ok()?.to_expr();
                if is_decl {
                    if let Some(scope) = scope {
                        scope.insert(ident.to_owned(), ass_expr.clone());
                    } else {
                        self.global_scope.insert(ident.to_owned(), ass_expr.clone());
                    }
                } else if let Some(scope) = scope {
                    match scope.entry(ident.clone()) {
                        std::collections::btree_map::Entry::Vacant(_) => {
                            if let Some(outer_scope) = outer_scope {
                                outer_scope.insert(ident.clone(), ass_expr.clone());
                            }
                        }
                        std::collections::btree_map::Entry::Occupied(mut occupied_entry) => {
                            occupied_entry.insert(ass_expr.clone());
                        }
                    }
                }
                Some(Stmt::Declaration(ident, ass_expr))
            }
            None => {
                self.result = Err(ParserError::UndefinedVariable("test".to_string()));
                None
            }
        }
    }

    fn parse_ident(&mut self) -> Option<Ident> {
        let next_token = self.peek_token()?;
        self.advance();
        match next_token.kind {
            TokenKind::Identifier => Some(Ident(next_token.lexeme.to_string())),
            _ => None,
        }
    }

    pub fn parse_statement(
        &mut self,
        scope: Option<&mut Scope>,
        outer_scope: &mut Option<&mut Scope>,
    ) -> Option<Stmt> {
        let token = self.peek_token()?;
        let stmt = match token.kind {
            TokenKind::PRINT => {
                self.advance();
                self.expect_print_stmt()
            }
            TokenKind::VAR => {
                self.advance();
                let ident = self.parse_ident()?;
                match self.peek_token()?.kind {
                    TokenKind::Equal => {
                        self.advance();
                        self.expect_assignment_stmt(ident, scope, outer_scope, true)
                    }
                    TokenKind::Semicolon => {
                        if let Some(scope) = scope {
                            scope.insert(ident.clone(), Expr::NIL);
                        } else {
                            self.global_scope.insert(ident.clone(), Expr::NIL);
                        }
                        Some(Stmt::Declaration(ident, Expr::NIL))
                    }
                    _ => unreachable!("Handle errors"),
                }
            }
            TokenKind::Identifier => {
                let ident = self.parse_ident()?;
                match self.peek_token()?.kind {
                    TokenKind::Equal => {
                        self.advance();
                        self.expect_assignment_stmt(ident, scope, &mut None, false)
                        // self.global_scope.insert(ident.to_owned(), ass_expr);
                    }
                    t => {
                        self.result = Err(ParserError::UndefinedVariable("test".to_string()));
                        None
                    }
                }
            }
            TokenKind::LeftBrace => {
                self.advance();
                let mut stmts = vec![];
                let mut inner_scope = match scope {
                    Some(outer_scope) => outer_scope.clone(),
                    None => self.global_scope.clone(),
                };
                loop {
                    if let Some(TokenKind::RightBrace) = self.peek_token().map(|t| t.kind) {
                        self.advance();
                        break;
                    }
                    let Some(stmt) = self.parse_statement(Some(&mut inner_scope), outer_scope)
                    else {
                        self.result = Err(ParserError::UnmatchedParens(1));
                        return None;
                    };
                    stmts.push(stmt);
                }
                Some(Stmt::Block(stmts, inner_scope))
            }
            _ => Some(Stmt::Expr(self.parse_expression(0)?)),
        };

        if stmt.is_some() {
            let token = self.peek_token()?;
            if matches!(token.kind, TokenKind::Semicolon) {
                self.advance();
            }
        }
        stmt
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
                        // TODO: handle statement cases
                        if let Stmt::Expr(expr) = expr {
                            group_tokens.push(expr);
                        }
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
            TokenKind::Identifier if self.peek_token()?.kind == TokenKind::Equal => {
                let ident = Ident(token.lexeme.to_string());
                self.advance(); // Equal
                let expr = self.parse_expression(0)?;

                let ass_expr = expr.evaluate(&self.global_scope);
                if let Err(EvaluationError::UndefinedVariable(var)) = ass_expr {
                    self.result = Err(ParserError::UndefinedVariable(var));
                    return None;
                }
                let ass_expr = ass_expr.ok()?.to_expr();
                // self.global_scope.insert(ident.to_owned(), ass_expr);
                Some(Expr::Assignment(ident, Box::new(expr)))
            }
            TokenKind::Identifier => Some(Expr::Ident(Ident(token.lexeme.to_string()))),
            t => {
                dbg!(t);
                eprintln!("[line 1] Error: Unexpected token");
                self.result = Err(ParserError::UnexpectedToken(1));
                None
            } // t => unimplemented!("{:?}", t),
        }
    }

    pub fn run(&mut self, stmt: Stmt) -> EvaluationResult<EvaluationValue> {
        match stmt {
            Stmt::Expr(expr) => {
                let evaluation_result = expr.evaluate(&self.global_scope)?;
                Ok(EvaluationValue::Void)
            }
            Stmt::Print(expr) => {
                println!("{:?}", expr.evaluate(&self.global_scope)?);
                Ok(EvaluationValue::Void)
            }
            Stmt::Declaration(_left, _right) => Ok(EvaluationValue::Void),
            Stmt::Block(block, ref scope) => {
                for stmt in block {
                    stmt.run(scope)?;
                }
                Ok(EvaluationValue::Void)
            } // Stmt::Assignment(_left, _right) => Ok(EvaluationValue::Void),
        }
    }
}

fn lexer_to_parser_result(lexer_result: LexerResult<()>) -> ParserResult<()> {
    Ok(lexer_result?)
}

impl Iterator for Parser {
    type Item = Stmt;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_statement(None, &mut None)
    }
}
