use std::fs;
use berus_lang::ast::{Module, Unverified, Verified};
use berus_lang::parser::{parse_module};

fn main() {
    let code_str = fs::read_to_string("code.txt").expect("cannot open file");
    let m:Module<Unverified> = code_str.parse().expect("cannot parse");
    print!("{}",m);
    match m.verify() {
        Ok(m) => println!("****\n{}",m ),
        Err(e) => print!("****\nERROR: {}",e)
    }

}
