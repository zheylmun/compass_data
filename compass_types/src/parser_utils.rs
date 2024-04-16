use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, line_ending, multispace0, not_line_ending},
    error::ParseError,
    number::complete::{double, i32},
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

pub(crate) fn parse_station_name(input: &str) -> IResult<&str, &str> {
    let (input, name) = ws(take_while1(|c| is_valid_station_name_char(c))).parse(input)?;
    Ok((input, name))
}

pub(crate) fn parse_uint(input: &str) -> IResult<&str, u32> {
    let (input, digits) = ws(digit1).parse(input)?;
    let num = digits.parse().unwrap();
    Ok((input, num))
}

pub(crate) fn recognize_line(input: &str) -> IResult<&str, &str> {
    let (input, line) = not_line_ending(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, line))
}
