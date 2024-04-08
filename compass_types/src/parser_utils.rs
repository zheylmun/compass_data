use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    error::ParseError,
    number::complete::double,
    sequence::{delimited, terminated},
    IResult, Parser,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub(crate) fn ws<'a, F, O, E>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    delimited(multispace0, inner, multispace0)
}

pub(crate) fn is_valid_station_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '\'' || c == '*'
}

pub(crate) fn parse_double(input: &str) -> IResult<&str, f64> {
    ws(double).parse(input)
}
