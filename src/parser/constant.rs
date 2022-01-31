use std::str::FromStr;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while};
use nom::character::complete::{anychar, digit1, multispace0, multispace1};
use nom::combinator::{map, opt};
use nom::IResult;
use nom::number::complete::float;
use nom::sequence::{delimited, tuple};
use crate::ast::ConstantValue;
use crate::parser::parse_name;

pub(crate) fn parse_constant_def(input: &str) -> IResult<&str, (String, ConstantValue)> {
    let (input, _) = tuple((multispace0, tag("const"), multispace1))(input)?;
    let (input, name) = parse_name(input)?;
    let (input, _) = tuple((multispace0, tag("=")))(input)?;
    let (input, value) = parse_constant_value(input)?;
    let (input, _) = tuple((multispace0, tag(";")))(input)?;

    Ok((input, (name, value)))
}

fn parse_constant_value(input: &str) -> IResult<&str, ConstantValue> {
    delimited(multispace0, alt((parse_string_constant,parse_number_constant)), multispace0)(input)


}

fn parse_string_constant(input: &str) -> IResult<&str, ConstantValue>{
    let (input,s) = delimited(tag("\""), is_not("\""), tag("\""))(input)?;
    Ok((input,ConstantValue::String(s.to_string())))
}

fn parse_number_constant(input: &str) -> IResult<&str, ConstantValue>{
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