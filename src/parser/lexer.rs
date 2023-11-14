use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
  IntLit,
  Ident,
  Eq,
  OpenParen,
  CloseParen,
  BinOp,
  Let,
  EOF
}

#[derive(Clone, Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub value: String
}

fn create_reserved() -> HashMap<&'static str, TokenType> {
  let mut reserved = HashMap::new();
  reserved.insert("let", TokenType::Let);
  reserved
}

pub fn tokenize(src: &str) -> Vec<Token> {
  let reserved = create_reserved();
  let mut tokens = Vec::new();

  let mut chars = src.chars().peekable();

  while let Some(ch) = chars.next() {
    match ch {
      '(' => tokens.push(Token { token_type: TokenType::OpenParen, value: ch.to_string() }),
      ')' => tokens.push(Token { token_type: TokenType::CloseParen, value: ch.to_string() }),
      '=' => tokens.push(Token { token_type: TokenType::Eq, value: ch.to_string() }),
      '+' | '-' | '*' | '/' | '%' => tokens.push(Token { token_type: TokenType::BinOp, value: ch.to_string() }),
      '0'..='9' => {
        let mut value = ch.to_string();
        while let Some('0'..='9') = chars.peek() {
          value.push(chars.next().unwrap());
        }
        tokens.push(Token { token_type: TokenType::IntLit, value });
      },
      'a'..='z' | 'A'..='Z' => {
        let mut value = ch.to_string();
        while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') = chars.peek() {
          value.push(chars.next().unwrap());
        }
        if let Some(token_type) = reserved.get(&value[..]) {
          tokens.push(Token { token_type: (*token_type).clone(), value });
        } else {
          tokens.push(Token { token_type: TokenType::Ident, value });
        }
      },
      _ => ()
    }
  }
  tokens.push(Token { token_type: TokenType::EOF, value: String::new() });
  tokens
}