pub mod expr;

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use nom::bytes::complete::tag;
use nom::{Finish, Parser};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded};
use nom::combinator::opt;
use crate::ast::expr::{Expr, PartialExpr};
use crate::parser::{parse_module, parse_name};

#[derive(Debug)]
pub struct Module<S:AstState> {
    pub variables: Vec<VariableName>,
    pub _state: std::marker::PhantomData<S>
}

impl FromStr for Module<Unverified> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (unparsed, module) = parse_module(s).finish().map_err(|e| format!("{:?}", e))?;
        if unparsed.len() > 0 {
            Err(format!("unparsed: {}", unparsed))
        } else { Ok(module) }
    }
}

impl FromStr for Module<Verified> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m:Module<Unverified>  = s.parse()?;
        m.verify()
    }
}

impl<S:AstState> Display for Module<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for vd in &self.variables {
            writeln!(f, "{} = {}", vd.name, vd.variable)?
        }
        Ok(())
    }
}

/*#[derive(Debug)]
pub enum TypeInfo {
    //Integer,
    //Float,
    //String,
    Struct(String),
    Enum(String),
    Tuple(Vec<TypeInfo>),
    Function(Vec<TypeInfo>, String),
    None,
}*/

pub(crate) type TypeName = String;

/*impl From<String> for TypeInfo {
    fn from(type_name: String) -> Self {
        match type_name.as_str() {
            "int" => TypeInfo::Integer,
            "float" => TypeInfo::Float,
            "str" => TypeInfo::String,
            name => TypeInfo::Struct(name.to_owned())
        }
    }
}*/

#[derive(Debug)]
pub struct VariableDef {
    pub mutable: bool,
    pub value: Expr,
    pub type_info: Option<TypeName>,
}

impl Display for VariableDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let unknown_string = "Unknown".to_string();
        let m = if self.mutable { "mut" } else { "" };
        let ti = self.type_info.as_ref().unwrap_or(&unknown_string);
        write!(f, "{} {:?} : {}", m, ti, self.value)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum ConstantValue {
    Integer(i32),
    Float(f32),
    String(String),
}

impl ConstantValue {
    pub fn to_type_info(&self) -> TypeName {
        match self {
            ConstantValue::Integer(_) => "Int",
            ConstantValue::Float(_) => "Float",
            ConstantValue::String(_) => "String"
        }.to_string()
    }
}

#[derive(Debug)]
pub struct FunctionDef {
    pub parameters: Vec<(String, TypeName, bool)>,
    pub closure: Vec<(String, bool)>,
    pub return_type: TypeName,
    pub expr: Expr,
}

impl FunctionDef {
    pub fn type_name(&self)->TypeName{
        let mut s = "(".to_string();
        let mut it = self.parameters.iter().map(|(_,tn,m)|(tn,m));
        if let Some(first_param) = it.next(){
            if *first_param.1 {
                s.push_str("mut ");
            }
            s.push_str(&first_param.0);
            for p in it {
                s.push(',');
                if *p.1 {
                    s.push_str("mut ")
                }
                s.push_str(&p.0);
            }
        }
        s.push_str("):");
        s.push_str(&self.return_type);
        s
    }
}
pub(crate) fn funcdef_from_type_name(input: &str) -> Option<(Vec<(TypeName, bool)>,TypeName)>{
    if let Ok((i,(params,ret))) = delimited(tag("("),separated_list0(tag(","), pair(opt(tag("mut ")),parse_name)),tag(")"))
        .and(preceded(tag(":"),parse_name)).parse(input).finish(){
        let params =params.iter().map(|(m,n)|(n.to_owned(),m.is_some())).collect();
        Some((params,ret))
    }else { None }
}

#[derive(Debug)]
pub struct VariableName {
    pub name: String,
    pub variable: VariableDef,
}

pub trait AstState{}
pub struct Unverified;
pub struct Verified;

impl AstState for Unverified {}
impl AstState for Verified {}