use nom::{InputIter, InputLength, IResult, Parser};
use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::multi::separated_list0;
use nom::sequence::delimited;

pub(crate) fn separated_list0_with_spaces<'a, O, O2, E, F, G>(
    sep: G,
    f: F
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
    where
        F: Parser<&'a str, O, E>,
        G: Parser<&'a str, O2, E>,
        E: ParseError<&'a str>,{
    separated_list0(sep, delimited(multispace0, f, multispace0))
    //separated_list0(sep,f)
}