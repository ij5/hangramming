pub struct VM {
  pub registers: [i32;32],
  pc: usize,
  pub program: Vec<u8>,
  remainder: u32,
  equal_flag: bool,
  heap: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum Op {
  HLT,
  IGL,
  LOAD,
  ADD,
  SUB,
  DIV,
  MUL,
  JMP,
  JMPF,
  EQ,
  NEQ,
  GT,
  LT,
  GTE,
  LTE,
  ALOC,
}

pub struct Instruction {
  op: Op,
}

impl Instruction {
  pub fn new(op: Op) -> Instruction {
    Instruction {
      op: op,
    }
  }
}

impl VM {
  pub fn new() -> VM {
    VM {
      registers: [0;32],
      program: vec![],
      pc: 0,
      remainder: 0,
      equal_flag: false,
      heap: vec![],
    }
  }
  pub fn run_once(&mut self) {
    self.execute_instruction();
  }

  pub fn run(&mut self) {
    let mut is_done = false;
    while !is_done {
      is_done = self.execute_instruction();
    }
  }

  fn execute_instruction(&mut self) -> bool {
    if self.pc >= self.program.len() {
      return false;
    }
    match self.decode_op() {
      Op::HLT => {
        println!("HLT encountered");
        return false;
      }
      Op::IGL => {
        panic!("Illegal instuction encountered.");
      }
      Op::LOAD => {
        let register = self.next_8_bits() as usize;
        let number = self.next_16_bits() as u32;
        self.registers[register] = number as i32;
        
      }
      Op::ADD => {
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register1 + register2;
      }
      Op::SUB => {
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register1 - register2;
      }
      Op::MUL => {
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register1 * register2;
      }
      Op::DIV => {
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register1 / register2;
        self.remainder = (register1 % register2) as u32;
      }
      Op::JMP => {
        let target = self.registers[self.next_8_bits() as usize];
        self.pc = target as usize;
      }
      Op::JMPF => {
        let value = self.registers[self.next_8_bits() as usize] as usize;
        self.pc += value;
      }
      Op::EQ => {
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = register1 == register2;
        self.next_8_bits();
      }
      Op::NEQ => {
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = register1 != register2;
        self.next_8_bits();
      }
      Op::GT => {
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = register1 > register2;
        self.next_8_bits();
      }
      Op::GTE => {
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = register1 >= register2;
        self.next_8_bits();
      }
      Op::LT => {
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = register1 < register2;
        self.next_8_bits();
      }
      Op::LTE => {
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = register1 <= register2;
        self.next_8_bits();
      }
      Op::ALOC => {
        let register = self.next_8_bits() as usize;
        let bytes = self.registers[register];
        let new_end = self.heap.len() as i32 + bytes;
        self.heap.resize(new_end as usize, 0);
      }
    }
    true
  }
  fn next_8_bits(&mut self) -> u8 {
    let result = self.program[self.pc];
    self.pc += 1;
    result
  }
  fn next_16_bits(&mut self) -> u16 {
    let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
    self.pc += 2;
    result
  }
  fn decode_op(&mut self) -> Op {
    let op = Op::from(self.program[self.pc]);
    self.pc += 1;
    op
  }
}

impl From<u8> for Op {
  fn from(v: u8) -> Self {
    match v {
      0 => return Op::HLT,
      _ => return Op::IGL,
    }
  }
}