mod constant;
mod expr;

use std::collections::{HashMap};
use nom::branch::alt;
use nom::{IResult, Parser};
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::{multispace0, multispace1};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::combinator::{opt};

use crate::ast::{ConstantValue, FunctionDef, Module, TypeInfo, VariableDef, VariableName};
use crate::parser::expr::parse_expr;

pub fn parse_module(input: &str) -> IResult<&str, Module> {
    let (input, variables) = many0(parse_variable_def)(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, Module { variables }))
}

fn parse_variable_def(input: &str) -> IResult<&str, VariableName> {
    let (input, _) = tuple((multispace0, tag("let"), multispace1))(input)?;
    let (input, mutable) = opt(tag("mut"))(input)?;
    let (input, name) = parse_name(input)?;
    let (input, type_info) = opt(parse_type_info)(input)?;
    let (input, _) = tuple((multispace0, tag("=")))(input)?;
    let (input, expr) = parse_expr(input)?;

    let variable = VariableDef { mutable: mutable.is_some(), value: expr, type_info };

    Ok((input, VariableName { name, variable }))
}

fn parse_type_info(input: &str) -> IResult<&str, TypeInfo> {
    let (input, _) = multispace0(input)?;
    let (input, type_name) = preceded(pair(tag(":"), multispace0), parse_name)(input)?;
    let type_info = type_name.into();
    Ok((input, type_info))
}


fn parse_name(input: &str) -> IResult<&str, String> {
    let (input, _) = multispace0(input)?;
    let (input, (name_0, name_1)) = pair(take_while1(|c: char| c.is_alphabetic() || c == '_'), take_while(|c: char| c.is_alphanumeric() || c == '_'))(input)?;
    let mut name = String::from(name_0);
    name.push_str(name_1);

    Ok((input, name))
}