use std::collections::HashSet;
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alpha1, multispace0, multispace1};
use nom::combinator::{opt, peek};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, tuple};

use crate::ast::{Module, VariableDef, VariableName};
use crate::ast::states::Unverified;
use crate::ast::types::{FuncType, TypeInfo, VariableType};
use crate::parser::expr::parse_expr;
use crate::parser::utils::separated_list0_with_spaces;

mod constant;
pub(crate) mod expr;
mod utils;

pub fn parse_module(input: &str) -> IResult<&str, Module<Unverified>> {
/*    enum ModuleItem{
        Var(VariableName),
        Str(StructDef)
    }*/
    let (input, variables) = many0(parse_variable_def)(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, Module { variables, structs:HashSet::new(), _state: Default::default() }))
}

fn parse_variable_def(input: &str) -> IResult<&str, VariableName> {
    let (input, _) = tuple((multispace0, tag("let"), multispace1))(input)?;
    let (input, mutable) = opt(tag("mut "))(input)?;
    let (input, name) = parse_name(input)?;
    let (input, type_info) = opt(preceded(pair(multispace0, tag(":")), parse_type_info))(input)?;
    let (input, _) = tuple((multispace0, tag("=")))(input)?;
    let (input, expr) = parse_expr(input)?;
    let variable = VariableDef { value: expr, v_type: VariableType { mutable: mutable.is_some(), info: type_info.unwrap_or(TypeInfo::Unknown) } };

    Ok((input, VariableName { name, variable }))
}

pub(crate) fn parse_variable_type(input: &str) -> IResult<&str, VariableType> {
    let (input, m) = preceded(multispace0, opt(tag("mut ")))(input)?;
    let (input, info) = preceded(multispace0, parse_type_info)(input)?;
    Ok((input, VariableType { mutable: m.is_some(), info }))
}

fn parse_type_info(input: &str) -> IResult<&str, TypeInfo> {
    let (input, _) = multispace0(input)?;
    alt((parse_name.map(|name| TypeInfo::Struct(name)), parse_func_type, parse_tuple_type))(input)
}

fn parse_func_type(input: &str) -> IResult<&str, TypeInfo> {
    let (input, params) = delimited(tag("<"), separated_list0_with_spaces(tag(","), parse_variable_type), tag(">"))(input)?;
    let (input, ret) = preceded(pair(multispace0, tag(":")), parse_variable_type)(input)?;
    //let params = params.iter().map(|(m,ti)|VariableType{ mutable: m.is_some(), info: (*ti).clone() }).collect();
    Ok((input, TypeInfo::Function(FuncType{params, return_type: Box::new(ret) })))
}

fn parse_tuple_type(input: &str)->IResult<&str,TypeInfo>{
    let(input,m) = delimited(tag("("),separated_list0_with_spaces(tag(","),parse_type_info ), tag(")"))(input)?;
    Ok((input,TypeInfo::Tuple(m)))
}


pub(crate) fn parse_name(input: &str) -> IResult<&str, String> {
    let (input, _) = multispace0(input)?;
    let (input, _) =peek(alt((alpha1, tag("_"))))(input)?;
    let (input, name) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;

    Ok((input, name.to_string()))
}

/*pub fn parse_struct(input: &str) -> IResult<&str, String>{
    let (input, _) = tuple((multispace0, tag("def"), multispace1))(input)?;
    let (input, m) =

}*/
