#[derive(Debug, Clone)]
pub enum Stmt {
  Expr(Expr)
}

#[derive(Debug, Clone)]
pub enum Expr {
  BinExp {
    left: Box<Expr>,
    op: String,
    right: Box<Expr>
  },
  IntLit {
    value: i32
  },
  Ident {
    symbol: String
  }
}

#[derive(Debug)]
pub struct Program {
  pub body: Vec<Stmt>
}

