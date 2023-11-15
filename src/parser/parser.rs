use std::iter::Peekable;
use crate::parser::ast::{Expr, Stmt, Program, Property};
use crate::parser::lexer::{Token, TokenType};

pub struct Parser<'a> {
  tokens: Peekable<std::slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
  pub fn new(tokens: &'a [Token]) -> Parser<'a> {
    Parser { tokens: tokens.iter().peekable() }
  }

  pub fn produce_ast(&mut self) -> Program {
    let mut program: Program = Program { body: Vec::new() };

    while self.not_eof() {
      program.body.push(self.parse_stmt());
    }
    program
  }

  fn at(&mut self) -> &'a Token {
    self.tokens.peek().unwrap()
  }

  fn consume(&mut self) -> Token {
    self.tokens.next().unwrap().clone()
  }

  fn consume_expected(&mut self, expected: TokenType, msg: &str) -> Token {
    let tk = self.at();
    if tk.token_type != expected {
      panic!("Parse Error\n{}: {:?}", msg, tk);
    }
    self.consume();
    tk.clone()
  }

  fn not_eof(&mut self) -> bool {
    self.at().token_type != TokenType::EOF
  }

  fn parse_stmt(&mut self) -> Stmt {
    match self.at().token_type {
      TokenType::Let => self.parse_var_decl(),
      TokenType::Const => self.parse_var_decl(),
      _ => {
        let expr = self.parse_expr();
        Stmt::Expr(expr)
      }
    }
  }

  fn parse_expr(&mut self) -> Expr {
    self.parse_assignment_expr()
    // self.parse_additive_expr()
  }

  fn parse_assignment_expr(&mut self) -> Expr {
    let left = self.parse_object_expr();
    if self.at().token_type == TokenType::Eq {
      self.consume();
      let value = self.parse_expr();
      Expr::Assign { assignee: Box::new(left), value: Box::new(value) }
    }
    else {
      left
    }
  }

  fn parse_object_expr(&mut self) -> Expr {
    if self.at().token_type != TokenType::OpenBrace {
      return self.parse_additive_expr();
    }

    self.consume(); // consume the open brace
    let mut properties = Vec::new();
    while self.not_eof() && self.at().token_type != TokenType::CloseBrace {
      // { key: value, key2: value }
      // { key }
      let key = self.consume_expected(TokenType::Ident, "Object literal key expected.");
      if self.at().token_type == TokenType::Comma {
        self.consume();
        properties.push(Property { key: key.value, value: None });
        continue;
      }
      else if self.at().token_type == TokenType::CloseBrace {
        properties.push(Property { key: key.value, value: None });
        continue;
      }

      self.consume_expected(TokenType::Colon, "Missing colon following identifier in Object literal.");
      let value = self.parse_expr();
      properties.push(Property { key: key.value, value: Some(Box::new(value)) });

      if self.at().token_type != TokenType::CloseBrace {
        self.consume_expected(TokenType::Comma, "Object literal missing comma or closing brace following property.");
      }
    }

    self.consume_expected(TokenType::CloseBrace, "Object literal missing closing brace.");
    Expr::ObjectLit { properties }
  }

  fn parse_additive_expr(&mut self) -> Expr {
    let mut left = self.parse_multiplicitive_expr();

    while self.at().value == "+" || self.at().value == "-" {
      let op = self.consume().value;
      let right = self.parse_multiplicitive_expr();
      left = Expr::BinExp { 
        left: Box::new(left.clone()), 
        op, 
        right: Box::new(right.clone()) 
      };
    }
    left
  }

  fn parse_multiplicitive_expr(&mut self) -> Expr {
    let mut left = self.parse_primary_expr();

    while self.at().value == "*" || self.at().value == "/" || self.at().value == "%" {
      let op = self.consume().value;
      let right = self.parse_primary_expr();
      left = Expr::BinExp { 
        left: Box::new(left.clone()), 
        op, 
        right: Box::new(right.clone()) 
      };
    }
    left
  }

  fn parse_primary_expr(&mut self) -> Expr {
    let tk = self.at().token_type.clone();
    match tk {
      TokenType::Ident => Expr::Ident { symbol: self.consume().value },
      TokenType::IntLit => Expr::IntLit { value: self.consume().value.parse::<f32>().unwrap() },
      TokenType::OpenParen => {
        self.consume(); // consume the open paren
        let expr = self.parse_expr();
        self.consume_expected(TokenType::CloseParen, "Unexpected token found inside parenthesised expression. Expected closing parenthesis, but got"); // consume the close paren
        expr
      }
      _ => panic!("Unexpected token type: {:?}", tk),
    }
  }


  fn parse_var_decl(&mut self) -> Stmt {
    let is_muteable = self.consume().token_type == TokenType::Let;
    let ident = self.consume_expected(TokenType::Ident, "Expected a Identifier.");

    if self.at().token_type == TokenType::Semi {
      self.consume();
      if !is_muteable {
        panic!("Const declarations must have a value");
      }
      let var = Stmt::VarDecl {
        muteable: is_muteable,
        name: ident.value,
        value: None
      };
      return var
    } 
    self.consume_expected(TokenType::Eq, "Expected a '='");
    let decl = Stmt::VarDecl {
      muteable: is_muteable,
      name: ident.value,
      value: Some(self.parse_expr())
    };
    self.consume_expected(TokenType::Semi, "Expected a ';'");
    decl
  }
}


