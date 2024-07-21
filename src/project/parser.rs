use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_till1, take_until1},
    character::complete::{char, multispace0, u8},
    combinator::value,
    multi::many0,
    IResult, Parser,
};

use crate::{
    parser_utils::{is_valid_station_name_char, parse_double, ws},
    EastNorthUp,
};

use super::{Datum, Project, Station, SurveyDataFile, UtmLocation};

#[derive(Clone, Debug, PartialEq)]
enum ProjectElement {
    BaseLocation(UtmLocation),
    CarriageReturn,
    Comment(String),
    Datum(Datum),
    LineFeed,
    File(SurveyDataFile),
    PushFolder(String),
    PopFolder,
    UtmZone(u8),
    Whitespace,
}

/// The meaning of the doubles is slightly different depending on the context, so just parse to a tuple
fn parse_triple_double(input: &str) -> IResult<&str, (f64, f64, f64)> {
    let (input, val_0) = parse_double(input)?;
    let (input, _) = char(',')(input)?;
    let (input, val_1) = parse_double(input)?;
    let (input, _) = char(',')(input)?;
    let (input, val_2) = parse_double(input)?;
    Ok((input, (val_0, val_1, val_2)))
}

fn parse_base_location(input: &str) -> IResult<&str, ProjectElement> {
    let (input, _) = char('@')(input)?;
    let (input, (east, north, elevation)) = parse_triple_double(input)?;
    let (input, _) = char(',')(input)?;
    let (input, zone) = u8(input)?;
    let (input, _) = char(',')(input)?;
    let (input, convergence_angle) = parse_double(input)?;
    let (input, _) = char(';')(input)?;
    let east_north_elevation = EastNorthUp::from_meters(east, north, elevation);
    Ok((
        input,
        ProjectElement::BaseLocation(UtmLocation {
            east_north_elevation,
            zone,
            convergence_angle,
        }),
    ))
}

fn parse_comment(input: &str) -> IResult<&str, ProjectElement> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("/")(input)?;
    let (input, comment) = take_till(is_end_of_comment)(input)?;

    Ok((input, ProjectElement::Comment(comment.to_string())))
}

fn parse_datum(input: &str) -> IResult<&str, ProjectElement> {
    let (input, _) = tag("&")(input)?;
    // Kinda dorky, but limited to 21 cases, so combine the with 2 alt blocks
    let (input, datum) = alt((
        alt((
            value(Datum::Adindan, tag("Adindan")),
            value(Datum::Arc1950, tag("Arc 1950")),
            value(Datum::Arc1960, tag("Arc 1960")),
            value(Datum::Australian1966, tag("Australian 1966")),
            value(Datum::Australian1984, tag("Australian 1984")),
            value(Datum::CampAreaAstro, tag("Camp Area Astro")),
            value(Datum::Cape, tag("Cape")),
            value(Datum::European1950, tag("European 1950")),
            value(Datum::European1979, tag("European 1979")),
            value(Datum::Geodetic1949, tag("Geodetic 1949")),
            value(Datum::HongKong1963, tag("HongKong 1963")),
        )),
        alt((
            value(Datum::HuTzuShan, tag("HuTzuShan")),
            value(Datum::Indian, tag("Indian")),
            value(Datum::NorthAmerican1927, tag("North American 1927")),
            value(Datum::NorthAmerican1983, tag("North American 1983")),
            value(Datum::Oman, tag("Oman")),
            value(Datum::OrdinanceSurvey1936, tag("Ordinance Survey 1936")),
            value(Datum::Pulkovo1942, tag("Pulkovo 1942")),
            value(Datum::SouthAmerican1956, tag("South American 1956")),
            value(Datum::SouthAmerican1969, tag("South American 1969")),
            value(Datum::Tokyo, tag("Tokyo")),
            value(Datum::Wgs1972, tag("Wgs 1972")),
            value(Datum::Wgs1984, tag("Wgs 1984")),
        )),
    ))(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, ProjectElement::Datum(datum)))
}

fn is_end_of_comment(c: char) -> bool {
    c == '/' || c == '\n' || c == '\r'
}

fn is_separator(c: char) -> bool {
    c == ','
}

fn is_terminator(c: char) -> bool {
    c == ';'
}

fn parse_station_fix(input: &str) -> IResult<&str, EastNorthUp> {
    let (input, _) = char('[')(input)?;
    // Eat the whitespace before and after the unit tag
    let (input, unit_char) = ws(alt((char('m'), char('M'), char('f'), char('F')))).parse(input)?;
    let (input, _) = char(',')(input)?;
    let (input, (east, north, elevation)) = parse_triple_double(input)?;
    let (input, _) = char(']')(input)?;
    let ene = match unit_char.to_ascii_lowercase() {
        'm' => EastNorthUp::from_meters(east, north, elevation),
        'f' => EastNorthUp::from_feet(east, north, elevation),
        _ => panic!("invalid unit tag"),
    };
    Ok((input, ene))
}

// Each station is a comma separated list of station name and optional fixed location
fn parse_station(input: &str) -> IResult<&str, Station> {
    let (input, _) = char(',')(input)?;
    let (input, _) = many0(parse_comment)(input)?;
    let (input, station_name) = ws(take_till(|c| !is_valid_station_name_char(c))).parse(input)?;
    let station_fixed = parse_station_fix(input);
    if let Ok((input, fix)) = station_fixed {
        Ok((
            input,
            Station {
                name: station_name.to_string(),
                location: Some(fix),
            },
        ))
    } else {
        Ok((
            input,
            Station {
                name: station_name.to_string(),
                location: None,
            },
        ))
    }
}

fn parse_project_file(input: &str) -> IResult<&str, ProjectElement> {
    let (input, _) = tag("#")(input)?;
    let (input, file_path) =
        ws(take_till1(|c| is_separator(c) || is_terminator(c))).parse(input)?;
    let (input, stations) = many0(parse_station)(input)?;
    let (input, _) = char(';')(input)?;
    Ok((
        input,
        ProjectElement::File(SurveyDataFile {
            file_path: file_path.to_string(),
            fixed_stations: stations,
            survey_data: Vec::new(),
        }),
    ))
}

fn parse_push_folder(input: &str) -> IResult<&str, ProjectElement> {
    let (input, _) = char('[')(input)?;
    let (input, folder_name) = take_until1(";")(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, ProjectElement::PushFolder(folder_name.to_string())))
}

fn parse_pop_folder(input: &str) -> IResult<&str, ProjectElement> {
    let (input, _) = char(']')(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, ProjectElement::PopFolder))
}

fn parse_utm_zone(input: &str) -> IResult<&str, ProjectElement> {
    let (input, _) = tag("$")(input)?;
    let (input, zone) = u8(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, ProjectElement::UtmZone(zone)))
}

fn parse_whitespace(input: &str) -> IResult<&str, ProjectElement> {
    let (input, _) = take_till1(|c: char| !c.is_whitespace())(input)?;
    Ok((input, ProjectElement::Whitespace))
}

fn parse_project_element(input: &str) -> IResult<&str, ProjectElement> {
    alt((
        parse_base_location,
        value(ProjectElement::CarriageReturn, char('\r')),
        parse_comment,
        parse_datum,
        value(ProjectElement::LineFeed, char('\n')),
        parse_project_file,
        parse_push_folder,
        parse_pop_folder,
        parse_utm_zone,
        parse_whitespace,
    ))(input)
}

pub fn parse_compass_project(input: &str) -> IResult<&str, Project> {
    let mut input = input;
    let mut base_location: Option<UtmLocation> = None;
    let mut datum: Option<Datum> = None;
    let mut survey_data: Vec<SurveyDataFile> = Vec::new();
    let mut folders = Vec::new();

    while let Ok((munched, element)) = parse_project_element(input) {
        input = munched;
        match element {
            ProjectElement::BaseLocation(parsed_base_location) => {
                base_location = Some(parsed_base_location)
            }
            ProjectElement::Datum(parsed_datum) => datum = Some(parsed_datum),
            ProjectElement::File(file_info) => survey_data.push(file_info),
            ProjectElement::PushFolder(folder) => folders.push(folder),
            ProjectElement::PopFolder => _ = folders.pop(),

            _ => (),
        }
    }
    if let (Some(base_location), Some(datum)) = (base_location, datum) {
        Ok((
            input,
            Project {
                base_location,
                datum,
                survey_data,
                utm_zone: None,
            },
        ))
    } else {
        Err(nom::Err::Incomplete(nom::Needed::Unknown))
    }
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use super::*;
    #[test]
    fn parse_format_examples() {
        let input = include_str!("../../test_data/project_file_examples");
        let (input, project) = parse_compass_project(input).unwrap();
        assert!(input.is_empty());
        let ene = project.base_location.east_north_elevation;
        assert_float_eq!(ene.east, 398_315.500, rmax <= 0.001);
        assert_float_eq!(ene.north, 4_483_735.300, rmax <= 0.001);
        assert_float_eq!(ene.up, 3_048.000, rmax <= 0.001);
        assert!(project.base_location.zone == 13);
        assert_float_eq!(
            project.base_location.convergence_angle,
            0.780,
            rmax <= 0.001
        );
        assert!(project.datum == Datum::NorthAmerican1983);
        assert!(project.survey_data.len() == 17)
    }

    #[test]
    fn parse_compass_sample_project() {
        let sample_project = include_str!("../../test_data/Fulfords.mak");
        let (_, project) = parse_compass_project(sample_project).unwrap();
        let enu = project.base_location.east_north_elevation;
        assert!(enu.east == 357715.717_f64);
        assert!(enu.north == 4372837.574_f64);
        assert!(enu.up == 3048_f64);
        assert!(project.base_location.zone == 13);
        assert!(project.base_location.convergence_angle == -1.050_f64);
        assert!(project.datum == Datum::NorthAmerican1983);
        assert!(!project.survey_data.is_empty())
    }
}
