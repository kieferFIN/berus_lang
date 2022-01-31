use berus_lang::parser::parse;

fn main() {
    let (s,m) = parse("const e = -0.9; const r=-2;fn rrt(s:int, rt :EE): float;").unwrap();
    println!("{}\n****",s);
    for (name, value) in m.constants{
        println!("{} = {:?}",name, value)
    }
    println!("***");
    for (name, def) in m.functions {
        println!("{} {:?}",name,def)
    }

}
