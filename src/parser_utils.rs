use nom::{
    IResult, Parser,
    bytes::complete::take_while1,
    character::complete::{digit1, line_ending, multispace0, not_line_ending},
    error::ParseError,
    number::complete::double,
    sequence::delimited,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
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
    let (input, name) = ws(take_while1(is_valid_station_name_char)).parse(input)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::tag;

    // Tests for is_valid_station_name_char
    #[test]
    fn test_valid_station_name_chars() {
        // Alphanumeric
        assert!(is_valid_station_name_char('A'));
        assert!(is_valid_station_name_char('z'));
        assert!(is_valid_station_name_char('0'));
        assert!(is_valid_station_name_char('9'));

        // Special allowed characters
        assert!(is_valid_station_name_char('_'));
        assert!(is_valid_station_name_char('-'));
        assert!(is_valid_station_name_char('\''));
        assert!(is_valid_station_name_char('*'));
    }

    #[test]
    fn test_invalid_station_name_chars() {
        assert!(!is_valid_station_name_char(' '));
        assert!(!is_valid_station_name_char('\t'));
        assert!(!is_valid_station_name_char('\n'));
        assert!(!is_valid_station_name_char('.'));
        assert!(!is_valid_station_name_char(','));
        assert!(!is_valid_station_name_char('@'));
        assert!(!is_valid_station_name_char('#'));
        assert!(!is_valid_station_name_char('!'));
    }

    // Tests for ws combinator
    #[test]
    fn test_ws_strips_leading_whitespace() {
        let result: IResult<&str, &str> = ws(tag("hello")).parse("   hello");
        assert_eq!(result, Ok(("", "hello")));
    }

    #[test]
    fn test_ws_strips_trailing_whitespace() {
        let result: IResult<&str, &str> = ws(tag("hello")).parse("hello   ");
        assert_eq!(result, Ok(("", "hello")));
    }

    #[test]
    fn test_ws_strips_both_whitespace() {
        let result: IResult<&str, &str> = ws(tag("hello")).parse("  hello  ");
        assert_eq!(result, Ok(("", "hello")));
    }

    #[test]
    fn test_ws_handles_tabs_and_newlines() {
        let result: IResult<&str, &str> = ws(tag("hello")).parse("\t\n hello \t\n");
        assert_eq!(result, Ok(("", "hello")));
    }

    #[test]
    fn test_ws_no_whitespace() {
        let result: IResult<&str, &str> = ws(tag("hello")).parse("hello");
        assert_eq!(result, Ok(("", "hello")));
    }

    // Tests for parse_double
    #[test]
    fn test_parse_double_positive() {
        let (remaining, value) = parse_double("  123.45  ").unwrap();
        assert_eq!(remaining, "");
        assert!((value - 123.45).abs() < f64::EPSILON);
    }

    #[test]
    fn test_parse_double_negative() {
        let (remaining, value) = parse_double("  -99.5  ").unwrap();
        assert_eq!(remaining, "");
        assert!((value - (-99.5)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_parse_double_integer() {
        let (remaining, value) = parse_double("42").unwrap();
        assert_eq!(remaining, "");
        assert!((value - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_parse_double_scientific_notation() {
        let (remaining, value) = parse_double("1.5e2").unwrap();
        assert_eq!(remaining, "");
        assert!((value - 150.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_parse_double_with_remaining() {
        let (remaining, value) = parse_double("  3.14  rest").unwrap();
        assert_eq!(remaining, "rest");
        assert!((value - 3.14).abs() < f64::EPSILON);
    }

    // Tests for parse_station_name
    #[test]
    fn test_parse_station_name_simple() {
        let (remaining, name) = parse_station_name("  A1  ").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(name, "A1");
    }

    #[test]
    fn test_parse_station_name_with_underscore() {
        let (remaining, name) = parse_station_name("STATION_1").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(name, "STATION_1");
    }

    #[test]
    fn test_parse_station_name_with_hyphen() {
        let (remaining, name) = parse_station_name("A-1").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(name, "A-1");
    }

    #[test]
    fn test_parse_station_name_with_apostrophe() {
        let (remaining, name) = parse_station_name("A'1").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(name, "A'1");
    }

    #[test]
    fn test_parse_station_name_with_asterisk() {
        let (remaining, name) = parse_station_name("A*").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(name, "A*");
    }

    #[test]
    fn test_parse_station_name_with_remaining() {
        let (remaining, name) = parse_station_name("  ABC123  next").unwrap();
        assert_eq!(remaining, "next");
        assert_eq!(name, "ABC123");
    }

    // Tests for parse_uint
    #[test]
    fn test_parse_uint_simple() {
        let (remaining, value) = parse_uint("  42  ").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(value, 42);
    }

    #[test]
    fn test_parse_uint_zero() {
        let (remaining, value) = parse_uint("0").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(value, 0);
    }

    #[test]
    fn test_parse_uint_large() {
        let (remaining, value) = parse_uint("4294967295").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(value, u32::MAX);
    }

    #[test]
    fn test_parse_uint_with_remaining() {
        let (remaining, value) = parse_uint("  123  abc").unwrap();
        assert_eq!(remaining, "abc");
        assert_eq!(value, 123);
    }

    // Tests for recognize_line
    #[test]
    fn test_recognize_line_lf() {
        let (remaining, line) = recognize_line("hello world\nrest").unwrap();
        assert_eq!(remaining, "rest");
        assert_eq!(line, "hello world");
    }

    #[test]
    fn test_recognize_line_crlf() {
        let (remaining, line) = recognize_line("hello world\r\nrest").unwrap();
        assert_eq!(remaining, "rest");
        assert_eq!(line, "hello world");
    }

    #[test]
    fn test_recognize_line_empty() {
        let (remaining, line) = recognize_line("\nrest").unwrap();
        assert_eq!(remaining, "rest");
        assert_eq!(line, "");
    }

    #[test]
    fn test_recognize_line_multiple() {
        let input = "line1\nline2\nline3\n";
        let (remaining, line1) = recognize_line(input).unwrap();
        assert_eq!(line1, "line1");
        let (remaining, line2) = recognize_line(remaining).unwrap();
        assert_eq!(line2, "line2");
        let (remaining, line3) = recognize_line(remaining).unwrap();
        assert_eq!(line3, "line3");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_recognize_line_no_newline_fails() {
        let result = recognize_line("no newline");
        assert!(result.is_err());
    }
}
