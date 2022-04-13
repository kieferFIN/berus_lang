use std::fs;
use std::ops::Deref;
use berus_lang::run_env::{DataObj, RunEnv};
use berus_lang::std_library::StdModBasic;

fn main() {
    let code_str = fs::read_to_string("code.txt").expect("cannot open file");
    let mut run_env:RunEnv<StdModBasic> = RunEnv::new();
    run_env.parse_and_add("Fib".to_string(), &code_str).unwrap();
    run_env.print();
    /*match m.verify(&std_mod) {
        Ok(m) => println!("****\n{}",m ),
        Err(e) => print!("****\nERROR: {}",e)
    }*/
/*    let (t,o) = run_env.run("fib(10);").unwrap();
    match o.borrow().deref() {
        DataObj::Int(v) => println!("type: {}, value: {}",t,v),
        _ => panic!("WRONG TYPE")
    };*/
    match run_env.find_variable("f").unwrap().borrow().deref(){
        DataObj::Int(v) => println!("{}",v),
        _=> panic!("WRONG TYPE")
    }





}
