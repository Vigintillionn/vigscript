use std::iter::Peekable;
mod ast;
pub mod lexer;

pub struct Parser<'a> {
  tokens: Peekable<std::slice::Iter<'a, lexer::Token>>,
}

impl<'a> Parser<'a> {
  pub fn new(tokens: &'a [lexer::Token]) -> Parser<'a> {
    Parser { tokens: tokens.iter().peekable() }
  }

  pub fn produce_ast(&mut self) -> ast::Program {
    let mut program: ast::Program = ast::Program { body: Vec::new() };

    while self.not_eof() {
      program.body.push(self.parse_stmt());
    }
    program
  }

  fn at(&mut self) -> &'a lexer::Token {
    self.tokens.peek().unwrap()
  }

  fn consume(&mut self) -> lexer::Token {
    self.tokens.next().unwrap().clone()
  }

  fn consume_expected(&mut self, expected: lexer::TokenType, msg: &str) -> lexer::Token {
    let tk = self.at();
    if tk.token_type != expected {
      panic!("Parse Error\n{}: {:?}", msg, tk);
    }
    self.consume();
    tk.clone()
  }

  fn not_eof(&mut self) -> bool {
    self.at().token_type != lexer::TokenType::EOF
  }

  fn parse_stmt(&mut self) -> ast::Stmt {
    let expr = self.parse_expr();
    ast::Stmt::Expr(expr)
  }

  fn parse_expr(&mut self) -> ast::Expr {
    self.parse_additive_expr()
  }

  // Left hand prescedende -> 10 + 5 - 5 = (10 + 5) - 5
  fn parse_additive_expr(&mut self) -> ast::Expr {
    let mut left = self.parse_multiplicitive_expr();

    while self.at().value == "+" || self.at().value == "-" {
      let op = self.consume().value;
      let right = self.parse_multiplicitive_expr();
      left = ast::Expr::BinExp { 
        left: Box::new(left.clone()), 
        op, 
        right: Box::new(right.clone()) 
      };
    }
    left
  }

  fn parse_multiplicitive_expr(&mut self) -> ast::Expr {
    let mut left = self.parse_primary_expr();

    while self.at().value == "*" || self.at().value == "/" || self.at().value == "%" {
      let op = self.consume().value;
      let right = self.parse_primary_expr();
      left = ast::Expr::BinExp { 
        left: Box::new(left.clone()), 
        op, 
        right: Box::new(right.clone()) 
      };
    }
    left
  }

  // order of prescedence
  // AdditiveExpr
  // MultExpr
  // PrimaryExpr

  fn parse_primary_expr(&mut self) -> ast::Expr {
    let tk = self.at().token_type.clone();
    match tk {
      lexer::TokenType::Ident => ast::Expr::Ident { symbol: self.consume().value },
      lexer::TokenType::IntLit => ast::Expr::IntLit { value: self.consume().value.parse::<i32>().unwrap() },
      lexer::TokenType::OpenParen => {
        self.consume(); // consume the open paren
        let expr = self.parse_expr();
        self.consume_expected(lexer::TokenType::CloseParen, "Unexpected token found inside parenthesised expression. Expected closing parenthesis, but got"); // consume the close paren
        expr
      }
      _ => panic!("Unexpected token type: {:?}", tk),
    }
  }
}


