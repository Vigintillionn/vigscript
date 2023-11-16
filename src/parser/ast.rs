#[derive(Debug, Clone)]
pub enum Stmt {
  Expr(Expr),
  VarDecl {
    muteable: bool,
    name: String,
    value: Option<Expr>
  }
}

#[derive(Debug, Clone)]
pub enum Expr {
  BinExp {
    left: Box<Expr>,
    op: String,
    right: Box<Expr>
  },
  IntLit {
    value: f32
  },
  Ident {
    symbol: String
  },
  Assign {
    assignee: Box<Expr>,
    value: Box<Expr>
  },
  Property(Property),
  ObjectLit {
    properties: Vec<Property>
  },
  Member {
    object: Box<Expr>,
    property: Box<Expr>,
    computed: bool
  },
  Call {
    callee: Box<Expr>,
    args: Vec<Expr>
  }
}

#[derive(Debug)]
pub struct Program {
  pub body: Vec<Stmt>
}

#[derive(Debug, Clone)]
pub struct Property {
  pub key: String,
  pub value: Option<Box<Expr>>
}