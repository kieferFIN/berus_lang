use std::collections::HashSet;

use nom::{IResult};
use nom::character::complete::{ multispace0};
use nom::multi::many0;

use crate::ast::{Module};
use crate::ast::states::Unverified;
use crate::parser::variable::parse_variable_def;

mod constant;
pub(crate) mod expr;
mod utils;
pub(crate) mod variable;

pub fn parse_module(input: &str) -> IResult<&str, Module<Unverified>> {
    /*    enum ModuleItem{
            Var(VariableName),
            Str(StructDef)
        }*/
    let (input, variables) = many0(parse_variable_def)(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, Module { variables, structs: HashSet::new(), _state: Default::default() }))
}

/*pub fn parse_struct(input: &str) -> IResult<&str, String>{
    let (input, _) = tuple((multispace0, tag("def"), multispace1))(input)?;
    let (input, m) =

}*/
