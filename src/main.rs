use std::fs;
use berus_lang::parser::parse;

fn main() {
    let code_str = fs::read_to_string("code.txt").unwrap();
    let (s,m) = parse(code_str.as_str()).unwrap();
    println!("{}\n****",s);
    for (name, value) in m.constants{
        println!("{} = {:?}",name, value)
    }
    println!("***");
    for (name, def) in m.functions {
        println!("{} {:?}",name,def)
    }

}
