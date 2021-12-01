use crate::INTP_MODE;
use crate::ast::Stmt;

use std::collections::HashMap;

#[allow(unused_imports)]
use super::lalrpop_util;

lalrpop_mod!(pub grammar);
#[allow(unused_imports)]
use super::ast::{Expr, OpCode, CompOpCode};

#[allow(unused_variables)]
pub fn parse(input: &str){
  let result = grammar::ProgramParser::new().parse(input);
  let mut ctx = Context::new();
  match result {
    Ok(value) => {
      for v in value {
        parse_stmt(&mut ctx, *v);
      }
    },
    Err(e) => println!("{:?}", e),
  }

}

#[derive(Debug)]
enum Node {
  Num(NumType),
  Bool(bool),
  VarDec,
  Null,
}


#[derive(Debug)]
enum NumType {
  Int(i32),
  Float(f32),
}

#[derive(Debug)]
struct Context {
  variables: HashMap<String, Box<Node>>,
}

impl Context {
  fn new() -> Context {
    Context {
      variables: HashMap::new()
    }
  }
}

fn parse_stmt(ctx: &mut Context, stmt: Stmt) -> Node {
  match stmt {
    Stmt::Expr(e) => {
      let result = parse_expr(ctx, *e);
      result
    }
    Stmt::VarDec(s, v) => {
      let result = parse_vardec(ctx, s, v);
      result
    }
  }
}

fn parse_vardec(ctx: &mut Context, s: String, v: Box<Expr>) -> Node {
  let result = parse_expr(ctx, *v);
  ctx.variables.insert(s, Box::new(result));
  Node::VarDec
}

#[allow(unused_variables)]
fn parse_expr(ctx: &mut Context, expr: Expr) -> Node {
  match expr {
      Expr::Int(n) => Node::Num(NumType::Int(n)),
      Expr::Float(n) => Node::Num(NumType::Float(n)),
      Expr::FnCall(f, a) => parse_fn_call(ctx, f, a),
      Expr::CompOp(l, o, r) => parse_comp_op(ctx, *l, o, *r),
      Expr::Op(l, o, r) => parse_op(ctx, *l, o, *r),
      Expr::Var(s) => ctx.variables.get(s),
      Expr::Null => Node::Null,
  }
}

fn parse_fn_call(ctx: &mut Context, s: String, p: Vec<Box<Expr>>) -> Node {
  match s.as_str() {
    "출력" => {
      for i in p {
        let exp = parse_expr(ctx, *i);
        match exp {
          Node::Num(n) => {
            match n {
              NumType::Float(f) => println!("{}", f),
              NumType::Int(a) => println!("{}", a),
            }
          },
          Node::Bool(b) => {
            if b {
              println!("참");
            }else{
              println!("거짓");
            }
          },
          Node::Null => println!("없음"),
          _ => {},
        }
      }
      Node::Null
    },
    _ => {
      unsafe {
        if INTP_MODE {
          println!("알 수 없는 함수 이름임.");
        }else{
          panic!("알 수 없는 함수 이름임.");
        }
      }
      Node::Null
    }
  }
}

fn parse_op(ctx: &mut Context, l: Expr, o: OpCode, r: Expr) -> Node {
  let left = parse_expr(ctx, l);
  let right = parse_expr(ctx, r);
  let (left_value, right_value) = match (left, right) {
    (Node::Num(n1), Node::Num(n2)) => {
      match (n1, n2) {
        (NumType::Float(f1), NumType::Int(i1)) => (f1, i1 as f32),
        (NumType::Int(i1), NumType::Float(f1)) => (i1 as f32, f1),
        (NumType::Float(f1), NumType::Float(f2)) => (f1, f2),
        (NumType::Int(i1), NumType::Int(i2)) => (i1 as f32, i2 as f32),
      }
    },
    _ => {
      unsafe {
        if INTP_MODE {
          println!("연산은 숫자만 가능합니다.");
          return Node::Null;
        }else{
          panic!("연산은 숫자만 가능합니다.");
        }
      }
    },
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
        unsafe {
          if INTP_MODE {
            println!("0으로 나눌 수 없습니다.");
            return Node::Null;
          }else{
            panic!("0으로 나눌 수 없습니다.");
          }
        }
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

fn parse_comp_op(ctx: &mut Context, l: Expr, o: CompOpCode, r: Expr) -> Node {
  let left = parse_expr(ctx, l);
  let right = parse_expr(ctx, r);
  let (left_value, right_value) = match (left, right) {
    (Node::Num(n1), Node::Num(n2)) => {
      match (n1, n2) {
        (NumType::Float(f1), NumType::Int(i1)) => (f1, i1 as f32),
        (NumType::Int(i1), NumType::Float(f1)) => (i1 as f32, f1),
        (NumType::Float(f1), NumType::Float(f2)) => (f1, f2),
        (NumType::Int(i1), NumType::Int(i2)) => (i1 as f32, i2 as f32),
      }
    },
    _ => {
      unsafe {
        if INTP_MODE {
          println!("연산은 숫자만 가능합니다.");
          return Node::Null;
        }else{
          panic!("연산은 숫자만 가능합니다.");
        }
      }
    },
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