use std::iter::Peekable;
use crate::frontend::ast::{Expr, Stmt, Program, Property};
use crate::frontend::lexer::{Token, TokenType};

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
      TokenType::Func => self.parse_func_decl(),
      TokenType::Ret => self.parse_return(),
      TokenType::If => self.parse_if_stmt(),
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

    while self.at().value == "+" || self.at().value == "-" || self.at().value == "==" || self.at().value == "!=" || self.at().value == "<" || self.at().value == ">" {
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
    let mut left = self.parse_call_member_expr();

    while self.at().value == "*" || self.at().value == "/" || self.at().value == "%" {
      let op = self.consume().value;
      let right = self.parse_call_member_expr();
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
      },
      TokenType::StringLit => Expr::String { value: self.consume().value },
      _ => panic!("Unexpected token type: {:?}", tk),
    }
  }


  fn parse_var_decl(&mut self) -> Stmt {
    let token_type = self.consume().token_type;
    let var: Stmt = match token_type {
      TokenType::Let => {
        let mutable: bool;
        if self.at().token_type == TokenType::Mut {
          mutable = true;
          self.consume();
        } else {
          mutable = false;
        }
        
        let ident = self.consume_expected(TokenType::Ident, "Expected a Identifier.");
        if self.at().token_type == TokenType::Semi {
          self.consume();
          Stmt::VarDecl {
            mutable,
            name: ident.value,
            value: None
          }
        } else {
          self.consume_expected(TokenType::Eq, "Expected a '='");
          let expr = self.parse_expr();
          self.consume_expected(TokenType::Semi, "Expected a ';'");
          Stmt::VarDecl {
            mutable,
            name: ident.value,
            value: Some(expr)
          }
        }
      },
      TokenType::Const => {
        let ident = self.consume_expected(TokenType::Ident, "Expected a Identifier.");
        self.consume_expected(TokenType::Eq, "Expected a '='");
        let decl = Stmt::VarDecl {
          mutable: false,
          name: ident.value,
          value: Some(self.parse_expr())
        };
        self.consume_expected(TokenType::Semi, "Expected a ';'");
        decl
      },
      _ => panic!("Unexpected token type: {:?}", token_type)
    };
    var
  }

  fn parse_func_decl(&mut self) -> Stmt {
    self.consume();
    let name = self.consume_expected(TokenType::Ident, "Expected a Identifier as function name.").value;
    let params: Vec<String> = self.parse_args().iter().map(|arg| {
      match arg {
        Expr::Ident { symbol } => symbol.clone(),
        _ => panic!("Expected an identifier as function argument")
      }
    }).collect();
    let body = self.parse_block();
    let func = Stmt::FuncDecl {
      params,
      name,
      body
    };
    func
  }

  fn parse_return(&mut self) -> Stmt {
    self.consume();
    if self.at().token_type == TokenType::Semi {
      self.consume();
      return Stmt::Return { value: None }
    }
    let value = self.parse_expr();
    self.consume_expected(TokenType::Semi, "Expected a ';'");
    Stmt::Return { value: Some(value) }
  }

  fn parse_call_member_expr(&mut self) -> Expr {
    let member = self.parse_member_expr();

    if self.at().token_type == TokenType::OpenParen {
      self.parse_call_expr(member)
    } else {
      member
    }
  }

  fn parse_call_expr(&mut self, callee: Expr) -> Expr {
    let mut call_expr = Expr::Call {
      callee: Box::new(callee),
      args: self.parse_args()
    };
    
    if self.at().token_type == TokenType::OpenParen {
      call_expr = self.parse_call_expr(call_expr)
    }
    call_expr
  }

  fn parse_args(&mut self) -> Vec<Expr> {
    let mut args = Vec::new();
    self.consume_expected(TokenType::OpenParen, "Expected a '('");
    while self.at().token_type != TokenType::CloseParen {
      args.push(self.parse_expr());
      if self.at().token_type != TokenType::CloseParen {
        self.consume_expected(TokenType::Comma, "Expected a ','");
      }
    }
    self.consume_expected(TokenType::CloseParen, "Expected a ')'");
    args
  }

  fn parse_member_expr(&mut self) -> Expr {
    let mut object = self.parse_primary_expr();

    while self.at().token_type == TokenType::MemAccess || self.at().token_type == TokenType::OpenBracket {
      let operator = self.consume();
      let property: Expr;
      let computed: bool;

      if operator.token_type == TokenType::MemAccess {
        computed = false;
        property = self.parse_primary_expr();

        if !matches!(property, Expr::Ident { .. }) {
          panic!("Expected an identifier after '.'");
        }
      } else {
        computed = true;
        property = self.parse_expr();
        self.consume_expected(TokenType::CloseBracket, "Expected a ']'");
      }

      object = Expr::Member {
        object: Box::new(object),
        property: Box::new(property),
        computed
      }
    }
    object
  }

  fn parse_if_stmt(&mut self) -> Stmt {
    self.consume();
    let condition = self.parse_expr();
    let body = self.parse_block();

    let mut else_body: Option<Vec<Stmt>> = None;
    if self.at().token_type == TokenType::Else {
      self.consume();
      else_body = Some(self.parse_block());
    }
    Stmt::If {
      condition,
      then_branch: body,
      else_branch: else_body
    }
  }

  fn parse_block(&mut self) -> Vec<Stmt> {
    self.consume_expected(TokenType::OpenBrace, "Expected a '{' to open block.");
    let mut body: Vec<Stmt> = Vec::new();

    while self.not_eof() && self.at().token_type != TokenType::CloseBrace {
      body.push(self.parse_stmt());
    }
    self.consume_expected(TokenType::CloseBrace, "Expected a '}' to close block.");
    body
  }
}


