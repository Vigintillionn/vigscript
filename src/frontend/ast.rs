#[derive(Debug, Clone)]
pub enum Stmt {
  Expr(Expr),
  VarDecl {
    mutable: bool,
    name: String,
    value: Option<Expr>
  },
  FuncDecl {
    params: Vec<String>,
    name: String,
    body: Vec<Stmt>
  },
  Return {
    value: Option<Expr>
  },
  If {
    condition: Expr,
    then_branch: Vec<Stmt>,
    else_branch: Option<Vec<Stmt>>
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
  Array {
    elements: Vec<Expr>
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
  },
  String {
    value: String
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