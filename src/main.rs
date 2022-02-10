use std::fs;
use berus_lang::ast::Module;
use berus_lang::parser::{parse_module};

fn main() {
    let code_str = fs::read_to_string("code.txt").expect("cannot open file");
    let m: Module = code_str.parse().expect("cannot parse");
    println!("****", );
    println!("{}", m)
}
