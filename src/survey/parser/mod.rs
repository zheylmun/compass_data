mod survey_parameters;

use chrono::NaiveDate;
use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_till1},
    character::complete::{multispace0, multispace1},
    error::Error,
    multi::many0,
};

use crate::{
    common_types::Date,
    parser_utils::{parse_double, parse_station_name, parse_uint, recognize_line, ws},
    survey::parser::survey_parameters::parse_survey_parameters,
};

use super::{Shot, Survey};

fn parse_cave_name(input: &str) -> IResult<&str, String> {
    let (input, cave_name) = recognize_line(input)?;
    let (cave_name, _) = multispace0(cave_name)?;
    Ok((input, cave_name.to_string()))
}

fn parse_survey_name(input: &str) -> IResult<&str, String> {
    let (input, survey_line) = recognize_line(input)?;
    let (name, _) = tag("SURVEY NAME:")(survey_line)?;
    let (_, name) = parse_station_name(name)?;

    Ok((input, name.to_string()))
}

fn parse_survey_date_line(input: &str) -> IResult<&str, (Date, Option<String>)> {
    let (input, date_line) = recognize_line(input)?;
    let (date_line, _) = tag("SURVEY DATE:")(date_line)?;
    let (date_line, month) = parse_uint(date_line)?;
    let (date_line, day) = parse_uint(date_line)?;
    let (date_line, year) = parse_uint(date_line)?;
    let comment = match tag::<&str, &str, Error<&str>>("COMMENT:")(date_line) {
        Ok((comment, _)) => Some(comment.to_string()),
        Err(_unused) => None,
    };
    #[allow(clippy::cast_possible_truncation)]
    let date = Date {
        month: month as u8,
        day: day as u8,
        year: year as u16,
    };
    Ok((input, (date, comment)))
}

fn parse_survey_team(input: &str) -> IResult<&str, String> {
    let (input, _) = (tag("SURVEY TEAM:"), multispace1).parse(input)?;
    let (input, team_line) = recognize_line(input)?;
    Ok((input, team_line.to_string()))
}

fn gobble_labels(input: &str) -> IResult<&str, &str> {
    let (input, _) = ws(tag("FROM")).parse(input)?;
    let (input, _) = ws(take_till1(|c| c == '\n')).parse(input)?;
    Ok((input, ""))
}

fn parse_shot(input: &str) -> IResult<&str, Shot> {
    let (input, line) = recognize_line(input)?;
    let (line, from) = parse_station_name(line)?;
    let (line, to) = parse_station_name(line)?;
    let (line, length) = parse_double(line)?;
    let (line, azimuth) = parse_double(line)?;
    let (line, inclination) = parse_double(line)?;
    let (line, left) = parse_double(line)?;
    let (line, up) = parse_double(line)?;
    let (line, down) = parse_double(line)?;
    let (_, right) = parse_double(line)?;
    let shot = Shot {
        from: from.to_string(),
        to: to.to_string(),
        length,
        azimuth,
        inclination,
        up,
        down,
        left,
        right,
        flags: None,
        comment: None,
    };
    Ok((input, shot))
}

pub(crate) fn parse_survey(input: &str) -> IResult<&str, Survey> {
    let (input, cave_name) = parse_cave_name(input)?;
    let (input, name) = parse_survey_name(input)?;
    let (input, (date, comment)) = parse_survey_date_line(input)?;
    let date =
        NaiveDate::from_ymd_opt(i32::from(date.year), u32::from(date.month), u32::from(date.day))
            .unwrap();
    let (input, team) = parse_survey_team(input)?;
    let (input, parameters) = parse_survey_parameters(input)?;
    let (input, _) = gobble_labels(input)?;
    let (input, shots) = many0(parse_shot).parse(input)?;
    let (input, _) = ws(tag("")).parse(input)?;
    Ok((
        input,
        Survey {
            cave_name,
            name,
            date,
            comment,
            team,
            parameters,
            shots,
        },
    ))
}

pub fn parse_dat_file(input: &str) -> IResult<&str, Vec<Survey>> {
    many0(parse_survey).parse(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_example_data() {
        let input = include_str!("../../../test_data/Fulford.dat");
        let (_input, surveys) = many0(parse_survey).parse(input).unwrap();

        // Test round-trip: parse -> serialize -> parse should give same result
        for survey in &surveys {
            let serialized = survey.serialize();
            let (_, reparsed_surveys) = many0(parse_survey).parse(&serialized).unwrap();
            assert_eq!(reparsed_surveys.len(), 1, "Should parse back to one survey");
            assert_eq!(
                &reparsed_surveys[0], survey,
                "Round-trip for survey '{}' failed",
                survey.name
            );
        }
    }

    #[test]
    fn dat_file_round_trip() {
        // Test that we can parse and re-serialize an entire .dat file
        let input = include_str!("../../../test_data/Fulsurf.dat");
        let (_, surveys) = parse_dat_file(input).unwrap();

        for survey in &surveys {
            let serialized = survey.serialize();
            let (_, reparsed) = many0(parse_survey).parse(&serialized).unwrap();
            assert_eq!(reparsed.len(), 1);
            assert_eq!(
                &reparsed[0], survey,
                "Round-trip for survey '{}' failed",
                survey.name
            );
        }
    }

    #[test]
    fn test_parse_dat_file_returns_surveys() {
        // Verify that parse_dat_file actually returns surveys (not an empty vec)
        let input = include_str!("../../../test_data/Fulford.dat");
        let (_, surveys) = parse_dat_file(input).unwrap();
        assert_eq!(surveys.len(), 25, "Should parse all 25 surveys from Fulford.dat");
    }

    #[test]
    fn test_read_to_string_matches_include_str() {
        // Verify that read_to_string produces the same result as include_str
        let include_input = include_str!("../../../test_data/Fulford.dat");
        let read_input = std::fs::read_to_string("test_data/Fulford.dat").unwrap();

        let (_, include_surveys) = parse_dat_file(include_input).unwrap();
        let (_, read_surveys) = parse_dat_file(&read_input).unwrap();

        assert_eq!(
            include_surveys.len(),
            read_surveys.len(),
            "Should parse same number of surveys"
        );
    }

    // Edge case tests
    #[test]
    fn test_parse_survey_no_shots() {
        let input = "Test Cave\n\
                     SURVEY NAME: A\n\
                     SURVEY DATE: 1 15 2024\n\
                     SURVEY TEAM:\n\
                     John Doe\n\
                     DECLINATION: 10.0\n\
                     \n\
                            FROM           TO   LENGTH  BEARING      INC     LEFT       UP     DOWN    RIGHT\n\
                     \n\
                     \x0c\n";
        let (_, survey) = parse_survey(input).unwrap();
        assert_eq!(survey.cave_name, "Test Cave");
        assert_eq!(survey.name, "A");
        assert!(survey.shots.is_empty());
    }

    #[test]
    fn test_parse_survey_with_comment() {
        let input = "Test Cave\n\
                     SURVEY NAME: B\n\
                     SURVEY DATE: 6 29 1987  COMMENT:This is a test comment\n\
                     SURVEY TEAM:\n\
                     Jane Doe\n\
                     DECLINATION: 11.5\n\
                     \n\
                            FROM           TO   LENGTH  BEARING      INC     LEFT       UP     DOWN    RIGHT\n\
                     \n\
                     \x0c\n";
        let (_, survey) = parse_survey(input).unwrap();
        assert_eq!(survey.comment, Some("This is a test comment".to_string()));
    }

    #[test]
    fn test_parse_survey_special_station_names() {
        let input = "Test Cave\n\
                     SURVEY NAME: C\n\
                     SURVEY DATE: 1 1 2000\n\
                     SURVEY TEAM:\n\
                     Team\n\
                     DECLINATION: 0.0\n\
                     \n\
                            FROM           TO   LENGTH  BEARING      INC     LEFT       UP     DOWN    RIGHT\n\
                     \n\
                            A_1          A-2     10.0     90.0      0.0      1.0      2.0      3.0      4.0\n\
                           A'3          A*4     15.0    180.0     -5.0      2.0      3.0      4.0      5.0\n\
                     \x0c\n";
        let (_, survey) = parse_survey(input).unwrap();
        assert_eq!(survey.shots.len(), 2);
        assert_eq!(survey.shots[0].from, "A_1");
        assert_eq!(survey.shots[0].to, "A-2");
        assert_eq!(survey.shots[1].from, "A'3");
        assert_eq!(survey.shots[1].to, "A*4");
    }

    #[test]
    fn test_parse_survey_negative_values() {
        let input = "Test Cave\n\
                     SURVEY NAME: D\n\
                     SURVEY DATE: 12 31 1999\n\
                     SURVEY TEAM:\n\
                     Team\n\
                     DECLINATION: -5.5\n\
                     \n\
                            FROM           TO   LENGTH  BEARING      INC     LEFT       UP     DOWN    RIGHT\n\
                     \n\
                              A1           A2     10.0    270.0    -45.0     -1.0     -2.0     -3.0     -4.0\n\
                     \x0c\n";
        let (_, survey) = parse_survey(input).unwrap();
        assert!((survey.parameters.declination - (-5.5)).abs() < 0.001);
        assert!((survey.shots[0].inclination - (-45.0)).abs() < 0.001);
    }

    #[test]
    fn test_parse_survey_missing_data_marker() {
        // -9999.0 is used to indicate missing LRUD data in Compass files
        let input = "Test Cave\n\
                     SURVEY NAME: E\n\
                     SURVEY DATE: 6 15 2020\n\
                     SURVEY TEAM:\n\
                     Team\n\
                     DECLINATION: 10.0\n\
                     \n\
                            FROM           TO   LENGTH  BEARING      INC     LEFT       UP     DOWN    RIGHT\n\
                     \n\
                              A1           A2     10.0     90.0      0.0  -9999.0  -9999.0  -9999.0  -9999.0\n\
                     \x0c\n";
        let (_, survey) = parse_survey(input).unwrap();
        assert!((survey.shots[0].left - (-9999.0)).abs() < 0.001);
        assert!((survey.shots[0].up - (-9999.0)).abs() < 0.001);
    }

    #[test]
    fn test_parse_survey_crlf_line_endings() {
        let input = "Test Cave\r\n\
                     SURVEY NAME: F\r\n\
                     SURVEY DATE: 3 14 2023\r\n\
                     SURVEY TEAM:\r\n\
                     Team\r\n\
                     DECLINATION: 0.0\r\n\
                     \r\n\
                            FROM           TO   LENGTH  BEARING      INC     LEFT       UP     DOWN    RIGHT\r\n\
                     \r\n\
                     \x0c\r\n";
        let (_, survey) = parse_survey(input).unwrap();
        assert_eq!(survey.cave_name, "Test Cave");
        assert_eq!(survey.name, "F");
    }

    #[test]
    fn test_parse_cave_name() {
        let (remaining, name) = parse_cave_name("My Cave Name\nrest").unwrap();
        assert_eq!(name, "My Cave Name");
        assert_eq!(remaining, "rest");
    }

    #[test]
    fn test_parse_survey_name() {
        let (remaining, name) = parse_survey_name("SURVEY NAME: ABC123\nrest").unwrap();
        assert_eq!(name, "ABC123");
        assert_eq!(remaining, "rest");
    }

    #[test]
    fn test_parse_survey_date_without_comment() {
        let (remaining, (date, comment)) =
            parse_survey_date_line("SURVEY DATE: 6 29 1987\nrest").unwrap();
        assert_eq!(date.month, 6);
        assert_eq!(date.day, 29);
        assert_eq!(date.year, 1987);
        assert!(comment.is_none());
        assert_eq!(remaining, "rest");
    }

    #[test]
    fn test_parse_survey_date_with_comment() {
        let (remaining, (date, comment)) =
            parse_survey_date_line("SURVEY DATE: 12 25 2000  COMMENT:Holiday survey\nrest").unwrap();
        assert_eq!(date.month, 12);
        assert_eq!(date.day, 25);
        assert_eq!(date.year, 2000);
        assert_eq!(comment, Some("Holiday survey".to_string()));
        assert_eq!(remaining, "rest");
    }

    #[test]
    fn test_parse_survey_team() {
        let (remaining, team) = parse_survey_team("SURVEY TEAM:\nAlice,Bob,Charlie\nrest").unwrap();
        assert_eq!(team, "Alice,Bob,Charlie");
        assert_eq!(remaining, "rest");
    }

    #[test]
    fn test_parse_shot() {
        let input = "          A1           A2     21.75    63.50   -28.00      2.60      2.60      2.60      2.60\nrest";
        let (remaining, shot) = parse_shot(input).unwrap();
        assert_eq!(shot.from, "A1");
        assert_eq!(shot.to, "A2");
        assert!((shot.length - 21.75).abs() < 0.001);
        assert!((shot.azimuth - 63.50).abs() < 0.001);
        assert!((shot.inclination - (-28.0)).abs() < 0.001);
        assert_eq!(remaining, "rest");
    }

    #[test]
    fn test_parse_multiple_surveys_in_file() {
        let input = "Cave One\n\
                     SURVEY NAME: A\n\
                     SURVEY DATE: 1 1 2020\n\
                     SURVEY TEAM:\n\
                     Team A\n\
                     DECLINATION: 10.0\n\
                     \n\
                            FROM           TO   LENGTH  BEARING      INC     LEFT       UP     DOWN    RIGHT\n\
                     \n\
                     \x0c\n\
                     Cave Two\n\
                     SURVEY NAME: B\n\
                     SURVEY DATE: 2 2 2021\n\
                     SURVEY TEAM:\n\
                     Team B\n\
                     DECLINATION: 11.0\n\
                     \n\
                            FROM           TO   LENGTH  BEARING      INC     LEFT       UP     DOWN    RIGHT\n\
                     \n\
                     \x0c\n";
        let (_, surveys) = parse_dat_file(input).unwrap();
        assert_eq!(surveys.len(), 2);
        assert_eq!(surveys[0].cave_name, "Cave One");
        assert_eq!(surveys[0].name, "A");
        assert_eq!(surveys[1].cave_name, "Cave Two");
        assert_eq!(surveys[1].name, "B");
    }
}
