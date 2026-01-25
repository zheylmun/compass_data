//! Parser for Compass project (.mak) files
//!
//! Converts tokens from the lexer into a Project structure.

// False positives from thiserror/miette derive macros - fields are used for error display
#![allow(unused_assignments)]

use std::{collections::HashSet, marker::PhantomData, path::PathBuf};

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;
use uuid::Uuid;

use crate::{EastNorthElevation, UtmLocation};

use super::{
    DatFile, Datum, DeclinationMode, FileConvergence, FileState, Project, ProjectParameters,
    Station, Unloaded,
    lexer::{Span, Spanned, StationToken, Token},
};

/// Parser error with source location for nice error reporting
#[derive(Error, Debug, Diagnostic)]
pub enum ParseError {
    #[error("Unknown datum '{name}'")]
    #[diagnostic(
        code(compass::parse::unknown_datum),
        help("Valid datums include: North American 1983, WGS 1984, European 1950, etc.")
    )]
    UnknownDatum {
        name: String,
        #[label("unknown datum")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Invalid UUID format")]
    #[diagnostic(code(compass::parse::invalid_uuid))]
    InvalidUuid {
        #[label("expected valid UUID")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Survey file encountered before required state was set")]
    #[diagnostic(
        code(compass::parse::missing_state),
        help("Ensure base location (@) and datum (&) are set before listing files (#)")
    )]
    MissingState {
        #[label("file listed here")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Duplicate project parameter flag '{flag}'")]
    #[diagnostic(code(compass::parse::duplicate_flag))]
    DuplicateFlag {
        flag: char,
        #[label("duplicate flag")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Unknown project parameter flag '{flag}'")]
    #[diagnostic(
        code(compass::parse::unknown_flag),
        help("Valid flags: G/g, I/E/A, V/v, O/o, T/t, S/s, X/x, P/p, L/l, C/c")
    )]
    UnknownFlag {
        flag: char,
        #[label("unknown flag")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },
}

/// Internal parser state for tracking rolling state during parsing
#[derive(Clone, Default)]
struct ParserState {
    datum: Option<Datum>,
    base_location: Option<UtmLocation>,
    utm_zone: Option<u8>,
    file_convergence: Option<FileConvergence>,
    project_parameters: Option<ProjectParameters>,
}

impl ParserState {
    /// Convert current state to a `FileState` if required fields are present
    fn to_file_state(&self) -> Option<FileState> {
        Some(FileState {
            datum: self.datum?,
            base_location: self.base_location?,
            utm_zone: self.utm_zone,
            file_convergence: self.file_convergence,
            project_parameters: self.project_parameters,
        })
    }
}

/// Parse tokens into a Project
///
/// # Arguments
///
/// * `file_path` - The path to the project file (for resolving relative paths)
/// * `tokens` - The tokens from the lexer
/// * `source` - The original source string (for error messages)
///
/// # Errors
///
/// Returns a `ParseError` if the tokens cannot be parsed into a valid project.
pub fn parse_project(
    file_path: PathBuf,
    tokens: &[Spanned<Token>],
    source: &str,
) -> Result<Project<Unloaded>, ParseError> {
    let mut project_id = None;
    let mut state = ParserState::default();
    let mut survey_files = Vec::new();
    let mut folders: Vec<String> = Vec::new();

    for spanned in tokens {
        match &spanned.value {
            Token::ProjectId(id) => {
                project_id = Some(Uuid::parse_str(id).map_err(|_| ParseError::InvalidUuid {
                    span: spanned.span.into(),
                    src: source.to_string(),
                })?);
            }
            Token::Datum(name) => {
                state.datum = Some(parse_datum_name(name, spanned.span, source)?);
            }
            Token::BaseLocation {
                east,
                north,
                elevation,
                zone,
                convergence,
            } => {
                state.base_location = Some(UtmLocation {
                    east_north_elevation: EastNorthElevation::from_meters(
                        *east, *north, *elevation,
                    ),
                    zone: *zone,
                    convergence_angle: *convergence,
                });
            }
            Token::UtmZone(zone) => {
                state.utm_zone = Some(*zone);
            }
            Token::FileConvergenceEnabled(angle) => {
                state.file_convergence = Some(FileConvergence {
                    enabled: true,
                    angle: *angle,
                });
            }
            Token::FileConvergenceDisabled(angle) => {
                state.file_convergence = Some(FileConvergence {
                    enabled: false,
                    angle: *angle,
                });
            }
            Token::ProjectParameters(flags) => {
                state.project_parameters = Some(parse_project_params(flags, spanned.span, source)?);
            }
            Token::SurveyFile { path, stations } => {
                let file_state = state
                    .to_file_state()
                    .ok_or_else(|| ParseError::MissingState {
                        span: spanned.span.into(),
                        src: source.to_string(),
                    })?;

                // Prepend current folder path
                let full_path = folders
                    .iter()
                    .fold(PathBuf::new(), |p, f| p.join(f))
                    .join(path);

                survey_files.push(DatFile {
                    file_path: full_path,
                    project_stations: convert_stations(stations),
                    file_state,
                    surveys: vec![],
                    state: PhantomData,
                });
            }
            Token::PushFolder(folder) => {
                folders.push(folder.clone());
            }
            Token::PopFolder => {
                folders.pop();
            }
            Token::Comment(_) => {
                // Ignore comments
            }
        }
    }

    Ok(Project {
        id: project_id,
        file_path,
        survey_files,
        state: PhantomData::<Unloaded>,
    })
}

fn parse_datum_name(name: &str, span: Span, source: &str) -> Result<Datum, ParseError> {
    match name {
        "Adindan" => Ok(Datum::Adindan),
        "Arc 1950" => Ok(Datum::Arc1950),
        "Arc 1960" => Ok(Datum::Arc1960),
        "Australian 1966" => Ok(Datum::Australian1966),
        "Australian 1984" => Ok(Datum::Australian1984),
        "Camp Area Astro" => Ok(Datum::CampAreaAstro),
        "Cape" => Ok(Datum::Cape),
        "European 1950" => Ok(Datum::European1950),
        "European 1979" => Ok(Datum::European1979),
        "Geodetic 1949" => Ok(Datum::Geodetic1949),
        "HongKong 1963" => Ok(Datum::HongKong1963),
        "HuTzuShan" => Ok(Datum::HuTzuShan),
        "Indian" => Ok(Datum::Indian),
        "North American 1927" => Ok(Datum::NorthAmerican1927),
        "North American 1983" => Ok(Datum::NorthAmerican1983),
        "Oman" => Ok(Datum::Oman),
        "Ordinance Survey 1936" => Ok(Datum::OrdinanceSurvey1936),
        "Pulkovo 1942" => Ok(Datum::Pulkovo1942),
        "South American 1956" => Ok(Datum::SouthAmerican1956),
        "South American 1969" => Ok(Datum::SouthAmerican1969),
        "Tokyo" => Ok(Datum::Tokyo),
        "WGS 1972" => Ok(Datum::WGS1972),
        "WGS 1984" => Ok(Datum::WGS1984),
        _ => Err(ParseError::UnknownDatum {
            name: name.to_string(),
            span: span.into(),
            src: source.to_string(),
        }),
    }
}

fn parse_project_params(
    flags: &str,
    span: Span,
    source: &str,
) -> Result<ProjectParameters, ParseError> {
    let mut params = ProjectParameters::default();
    let mut seen = HashSet::new();

    for c in flags.chars() {
        let key = c.to_ascii_uppercase();
        // For I/E/A, they're mutually exclusive but different keys
        let duplicate_key = match key {
            'I' | 'E' | 'A' => 'D', // Use 'D' as declination group key
            _ => key,
        };
        if !seen.insert(duplicate_key) {
            return Err(ParseError::DuplicateFlag {
                flag: c,
                span: span.into(),
                src: source.to_string(),
            });
        }
        match c {
            'G' => params.global_override = true,
            'g' => params.global_override = false,
            'I' => params.declination_mode = DeclinationMode::Ignore,
            'E' => params.declination_mode = DeclinationMode::Entered,
            'A' => params.declination_mode = DeclinationMode::Auto,
            'V' => params.utm_convergence_applied = true,
            'v' => params.utm_convergence_applied = false,
            'O' => params.override_lrud_association = true,
            'o' => params.override_lrud_association = false,
            'T' => params.lrud_to_station = true,
            't' => params.lrud_to_station = false,
            'S' => params.shot_flags_applied = true,
            's' => params.shot_flags_applied = false,
            'X' => params.total_exclusion_applied = true,
            'x' => params.total_exclusion_applied = false,
            'P' => params.plotting_exclusion_applied = true,
            'p' => params.plotting_exclusion_applied = false,
            'L' => params.length_exclusion_applied = true,
            'l' => params.length_exclusion_applied = false,
            'C' => params.close_exclusion_applied = true,
            'c' => params.close_exclusion_applied = false,
            _ => {
                return Err(ParseError::UnknownFlag {
                    flag: c,
                    span: span.into(),
                    src: source.to_string(),
                });
            }
        }
    }
    Ok(params)
}

fn convert_stations(tokens: &[StationToken]) -> Vec<Station> {
    tokens
        .iter()
        .map(|t| Station {
            name: t.name.clone(),
            location: t.fix.as_ref().map(|f| {
                if f.unit == 'f' {
                    EastNorthElevation::from_feet(f.east, f.north, f.elevation)
                } else {
                    EastNorthElevation::from_meters(f.east, f.north, f.elevation)
                }
            }),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::project::lexer::tokenize;

    use super::*;

    #[test]
    fn parse_simple_project() {
        let input =
            "@357715.717,4372837.574,3048.000,13,-1.050;\n&North American 1983;\n#Fulford.dat;";
        let tokens = tokenize(input).unwrap();
        let project = parse_project(PathBuf::from("test.mak"), &tokens, input).unwrap();

        assert_eq!(project.survey_files.len(), 1);
        assert_eq!(
            project.survey_files[0].file_state.datum,
            Datum::NorthAmerican1983
        );
        assert_float_eq!(
            project.survey_files[0]
                .file_state
                .base_location
                .east_north_elevation
                .easting,
            357_715.717,
            rmax <= 0.001
        );
    }

    #[test]
    fn parse_project_with_parameters() {
        let input = "@100.0,200.0,300.0,13,0.5;\n&WGS 1984;\n!GAVOTSCXPL;\n#test.dat;";
        let tokens = tokenize(input).unwrap();
        let project = parse_project(PathBuf::from("test.mak"), &tokens, input).unwrap();

        let params = project.survey_files[0]
            .file_state
            .project_parameters
            .as_ref()
            .unwrap();
        assert!(params.global_override);
        assert!(params.utm_convergence_applied);
    }

    #[test]
    fn parse_project_with_folders() {
        let input =
            "@100.0,200.0,300.0,13,0.5;\n&WGS 1984;\n[Folder1;\n#cave1.dat;\n];\n#cave2.dat;";
        let tokens = tokenize(input).unwrap();
        let project = parse_project(PathBuf::from("test.mak"), &tokens, input).unwrap();

        assert_eq!(project.survey_files.len(), 2);
        assert_eq!(
            project.survey_files[0].file_path,
            PathBuf::from("Folder1/cave1.dat")
        );
        assert_eq!(
            project.survey_files[1].file_path,
            PathBuf::from("cave2.dat")
        );
    }

    #[test]
    fn parse_error_missing_state() {
        let input = "#test.dat;";
        let tokens = tokenize(input).unwrap();
        let result = parse_project(PathBuf::from("test.mak"), &tokens, input);

        assert!(matches!(result, Err(ParseError::MissingState { .. })));
    }

    #[test]
    fn parse_error_unknown_datum() {
        let input = "@100.0,200.0,300.0,13,0.5;\n&Unknown Datum;\n#test.dat;";
        let tokens = tokenize(input).unwrap();
        let result = parse_project(PathBuf::from("test.mak"), &tokens, input);

        assert!(matches!(result, Err(ParseError::UnknownDatum { .. })));
    }

    #[test]
    fn parse_format_examples() {
        const FILE_PATH: &str = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_data/project_file_examples"
        );
        let input = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_data/project_file_examples"
        ));
        let tokens = tokenize(input).unwrap();
        let path = PathBuf::from(FILE_PATH);
        let project = parse_project(path, &tokens, input).unwrap();

        assert!(project.id.is_some());
        assert_eq!(project.survey_files.len(), 17);

        // Check state from first file
        let first_file = &project.survey_files[0];
        let ene = &first_file.file_state.base_location.east_north_elevation;
        assert_float_eq!(ene.easting, 398_315.500, rmax <= 0.001);
        assert_float_eq!(ene.northing, 4_483_735.300, rmax <= 0.001);
        assert_float_eq!(ene.up, 3_048.000, rmax <= 0.001);
        assert!(first_file.file_state.base_location.zone == 13);
        assert_float_eq!(
            first_file.file_state.base_location.convergence_angle,
            0.780,
            rmax <= 0.001
        );
        assert!(first_file.file_state.datum == Datum::NorthAmerican1983);
    }

    #[test]
    fn parse_compass_sample_project() {
        let sample_project = include_str!("../../test_data/Fulfords.mak");
        let tokens = tokenize(sample_project).unwrap();
        let file_path = PathBuf::from("../../test_data/Fulfords.mak");
        let project = parse_project(file_path, &tokens, sample_project).unwrap();

        // Access state from first file
        let first_file = &project.survey_files[0];
        let enu = &first_file.file_state.base_location.east_north_elevation;
        assert_float_eq!(enu.easting, 357_715.717_f64, rmax <= 0.001);
        assert_float_eq!(enu.northing, 4_372_837.574_f64, rmax <= 0.001);
        assert_float_eq!(enu.up, 3_048_f64, rmax <= 0.001);
        assert!(first_file.file_state.base_location.zone == 13);
        assert_float_eq!(
            first_file.file_state.base_location.convergence_angle,
            -1.050_f64,
            rmax <= 0.001
        );
        assert!(first_file.file_state.datum == Datum::NorthAmerican1983);
        assert!(!project.survey_files.is_empty());
    }
}
