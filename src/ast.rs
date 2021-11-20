
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Expr {
  Int(i32),
  Float(f32),
  Op(Box<Expr>, OpCode, Box<Expr>),
  CompOp(Box<Expr>, CompOpCode, Box<Expr>),
  FnCall(String, Vec<Box<Expr>>),
  Null
}


#[derive(Debug, Clone)]
pub enum OpCode {
  Mul,
  Div,
  Add,
  Sub,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum CompOpCode {
  Eq,
  NotEq,
  Lt,
  LtE,
  Gt,
  GtE,
}