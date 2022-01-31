use std::collections::{HashMap, HashSet};
use nom::branch::alt;
use nom::{IResult, Parser};
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::{alphanumeric1, multispace1, multispace0, digit1, digit0, char};
use nom::character::{is_alphabetic, is_alphanumeric, is_digit};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, pair, preceded, tuple};
use std::str::FromStr;
use nom::combinator::{map, map_opt, opt};
use nom::error::ErrorKind::Tag;
use nom::number::complete::float;

use crate::ast::{ConstantValue, FunctionDef, Module, TypeInfo};

pub fn parse(input: &str) -> IResult<&str, Module> {
    enum ModuleItem {
        F((String, FunctionDef)),
        C((String, ConstantValue)),
    }
    ;
    let (input, items) = many0(alt(
        (parse_constant_def.map(|c| ModuleItem::C(c)),
         parse_function_def.map(|f| ModuleItem::F(f)))))(input)?;
    let mut functions = HashMap::new();
    let mut constants = HashMap::new();
    for item in items {
        //TODO: write info when override
        match item {
            ModuleItem::F((name, definition)) => functions.insert(name, definition).is_some(),
            ModuleItem::C((name, value)) => constants.insert(name, value).is_some()
        };
    };
    Ok((input, Module { constants, functions }))
}

fn parse_function_def(input: &str) -> IResult<&str, (String, FunctionDef)> {
    let (input, _) = tuple((multispace0, tag("fn ")))(input)?;
    let (input, name) = parse_name(input)?;
    let (input, _) = tuple((multispace0, tag("(")))(input)?;
    let (input, parameters) = separated_list0(tag(","), delimited(multispace0, pair(parse_name, parse_type_info), multispace0))(input)?;
    let (input, _) = tuple((multispace0, tag(")"), multispace0))(input)?;
    let (input, return_type) = opt(parse_type_info)(input)?;
    let (input, _) = pair(multispace0, tag(";"))(input)?;

    Ok((input, (name, FunctionDef { parameters, return_type })))
}

fn parse_constant_def(input: &str) -> IResult<&str, (String, ConstantValue)> {
    let (input, _) = tuple((multispace0, tag("const"), multispace1))(input)?;
    let (input, name) = parse_name(input)?;
    let (input, _) = tuple((multispace0, tag("=")))(input)?;
    let (input, value) = parse_constant_value(input)?;
    let (input, _) = tuple((multispace0, tag(";")))(input)?;

    Ok((input, (name, value)))
}

fn parse_type_info(input: &str) -> IResult<&str, TypeInfo> {
    let (input, _) = multispace0(input)?;
    let (input, type_name) = preceded(pair(tag(":"), multispace0), parse_name)(input)?;
    let type_info = type_name.into();
    Ok((input, type_info))
}

fn parse_constant_value(input: &str) -> IResult<&str, ConstantValue> {
    let (input, _) = multispace0(input)?;
    let (input, neg) = opt(tag("-"))(input)?;
    let (input, integer_value) = map(digit1, |v| i32::from_str(v).unwrap())(input)?;
    let (input, end) = opt(float)(input)?;

    let sign = neg.map_or(1, |_| -1);

    let value = match end {
        Some(end) => ConstantValue::Float((end + integer_value as f32) * sign as f32),
        None => ConstantValue::Integer(integer_value * sign)
    };
    Ok((input, value))
}

fn parse_name(input: &str) -> IResult<&str, String> {
    let (input, _) = multispace0(input)?;
    let (input, (name_0, name_1)) = pair(take_while1(char::is_alphabetic), take_while(|c: char| c.is_alphanumeric() || c == '_'))(input)?;
    let mut name = String::from(name_0);
    name.push_str(name_1);

    Ok((input, name))
}