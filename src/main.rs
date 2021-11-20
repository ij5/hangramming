#[macro_use]
extern crate lalrpop_util;

mod parser;
mod ast;

use std::io::Write;
#[allow(unused_imports)]
use std::{env, io::Read};
#[allow(unused_imports)]
use std::fs::File;

#[allow(unused_imports)]
use std::io;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() == 1 {
    //     panic!("Please input file name.");
    // }
    // let mut f = File::open(args[1].as_str()).expect("file not found.");
    // let mut input = String::new();
    // f.read_to_string(&mut input).expect("Something went wrong reading this file.");
    
    loop{
        print!(">> ");
        io::stdout().flush().expect("flush failed!");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Can't get input string.");
        if input.trim() == "끄기" || input.trim() == "종료" {
            println!("인터프리터가 종료되었습니다.");
            break;
        }
        parser::parse(input.as_str());
    }
}
