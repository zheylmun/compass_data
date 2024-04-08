use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_till1, take_until1},
    character::complete::{char, multispace0, u8},
    combinator::value,
    error::ParseError,
    multi::many0,
    number::complete::double,
    sequence::delimited,
    IResult, Parser,
};

use super::Survey;
fn is_newline_char(c: char) -> bool {
    c == '\n' || c == '\r'
}

fn line_ending(input: &str) -> IResult<&str, &str> {
    alt((tag("\r\n"), tag("\n")))(input)
}

fn parse_cave_name(input: &str) -> IResult<&str, String> {
    let (input, cave_name) = take_till1(is_newline_char)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, cave_name.to_string()))
}

fn parse_survey_name(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("SURVEY NAME:")(input)?;
    let (input, cave_name) = take_till1(is_newline_char)(input)?;
    Ok((input, cave_name.to_string()))
}
fn parse_survey(input: &str) -> IResult<&str, Survey> {
    let (input, cave_name) = parse_cave_name(input)?;
    let (input, survey_name) = parse_survey_name(input)?;
    Ok((
        input,
        Survey {
            cave_name,
            survey_name,
            survey_date: "".to_string(),
            survey_comment: "".to_string(),
            survey_team: "".to_string(),
            declination: 0.0,
            correction_factor_azimuth: 0.0,
            correction_factor_inclination: 0.0,
            correction_factor_length: 0.0,
            back_sight_correction_factor_azimuth: 0.0,
            back_sight_correction_factor_inclination: 0.0,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_example_data() {
        let input = include_str!("../../test_data/fulford.dat");
        let (input, survey) = parse_survey(input).unwrap();
        println!("{} Survey: {}", survey.cave_name, survey.survey_name);
    }
}
