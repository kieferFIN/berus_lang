use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, one_of};
use nom::error::make_error;
use nom::{IResult, Parser};
use nom::combinator::opt;
use nom::multi::{many0, separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use crate::ast::expr::{Expr, FunctionCallExpr, IfExpr, Operand, PartialExpr, VariableExpr};
use crate::ast::{FunctionDef, VariableDef};
use crate::parser::constant::parse_constant_value;
use crate::parser::{parse_name, parse_type_info};

pub(crate) fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, _) = multispace0(input)?;
    let (input, first) = parse_partial_expr(input)?;
    let (input, rest) = many0(pair(parse_operand, parse_partial_expr))(input)?;
    let (input, _) = opt(tuple((multispace0, tag(";"))))(input)?;

    let mut exprs = Vec::new();
    let mut operands = Vec::new();
    exprs.push(first);
    for (op, e) in rest {
        exprs.push(e);
        operands.push(op);
    };
    Ok((input, Expr { exprs, operands }))
}

fn parse_partial_expr(input: &str) -> IResult<&str, PartialExpr> {
    alt((parse_func_call, parse_lambda, parse_variable ))(input)
}

fn parse_variable(input: &str) -> IResult<&str, PartialExpr> {
    let (input, v) = preceded(multispace0, alt((parse_name.map(|name| VariableExpr::Variable(name)), parse_constant_value.map(|c| VariableExpr::Constant(c)))))(input)?;
    Ok((input, PartialExpr::Variable(v)))
}

fn parse_lambda(input: &str) -> IResult<&str, PartialExpr> {
    let (input, _) = multispace0(input)?;
    let (input, parameters) = separated_list0(tag(","), delimited(multispace0, pair(parse_name, parse_type_info), multispace0))(input)?;
    let(input,_) = pair(multispace0, tag("->"))(input)?;
    let (input, expr) = parse_expr(input)?;

    Ok((input, PartialExpr::Lambda(FunctionDef { parameters, expr })))
}

fn parse_if(input: &str) -> IResult<&str, PartialExpr>{
    let(input,_) = pair(multispace0, tag("if"))(input)?;
    let(input,cond_expr) = parse_expr(input)?;
    let (input, main_branch) = parse_expr(input)?;
    let (input, else_branch) = opt(preceded(pair(multispace0, tag("else")),parse_expr))(input)?;
    Ok((input,PartialExpr::If(IfExpr{cond_expr,main_branch,else_branch})))

}

fn parse_func_call(input:&str) -> IResult<&str, PartialExpr>{
    let (input,_) = multispace0(input)?;
    let (input,name) = terminated(parse_name, tag("("))(input)?;
    let (input, params) = separated_list0(tag(","), delimited(multispace0, parse_expr, multispace0))(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input,PartialExpr::FunctionCall(FunctionCallExpr{ name, params })))
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    let (input, op) = preceded(multispace0, one_of("+-"))(input)?;
    let op = match op {
        '+' => Operand::Plus,
        '-' => Operand::Minus,
        _ => unreachable!()
    };
    Ok((input, op))
}

