use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, opt};
use nom::IResult;
use nom::number::complete::float;
use nom::sequence::{delimited, preceded};

use crate::ast::ConstantValue;

//TODO: add boolean
pub(crate) fn parse_constant_value(input: &str) -> IResult<&str, ConstantValue> {
    preceded(multispace0, alt((parse_string_constant, parse_number_constant)))(input)
}

fn parse_string_constant(input: &str) -> IResult<&str, ConstantValue> {
    let (input, s) = delimited(tag("\""), is_not("\""), tag("\""))(input)?;
    Ok((input, ConstantValue::String(s.to_string())))
}

fn parse_number_constant(input: &str) -> IResult<&str, ConstantValue> {
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