use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_till1, take_until},
    character::complete::{char, u8},
    combinator::value,
    number::complete::double,
    IResult,
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct UtmLocation {
    east: f64,
    north: f64,
    elevation: f64,
    zone: u8,
    convergence: f64,
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
    Tokyio,
    Wgs1972,
    Wgs1984,
}

#[derive(Clone, Debug, PartialEq)]
enum ProjectElement {
    BaseLocation(UtmLocation),
    CarriageReturn,
    Comment(String),
    Datum(Datum),
    LineFeed,
    FilePath(String),
}

struct Project {
    base_location: UtmLocation,
    datum: Datum,
    survey_data: Vec<SurveyData>,
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
    let (input, convergence) = double(input)?;
    let (input, _) = char(';')(input)?;
    Ok((
        input,
        ProjectElement::BaseLocation(UtmLocation {
            east,
            north,
            elevation,
            zone,
            convergence,
        }),
    ))
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
            value(Datum::Tokyio, tag("Tokyo")),
            value(Datum::Wgs1972, tag("Wgs 1972")),
            value(Datum::Wgs1984, tag("Wgs 1984")),
        )),
    ))(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, ProjectElement::Datum(datum)))
}

fn is_terminator(c: char) -> bool {
    c == ',' || c == ';' || c == '\r' || c == '\n'
}

fn parse_file_path(input: &str) -> IResult<&str, ProjectElement> {
    let (input, _) = tag("#")(input)?;
    let (input, _) = char(' ')(input)?;
    // This should be the file path, but there can be 0 or more fixed stations associated
    let (input, file_path) = take_till1(|c| is_terminator(c))(input)?;

    Ok((input, ProjectElement::FilePath(file_path.to_string())))
}

fn parse_project_element(input: &str) -> IResult<&str, ProjectElement> {
    alt((
        parse_base_location,
        value(ProjectElement::CarriageReturn, char('\r')),
        parse_datum,
        value(ProjectElement::LineFeed, char('\n')),
    ))(input)
}

pub fn parse_compass_project(input: &str) -> IResult<&str, Project> {
    let mut input = input;
    let mut base_location: Option<UtmLocation> = None;
    let mut datum: Option<Datum> = None;

    while let Ok((munched, element)) = parse_project_element(input) {
        input = munched;
        match element {
            ProjectElement::BaseLocation(parsed_base_location) => {
                base_location = Some(parsed_base_location)
            }
            ProjectElement::Datum(parsed_datum) => datum = Some(parsed_datum),
            ProjectElement::FilePath(file_path) => {}
            _ => (),
        }
    }
    if base_location.is_some() && datum.is_some() {
        Ok((
            input,
            Project {
                base_location: base_location.unwrap(),
                datum: datum.unwrap(),
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
        assert!(project.base_location.east == 357715.717_f64);
        assert!(project.base_location.north == 4372837.574_f64);
        assert!(project.base_location.elevation == 3048_f64);
        assert!(project.base_location.zone == 13);
        assert!(project.base_location.convergence == -1.050_f64);
        assert!(project.datum == Datum::NorthAmerican1983);
    }
}
