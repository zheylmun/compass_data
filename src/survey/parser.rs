use nom::{
    bytes::complete::{tag, take_till1},
    character::complete::{alpha1, multispace0, multispace1},
    error::Error,
    multi::many0,
    sequence::Tuple,
    IResult, Parser,
};

use crate::{
    common_types::Date,
    parser_utils::{parse_double, parse_station_name, parse_uint, recognize_line, ws},
};

use super::{BackSightCorrectionFactors, CorrectionFactors, Parameters, Shot, Survey};

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

fn parse_correction_factors(input: &str) -> IResult<&str, CorrectionFactors> {
    let (input, _) = tag("CORRECTIONS:")(input)?;
    let (input, azimuth) = parse_double(input)?;
    let (input, inclination) = parse_double(input)?;
    let (input, length) = parse_double(input)?;
    Ok((
        input,
        CorrectionFactors {
            azimuth,
            inclination,
            length,
        },
    ))
}

fn parse_backsight_correction_factors(input: &str) -> IResult<&str, BackSightCorrectionFactors> {
    let (input, _) = tag("CORRECTIONS2:")(input)?;
    let (input, azimuth) = parse_double(input)?;
    let (input, inclination) = parse_double(input)?;
    Ok((
        input,
        BackSightCorrectionFactors {
            azimuth,
            inclination,
        },
    ))
}

fn parse_survey_parameters(input: &str) -> IResult<&str, Parameters> {
    let (input, parameter_line) = recognize_line(input)?;
    let (parameter_line, _) = tag("DECLINATION:")(parameter_line)?;
    let (parameter_line, declination) = parse_double(parameter_line)?;
    let (parameter_line, _) = tag("FORMAT:")(parameter_line)?;
    let (parameter_line, _) = multispace0(parameter_line)?;
    let (parameter_line, _) = alpha1(parameter_line)?;
    let (parameter_line, _) = multispace0(parameter_line)?;
    let correction_factor_result = parse_correction_factors(parameter_line);
    let (parameter_line, correction_factors) = match correction_factor_result {
        Ok((input, correction_factors)) => (input, Some(correction_factors)),
        Err(_) => (parameter_line, None),
    };
    let backsight_correction_factor_result = parse_backsight_correction_factors(parameter_line);
    let (_, backsight_correction_factors) = match backsight_correction_factor_result {
        Ok((input, backsight_correction_factors)) => (input, Some(backsight_correction_factors)),
        Err(_) => (parameter_line, None),
    };

    Ok((
        input,
        Parameters {
            declination,
            correction_factors,
            backsight_correction_factors,
        },
    ))
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
    let (input, team) = parse_survey_team(input)?;
    let (input, parameters) = parse_survey_parameters(input)?;
    let (input, _) = gobble_labels(input)?;
    let (input, shots) = many0(parse_shot)(input)?;
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
    many0(parse_survey)(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn parse_example_data() {
        let input = include_str!("../../test_data/Fulford.dat");
        let (_input, surveys) = many0(parse_survey)(input).unwrap();

        for survey in &surveys {
            // We have at least one gap in the serialization (FORMAT), so for now just do a test
            // against a simplified survey
            // Eventually we should be able to just do
            // `assert_str_eq!(survey.serialize(), input)`
            if survey.name == "CL" {
                let perfection = include_str!("../../test_data/fulford_cave_survey.dat").trim();
                assert_str_eq!(
                    survey.serialize().trim().replace("\r\n", "\n"),
                    perfection.trim()
                );

                break;
            }
        }
    }
}
