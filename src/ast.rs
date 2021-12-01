#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
  Int(i32),
  Float(f32),
  Op(Box<Expr>, OpCode, Box<Expr>),
  CompOp(Box<Expr>, CompOpCode, Box<Expr>),
  FnCall(String, Vec<Box<Expr>>),
  Var(String),
  Null
}

#[derive(Debug, Clone)]
pub enum Stmt {
  VarDec(String, Box<Expr>),
  Expr(Box<Expr>),
}


#[derive(Debug, Clone, Copy)]
pub enum OpCode {
  Mul,
  Div,
  Add,
  Sub,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum CompOpCode {
  Eq,
  NotEq,
  Lt,
  LtE,
  Gt,
  GtE,
}