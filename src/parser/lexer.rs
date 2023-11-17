use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
  IntLit,
  Ident,
  Eq,
  Semi,
  OpenParen, // (
  CloseParen, // )
  OpenBrace, // {
  CloseBrace, // }
  OpenBracket, // [
  CloseBracket, // ]
  Func,
  If,
  Else,
  Comma,
  Dot,
  MemAccess,
  Colon,
  StringLit,
  BinOp,
  Let,
  Const,
  Mut,
  Ret,
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
  reserved.insert("const", TokenType::Const);
  reserved.insert("func", TokenType::Func);
  reserved.insert("ret", TokenType::Ret);
  reserved.insert("mut", TokenType::Mut);
  reserved.insert("if", TokenType::If);
  reserved.insert("else", TokenType::Else);
  reserved
}

pub fn tokenize(src: &str) -> Vec<Token> {
  let reserved = create_reserved();
  let mut tokens = Vec::new();

  let mut chars = src.chars().peekable();

  while let Some(ch) = chars.next() {


    match ch {
      '"' => {
        let mut buffer = String::new();
        while chars.peek() != Some(&'"') {
          buffer.push(chars.next().unwrap());
        }
        if chars.peek() != Some(&'"') {
          panic!("Unterminated string literal");
        }
        chars.next();
        tokens.push(Token { token_type: TokenType::StringLit, value: buffer });
      },
      '(' => tokens.push(Token { token_type: TokenType::OpenParen, value: ch.to_string() }),
      ')' => tokens.push(Token { token_type: TokenType::CloseParen, value: ch.to_string() }),
      '{' => tokens.push(Token { token_type: TokenType::OpenBrace, value: ch.to_string() }),
      '}' => tokens.push(Token { token_type: TokenType::CloseBrace, value: ch.to_string() }),
      '[' => tokens.push(Token { token_type: TokenType::OpenBracket, value: ch.to_string() }),
      ']' => tokens.push(Token { token_type: TokenType::CloseBracket, value: ch.to_string() }),
      ':' => {
        if let Some(':') = chars.peek() {
          chars.next();
          tokens.push(Token { token_type: TokenType::MemAccess, value: "::".to_string() });
        } else {
          tokens.push(Token { token_type: TokenType::Colon, value: ch.to_string() });
        }
      }
      ',' => tokens.push(Token { token_type: TokenType::Comma, value: ch.to_string() }),
      '.' => tokens.push(Token { token_type: TokenType::Dot, value: ch.to_string() }),
      '=' => tokens.push(Token { token_type: TokenType::Eq, value: ch.to_string() }),
      ';' => tokens.push(Token { token_type: TokenType::Semi, value: ch.to_string() }),
      '+' | '-' | '*' | '/' | '%' => tokens.push(Token { token_type: TokenType::BinOp, value: ch.to_string() }),
      '0' ..= '9' => {
        let mut value = ch.to_string();
        while let Some('0'..='9') = chars.peek() {
          value.push(chars.next().unwrap());
        }
        tokens.push(Token { token_type: TokenType::IntLit, value });
      },
      'a' ..= 'z' | 'A' ..= 'Z' => {
        let mut value = ch.to_string();
        while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '!') = chars.peek() {
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