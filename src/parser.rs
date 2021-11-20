#[allow(unused_imports)]
use super::lalrpop_util;

lalrpop_mod!(pub grammar);
#[allow(unused_imports)]
use super::ast::{Expr, OpCode, CompOpCode};

#[allow(unused_variables)]
pub fn parse(input: &str){
  let result = grammar::ProgramParser::new().parse(input);
  match result {
      Ok(value) => {
        let res = parse_expr(*value);
        match res {
            Node::Num(n) => {
              match n {
                NumType::Float(n) => println!("{}", n),
                NumType::Int(n) => println!("{}", n),
              }
            },
            Node::Bool(b) => {
              if b {
                println!("참");
              }else{
                println!("거짓");
              }
            },
            _ => {},
        };
      },
      Err(e) => println!("{:?}", e),
  }

}

enum Node {
  Num(NumType),
  Bool(bool),
  Null,
}

enum NumType {
  Int(i32),
  Float(f32),
}

#[allow(unused_variables)]
fn parse_expr(expr: Expr) -> Node {
  match expr {
      Expr::Int(n) => Node::Num(NumType::Int(n)),
      Expr::Float(n) => Node::Num(NumType::Float(n)),
      Expr::FnCall(f, a) => Node::Null,
      Expr::CompOp(l, o, r) => parse_comp_op(*l, o, *r),
      Expr::Op(l, o, r) => parse_op(*l, o, *r),
      Expr::Null => Node::Null,
  }
}

fn parse_op(l: Expr, o: OpCode, r: Expr) -> Node {
  let left = parse_expr(l);
  let right = parse_expr(r);
  let (left_value, right_value) = match (left, right) {
    (Node::Num(n1), Node::Num(n2)) => {
      match (n1, n2) {
        (NumType::Float(f1), NumType::Int(i1)) => (f1, i1 as f32),
        (NumType::Int(i1), NumType::Float(f1)) => (i1 as f32, f1),
        (NumType::Float(f1), NumType::Float(f2)) => (f1, f2),
        (NumType::Int(i1), NumType::Int(i2)) => (i1 as f32, i2 as f32),
      }
    },
    _ => panic!("연산은 숫자만 가능합니다."),
  };

  match o {
    OpCode::Add => {
      let result = left_value + right_value;
      if result.fract() == 0.0 {
        Node::Num(NumType::Int(result as i32))
      }else{
        Node::Num(NumType::Float(result))
      }
    },
    OpCode::Sub => {
      let result = left_value - right_value;
      if result.fract() == 0.0 {
        Node::Num(NumType::Int(result as i32))
      }else{
        Node::Num(NumType::Float(result))
      }
    },
    OpCode::Mul => {
      let result = left_value * right_value;
      if result.fract() == 0.0 {
        Node::Num(NumType::Int(result as i32))
      }else{
        Node::Num(NumType::Float(result))
      }
    },
    OpCode::Div => {
      if right_value == 0.0 {
        panic!("0으로 나눌 수 없습니다.")
      }
      let result = left_value / right_value;
      if result.fract() == 0.0 {
        Node::Num(NumType::Int(result as i32))
      }else{
        Node::Num(NumType::Float(result))
      }
    },
  }
}

fn parse_comp_op(l: Expr, o: CompOpCode, r: Expr) -> Node {
  let left = parse_expr(l);
  let right = parse_expr(r);
  let (left_value, right_value) = match (left, right) {
    (Node::Num(n1), Node::Num(n2)) => {
      match (n1, n2) {
        (NumType::Float(f1), NumType::Int(i1)) => (f1, i1 as f32),
        (NumType::Int(i1), NumType::Float(f1)) => (i1 as f32, f1),
        (NumType::Float(f1), NumType::Float(f2)) => (f1, f2),
        (NumType::Int(i1), NumType::Int(i2)) => (i1 as f32, i2 as f32),
      }
    },
    _ => panic!("연산은 숫자만 가능합니다."),
  };

  match o {
    CompOpCode::Eq => Node::Bool(left_value == right_value),
    CompOpCode::Gt => Node::Bool(left_value > right_value),
    CompOpCode::GtE => Node::Bool(left_value >= right_value),
    CompOpCode::Lt => Node::Bool(left_value < right_value),
    CompOpCode::LtE => Node::Bool(left_value <= right_value),
    CompOpCode::NotEq => Node::Bool(left_value != right_value),
  }
}