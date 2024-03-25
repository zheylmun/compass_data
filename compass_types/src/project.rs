use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{char, u8},
    combinator::value,
    number::complete::double,
    IResult,
};

use crate::FEET_TO_METERS;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EastNorthElevation {
    east: f64,
    north: f64,
    elevation: f64,
}

impl EastNorthElevation {
    pub fn from_meters(east: f64, north: f64, elevation: f64) -> Self {
        Self {
            east,
            north,
            elevation,
        }
    }
    pub fn from_feet(east: f64, north: f64, elevation: f64) -> Self {
        Self {
            east: east * FEET_TO_METERS,
            north: north * FEET_TO_METERS,
            elevation: elevation * FEET_TO_METERS,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UtmLocation {
    east_north_elevation: EastNorthElevation,
    zone: u8,
    convergence_angle: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Datum {
    Adindan,
    Arc1950,
    Arc1960,
    Australian1966,
    Australian1984,
    CampAreaAstro,
    Cape,
    European1950,
    European1979,
    Geodetic1949,
    HongKong1963,
    HuTzuShan,
    Indian,
    NorthAmerican1927,
    NorthAmerican1983,
    Oman,
    OrdinanceSurvey1936,
    Pulkovo1942,
    SouthAmerican1956,
    SouthAmerican1969,
    Tokyo,
    Wgs1972,
    Wgs1984,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FixedStation {
    name: String,
    location: UtmLocation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurveyDataFile {
    file_path: String,
    fixed_stations: Vec<FixedStation>,
}

#[derive(Clone, Debug, PartialEq)]
enum ProjectElement {
    BaseLocation(UtmLocation),
    CarriageReturn,
    Comment(String),
    Datum(Datum),
    LineFeed,
    FilePath(SurveyDataFile),
    Space,
    UtmZone(u8),
}

pub struct Project {
    pub base_location: UtmLocation,
    pub datum: Datum,
    pub survey_data: Vec<SurveyDataFile>,
    pub utm_zone: Option<u8>,
}

fn parse_base_location(input: &str) -> IResult<&str, ProjectElement> {
    let (input, _) = char('@')(input)?;
    let (input, east) = double(input)?;
    let (input, _) = char(',')(input)?;
    let (input, north) = double(input)?;
    let (input, _) = char(',')(input)?;
    let (input, elevation) = double(input)?;
    let (input, _) = char(',')(input)?;
    let (input, zone) = u8(input)?;
    let (input, _) = char(',')(input)?;
    let (input, convergence_angle) = double(input)?;
    let (input, _) = char(';')(input)?;
    let east_north_elevation = EastNorthElevation::from_meters(east, north, elevation);
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
    let (input, _) = tag("/")(input)?;
    let (input, comment) = take_till(|c| internal_separator(c))(input)?;

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

fn consume_internal_whitespace(input: &str) -> IResult<&str, &str> {
    take_till(|c: char| !c.is_whitespace())(input)
}

fn internal_separator(c: char) -> bool {
    c == ',' || c == '\r' || c == '\n'
}

fn is_terminator(c: char) -> bool {
    c == ';'
}

fn parse_file_path(input: &str) -> IResult<&str, ProjectElement> {
    let (input, _) = tag("#")(input)?;
    // This should be the full file description, including any links
    let (input, file_info) = take_till(|c| is_terminator(c))(input)?;

    let (input, file_path) = take_till(|c| internal_separator(c) || is_terminator(c))(file_info)?;
    while input.chars().next().unwrap() == ',' {
        let (input, _) = char(',')(input)?;
        let (input, _) = consume_internal_whitespace(input)?;
        let (input, station_name) = take_until("[")(input)?;
        let (input, _) = char('[')(input)?;
        let (input, unit_char) = alt((char('m'), char('f')))(input)?;
        let (input, _) = consume_internal_whitespace(input)?;
        let (input, _) = char(',')(input)?;
        let (input, east) = double(input)?;
        let (input, _) = char(',')(input)?;
        let (input, north) = double(input)?;
        let (input, _) = char(',')(input)?;
        let (input, elevation) = double(input)?;
        let (input, _) = consume_internal_whitespace(input)?;
        let (input, _) = char(']')(input)?;
        let ene = match unit_char {
            'm' => EastNorthElevation::from_meters(east, north, elevation),
            'f' => EastNorthElevation::from_feet(east, north, elevation),
            _ => panic!("invalid unit tag"),
        };
        let (input, _) = char(']')(input)?;
    }

    let data_file = SurveyDataFile {
        file_path: file_path.to_string(),
        fixed_stations: Vec::new(),
    };
    let (input, _) = consume_internal_whitespace(input)?;

    let (input, _) = char(';')(input)?;

    Ok((input, ProjectElement::FilePath(data_file)))
}

fn parse_utm_zone(input: &str) -> IResult<&str, ProjectElement> {
    let (input, _) = tag("$")(input)?;
    let (input, zone) = u8(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, ProjectElement::UtmZone(zone)))
}

fn parse_project_element(input: &str) -> IResult<&str, ProjectElement> {
    alt((
        parse_base_location,
        value(ProjectElement::CarriageReturn, char('\r')),
        parse_comment,
        parse_datum,
        value(ProjectElement::LineFeed, char('\n')),
        parse_file_path,
        parse_utm_zone,
    ))(input)
}

pub fn parse_compass_project(input: &str) -> IResult<&str, Project> {
    let mut input = input;
    let mut base_location: Option<UtmLocation> = None;
    let mut datum: Option<Datum> = None;
    let mut survey_data: Vec<SurveyDataFile> = Vec::new();

    while let Ok((munched, element)) = parse_project_element(input) {
        input = munched;
        match element {
            ProjectElement::BaseLocation(parsed_base_location) => {
                base_location = Some(parsed_base_location)
            }
            ProjectElement::Datum(parsed_datum) => datum = Some(parsed_datum),
            ProjectElement::FilePath(file_info) => survey_data.push(file_info),
            _ => (),
        }
    }
    if base_location.is_some() && datum.is_some() {
        Ok((
            input,
            Project {
                base_location: base_location.unwrap(),
                datum: datum.unwrap(),
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
    use std::path::PathBuf;

    use super::*;
    #[test]
    fn parse_compass_sample_project() {
        let sample_project = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("Fulfords.mak");
        let input = std::fs::read_to_string(sample_project).unwrap();
        let (_, project) = parse_compass_project(&input).unwrap();
        let ene = project.base_location.east_north_elevation;
        assert!(ene.east == 357715.717_f64);
        assert!(ene.north == 4372837.574_f64);
        assert!(ene.elevation == 3048_f64);
        assert!(project.base_location.zone == 13);
        assert!(project.base_location.convergence_angle == -1.050_f64);
        assert!(project.datum == Datum::NorthAmerican1983);
        assert!(project.survey_data.len() > 0)
    }
}
