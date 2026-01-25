//! Lexer for Compass project (.mak) files
//!
//! Converts raw input strings into a stream of tokens with source spans for error reporting.

use miette::{Diagnostic, SourceSpan};
use nom::{
    IResult, Parser,
    bytes::complete::{take_till, take_while, take_while1},
    character::complete::{char, multispace0, one_of, u8 as parse_u8},
    combinator::opt,
};
use thiserror::Error;

/// Source span for a token
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    pub offset: usize,
    pub length: usize,
}

impl Span {
    #[must_use]
    pub const fn new(offset: usize, length: usize) -> Self {
        Self { offset, length }
    }
}

impl From<Span> for SourceSpan {
    fn from(span: Span) -> Self {
        SourceSpan::new(span.offset.into(), span.length)
    }
}

/// Token with source location
#[derive(Clone, Debug, PartialEq)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

/// Project file tokens
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// Project ID: `/uuid;`
    ProjectId(String),
    /// Base location: `@east,north,elev,zone,convergence;`
    BaseLocation {
        east: f64,
        north: f64,
        elevation: f64,
        zone: u8,
        convergence: f64,
    },
    /// Datum: `&datum_name;`
    Datum(String),
    /// UTM Zone: `$zone;`
    UtmZone(u8),
    /// File convergence enabled: `%angle;`
    FileConvergenceEnabled(f64),
    /// File convergence disabled: `*angle;`
    FileConvergenceDisabled(f64),
    /// Project parameters: `!flags;`
    ProjectParameters(String),
    /// Survey file: `#path,stations;`
    SurveyFile {
        path: String,
        stations: Vec<StationToken>,
    },
    /// Push folder: `[folder;`
    PushFolder(String),
    /// Pop folder: `];`
    PopFolder,
    /// Comment (filtered before parsing)
    Comment(String),
}

/// Station reference in a survey file declaration
#[derive(Clone, Debug, PartialEq)]
pub struct StationToken {
    pub name: String,
    pub fix: Option<StationFixToken>,
}

/// Fixed location for a station
#[derive(Clone, Debug, PartialEq)]
pub struct StationFixToken {
    pub unit: char,
    pub east: f64,
    pub north: f64,
    pub elevation: f64,
}

/// Lexer error with source location
#[derive(Error, Debug, Diagnostic)]
pub enum LexError {
    #[error("Unexpected character '{char}'")]
    #[diagnostic(code(compass::lex::unexpected_char))]
    UnexpectedChar {
        char: char,
        #[label("unexpected character here")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Invalid number format")]
    #[diagnostic(code(compass::lex::invalid_number))]
    InvalidNumber {
        #[label("expected a valid number")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Missing semicolon terminator")]
    #[diagnostic(code(compass::lex::missing_semicolon))]
    MissingSemicolon {
        #[label("expected ';' after this")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Invalid base location format")]
    #[diagnostic(
        code(compass::lex::invalid_base_location),
        help("Expected format: @east,north,elevation,zone,convergence;")
    )]
    InvalidBaseLocation {
        #[label("invalid base location")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Invalid station fix format")]
    #[diagnostic(
        code(compass::lex::invalid_station_fix),
        help("Expected format: [f,east,north,elevation] or [m,east,north,elevation]")
    )]
    InvalidStationFix {
        #[label("invalid station fix")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },
}

/// Tokenize input, returning tokens with their source spans
///
/// # Errors
///
/// Returns a `LexError` if the input contains invalid syntax.
pub fn tokenize(input: &str) -> Result<Vec<Spanned<Token>>, LexError> {
    let mut tokens = Vec::new();
    let mut remaining = input;
    let mut offset = 0;

    while !remaining.is_empty() {
        // Skip whitespace and control characters (including SUB/EOF char 0x1A)
        let (new_remaining, ws) = take_while::<_, _, nom::error::Error<&str>>(|c: char| {
            c.is_ascii_whitespace() || c == '\x1a'
        })
        .parse(remaining)
        .unwrap();
        offset += ws.len();
        remaining = new_remaining;

        if remaining.is_empty() {
            break;
        }

        let start = offset;
        let first_char = remaining.chars().next().unwrap();

        let (new_remaining, token) = match first_char {
            '/' => lex_project_id_or_comment(remaining, input, start)?,
            '@' => lex_base_location(remaining, input, start)?,
            '&' => lex_datum(remaining, input, start)?,
            '$' => lex_utm_zone(remaining, input, start)?,
            '%' => lex_file_convergence(remaining, input, start, true)?,
            '*' => lex_file_convergence(remaining, input, start, false)?,
            '!' => lex_project_parameters(remaining, input, start)?,
            '#' => lex_survey_file(remaining, input, start)?,
            '[' => lex_push_folder(remaining, input, start)?,
            ']' => lex_pop_folder(remaining, input, start)?,
            c => {
                return Err(LexError::UnexpectedChar {
                    char: c,
                    span: SourceSpan::new(start.into(), 1),
                    src: input.to_string(),
                });
            }
        };

        let consumed = remaining.len() - new_remaining.len();
        let span = Span::new(start, consumed);
        tokens.push(Spanned { value: token, span });

        offset += consumed;
        remaining = new_remaining;
    }

    Ok(tokens)
}

fn lex_project_id_or_comment<'a>(
    input: &'a str,
    full_src: &str,
    start: usize,
) -> Result<(&'a str, Token), LexError> {
    // Skip the '/'
    let remaining = &input[1..];

    // Check if this looks like a UUID (starts with hex digit or has hyphen pattern)
    // UUIDs end with ';', comments end with newline or another '/'
    if let Some(semi_pos) = remaining.find(';') {
        let potential_uuid = &remaining[..semi_pos];
        // Check if it looks like a UUID (only hex chars and hyphens)
        if potential_uuid
            .chars()
            .all(|c| c.is_ascii_hexdigit() || c == '-')
            && potential_uuid.len() >= 32
        {
            // This is a project ID
            let after_semi = &remaining[semi_pos + 1..];
            return Ok((after_semi, Token::ProjectId(potential_uuid.to_string())));
        }
    }

    // This is a comment - take until newline or another '/'
    let (remaining, comment) =
        take_till::<_, _, nom::error::Error<&str>>(|c| c == '/' || c == '\n' || c == '\r')
            .parse(remaining)
            .map_err(|_| LexError::UnexpectedChar {
                char: '/',
                span: SourceSpan::new(start.into(), 1),
                src: full_src.to_string(),
            })?;

    Ok((remaining, Token::Comment(comment.to_string())))
}

fn parse_double(input: &str) -> IResult<&str, f64> {
    let (input, _) = multispace0(input)?;
    let (input, sign) = opt(one_of("+-")).parse(input)?;
    let (input, int_part) = take_while1(|c: char| c.is_ascii_digit())(input)?;
    let (input, frac_part) = opt(|i| {
        let (i, _) = char('.')(i)?;
        take_while1(|c: char| c.is_ascii_digit())(i)
    })
    .parse(input)?;

    let mut num_str = String::new();
    if let Some(s) = sign {
        num_str.push(s);
    }
    num_str.push_str(int_part);
    if let Some(frac) = frac_part {
        num_str.push('.');
        num_str.push_str(frac);
    }

    let value = num_str.parse::<f64>().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Float))
    })?;

    Ok((input, value))
}

fn lex_base_location<'a>(
    input: &'a str,
    full_src: &str,
    start: usize,
) -> Result<(&'a str, Token), LexError> {
    let err = || LexError::InvalidBaseLocation {
        span: SourceSpan::new(start.into(), input.find(';').unwrap_or(20).min(50)),
        src: full_src.to_string(),
    };

    // Skip '@'
    let remaining = &input[1..];

    let (remaining, east) = parse_double(remaining).map_err(|_| err())?;
    let (remaining, _) = char::<_, nom::error::Error<&str>>(',')
        .parse(remaining)
        .map_err(|_| err())?;
    let (remaining, north) = parse_double(remaining).map_err(|_| err())?;
    let (remaining, _) = char::<_, nom::error::Error<&str>>(',')
        .parse(remaining)
        .map_err(|_| err())?;
    let (remaining, elevation) = parse_double(remaining).map_err(|_| err())?;
    let (remaining, _) = char::<_, nom::error::Error<&str>>(',')
        .parse(remaining)
        .map_err(|_| err())?;
    let (remaining, zone) = parse_u8::<_, nom::error::Error<&str>>(remaining).map_err(|_| err())?;
    let (remaining, _) = char::<_, nom::error::Error<&str>>(',')
        .parse(remaining)
        .map_err(|_| err())?;
    let (remaining, convergence) = parse_double(remaining).map_err(|_| err())?;
    let (remaining, _) = char::<_, nom::error::Error<&str>>(';')
        .parse(remaining)
        .map_err(|_| err())?;

    Ok((
        remaining,
        Token::BaseLocation {
            east,
            north,
            elevation,
            zone,
            convergence,
        },
    ))
}

fn lex_datum<'a>(
    input: &'a str,
    full_src: &str,
    start: usize,
) -> Result<(&'a str, Token), LexError> {
    // Skip '&'
    let remaining = &input[1..];

    let semi_pos = remaining.find(';').ok_or_else(|| LexError::MissingSemicolon {
        span: SourceSpan::new(start.into(), remaining.len().min(20)),
        src: full_src.to_string(),
    })?;

    let datum_name = &remaining[..semi_pos];
    let after_semi = &remaining[semi_pos + 1..];

    Ok((after_semi, Token::Datum(datum_name.to_string())))
}

fn lex_utm_zone<'a>(
    input: &'a str,
    full_src: &str,
    start: usize,
) -> Result<(&'a str, Token), LexError> {
    // Skip '$'
    let remaining = &input[1..];

    let (remaining, zone) =
        parse_u8::<_, nom::error::Error<&str>>(remaining).map_err(|_| LexError::InvalidNumber {
            span: SourceSpan::new((start + 1).into(), 2),
            src: full_src.to_string(),
        })?;

    let (remaining, _) =
        char::<_, nom::error::Error<&str>>(';')
            .parse(remaining)
            .map_err(|_| LexError::MissingSemicolon {
                span: SourceSpan::new(start.into(), 3),
                src: full_src.to_string(),
            })?;

    Ok((remaining, Token::UtmZone(zone)))
}

fn lex_file_convergence<'a>(
    input: &'a str,
    full_src: &str,
    start: usize,
    enabled: bool,
) -> Result<(&'a str, Token), LexError> {
    // Skip '%' or '*'
    let remaining = &input[1..];

    let (remaining, angle) =
        parse_double(remaining).map_err(|_| LexError::InvalidNumber {
            span: SourceSpan::new((start + 1).into(), 5),
            src: full_src.to_string(),
        })?;

    let (remaining, _) =
        char::<_, nom::error::Error<&str>>(';')
            .parse(remaining)
            .map_err(|_| LexError::MissingSemicolon {
                span: SourceSpan::new(start.into(), 10),
                src: full_src.to_string(),
            })?;

    let token = if enabled {
        Token::FileConvergenceEnabled(angle)
    } else {
        Token::FileConvergenceDisabled(angle)
    };

    Ok((remaining, token))
}

fn lex_project_parameters<'a>(
    input: &'a str,
    full_src: &str,
    start: usize,
) -> Result<(&'a str, Token), LexError> {
    // Skip '!'
    let remaining = &input[1..];

    let semi_pos = remaining.find(';').ok_or_else(|| LexError::MissingSemicolon {
        span: SourceSpan::new(start.into(), remaining.len().min(15)),
        src: full_src.to_string(),
    })?;

    let flags = &remaining[..semi_pos];
    let after_semi = &remaining[semi_pos + 1..];

    Ok((after_semi, Token::ProjectParameters(flags.to_string())))
}

fn is_valid_station_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '*' || c == '\'' || c == '-' || c == '_'
}

fn lex_station_fix<'a>(
    input: &'a str,
    full_src: &str,
    start: usize,
) -> Result<(&'a str, StationFixToken), LexError> {
    let err = || LexError::InvalidStationFix {
        span: SourceSpan::new(start.into(), input.find(']').unwrap_or(20).min(30)),
        src: full_src.to_string(),
    };

    // Skip '['
    let remaining = &input[1..];

    // Skip whitespace
    let (remaining, _) = multispace0::<_, nom::error::Error<&str>>(remaining).map_err(|_| err())?;

    // Parse unit (f or m)
    let (remaining, unit_char) = one_of::<_, _, nom::error::Error<&str>>("fFmM")
        .parse(remaining)
        .map_err(|_| err())?;

    // Skip whitespace
    let (remaining, _) = multispace0::<_, nom::error::Error<&str>>(remaining).map_err(|_| err())?;

    let (remaining, _) = char::<_, nom::error::Error<&str>>(',')
        .parse(remaining)
        .map_err(|_| err())?;

    let (remaining, east) = parse_double(remaining).map_err(|_| err())?;
    let (remaining, _) = char::<_, nom::error::Error<&str>>(',')
        .parse(remaining)
        .map_err(|_| err())?;
    let (remaining, north) = parse_double(remaining).map_err(|_| err())?;
    let (remaining, _) = char::<_, nom::error::Error<&str>>(',')
        .parse(remaining)
        .map_err(|_| err())?;
    let (remaining, elevation) = parse_double(remaining).map_err(|_| err())?;

    let (remaining, _) = char::<_, nom::error::Error<&str>>(']')
        .parse(remaining)
        .map_err(|_| err())?;

    Ok((
        remaining,
        StationFixToken {
            unit: unit_char.to_ascii_lowercase(),
            east,
            north,
            elevation,
        },
    ))
}

fn lex_station<'a>(
    input: &'a str,
    full_src: &str,
    start: usize,
) -> Result<(&'a str, StationToken), LexError> {
    // Skip leading comma
    let (remaining, _) =
        char::<_, nom::error::Error<&str>>(',')
            .parse(input)
            .map_err(|_| LexError::UnexpectedChar {
                char: input.chars().next().unwrap_or('?'),
                span: SourceSpan::new(start.into(), 1),
                src: full_src.to_string(),
            })?;

    // Skip any comments and whitespace
    let mut remaining = remaining;
    loop {
        let (new_remaining, _) =
            multispace0::<_, nom::error::Error<&str>>(remaining).unwrap();
        remaining = new_remaining;

        // Check for inline comment (any '/' in station context is a comment, not a UUID)
        if remaining.starts_with('/') {
            // Skip to end of comment (newline)
            let (new_remaining, _) =
                take_till::<_, _, nom::error::Error<&str>>(|c| c == '\n' || c == '\r')
                    .parse(&remaining[1..])
                    .unwrap();
            remaining = new_remaining;
        } else {
            break;
        }
    }

    // Parse station name
    let (remaining, _) = multispace0::<_, nom::error::Error<&str>>(remaining).unwrap();
    let (remaining, name) =
        take_while1::<_, _, nom::error::Error<&str>>(is_valid_station_name_char)
            .parse(remaining)
            .map_err(|_| LexError::UnexpectedChar {
                char: remaining.chars().next().unwrap_or('?'),
                span: SourceSpan::new(start.into(), 1),
                src: full_src.to_string(),
            })?;

    // Check for optional fix
    let (remaining, _) = multispace0::<_, nom::error::Error<&str>>(remaining).unwrap();

    if remaining.starts_with('[') {
        let fix_start = start + (input.len() - remaining.len());
        let (remaining, fix) = lex_station_fix(remaining, full_src, fix_start)?;
        Ok((
            remaining,
            StationToken {
                name: name.to_string(),
                fix: Some(fix),
            },
        ))
    } else {
        Ok((
            remaining,
            StationToken {
                name: name.to_string(),
                fix: None,
            },
        ))
    }
}

fn lex_survey_file<'a>(
    input: &'a str,
    full_src: &str,
    start: usize,
) -> Result<(&'a str, Token), LexError> {
    // Skip '#'
    let remaining = &input[1..];

    // Find the semicolon that ends this file entry
    // But be careful - there might be brackets inside for station fixes
    let mut depth = 0;
    let mut semi_pos = None;
    for (i, c) in remaining.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ';' if depth == 0 => {
                semi_pos = Some(i);
                break;
            }
            _ => {}
        }
    }

    let semi_pos = semi_pos.ok_or_else(|| LexError::MissingSemicolon {
        span: SourceSpan::new(start.into(), remaining.len().min(30)),
        src: full_src.to_string(),
    })?;

    let file_content = &remaining[..semi_pos];
    let after_semi = &remaining[semi_pos + 1..];

    // Parse the file path (up to first comma or end)
    let comma_pos = file_content.find(',');
    let (path, stations_str) = match comma_pos {
        Some(pos) => (&file_content[..pos], &file_content[pos..]),
        None => (file_content, ""),
    };

    // Trim whitespace from path
    let path = path.trim();

    // Parse stations
    let mut stations = Vec::new();
    let mut stations_remaining = stations_str;

    while !stations_remaining.is_empty() {
        // Skip whitespace
        let (new_remaining, _) =
            multispace0::<_, nom::error::Error<&str>>(stations_remaining).unwrap();
        stations_remaining = new_remaining;

        if stations_remaining.is_empty() {
            break;
        }

        // Check for inline comment (any '/' in station context is a comment, not a UUID)
        if stations_remaining.starts_with('/') {
            // Skip to end of comment (newline)
            let (new_remaining, _) =
                take_till::<_, _, nom::error::Error<&str>>(|c| c == '\n' || c == '\r')
                    .parse(&stations_remaining[1..])
                    .unwrap();
            stations_remaining = new_remaining;
            continue;
        }

        // Check if there's a station (starts with comma)
        if stations_remaining.starts_with(',') {
            let station_start = start + (input.len() - stations_remaining.len() - 1);
            let (new_remaining, station) = lex_station(stations_remaining, full_src, station_start)?;
            stations.push(station);
            stations_remaining = new_remaining;
        } else {
            break;
        }
    }

    Ok((
        after_semi,
        Token::SurveyFile {
            path: path.to_string(),
            stations,
        },
    ))
}

fn lex_push_folder<'a>(
    input: &'a str,
    full_src: &str,
    start: usize,
) -> Result<(&'a str, Token), LexError> {
    // Skip '['
    let remaining = &input[1..];

    // Check if this is a pop folder `];`
    let (remaining, _) = multispace0::<_, nom::error::Error<&str>>(remaining).unwrap();
    if remaining.starts_with(';') {
        // This is actually PopFolder - but we should have caught this as `]`
        // This case shouldn't happen in normal parsing
        return Err(LexError::UnexpectedChar {
            char: ';',
            span: SourceSpan::new((start + 1).into(), 1),
            src: full_src.to_string(),
        });
    }

    let semi_pos = remaining.find(';').ok_or_else(|| LexError::MissingSemicolon {
        span: SourceSpan::new(start.into(), remaining.len().min(20)),
        src: full_src.to_string(),
    })?;

    let folder_name = &remaining[..semi_pos];
    let after_semi = &remaining[semi_pos + 1..];

    Ok((after_semi, Token::PushFolder(folder_name.to_string())))
}

fn lex_pop_folder<'a>(
    input: &'a str,
    full_src: &str,
    start: usize,
) -> Result<(&'a str, Token), LexError> {
    // Skip ']'
    let remaining = &input[1..];

    let (remaining, _) =
        char::<_, nom::error::Error<&str>>(';')
            .parse(remaining)
            .map_err(|_| LexError::MissingSemicolon {
                span: SourceSpan::new(start.into(), 2),
                src: full_src.to_string(),
            })?;

    Ok((remaining, Token::PopFolder))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_project() {
        let input = "@357715.717,4372837.574,3048.000,13,-1.050;\n&North American 1983;\n#Fulford.dat;";
        let tokens = tokenize(input).unwrap();

        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0].value, Token::BaseLocation { .. }));
        assert!(matches!(&tokens[1].value, Token::Datum(d) if d == "North American 1983"));
        assert!(matches!(&tokens[2].value, Token::SurveyFile { path, .. } if path == "Fulford.dat"));
    }

    #[test]
    fn test_tokenize_with_project_id() {
        let input = "/2602942a-ef0c-46d5-b528-77a72c77239c;";
        let tokens = tokenize(input).unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(&tokens[0].value, Token::ProjectId(id) if id == "2602942a-ef0c-46d5-b528-77a72c77239c"));
    }

    #[test]
    fn test_tokenize_with_comments() {
        let input = "@357715.717,4372837.574,3048.000,13,-1.050;\n/ This is a comment\n&North American 1983;";
        let tokens = tokenize(input).unwrap();

        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0].value, Token::BaseLocation { .. }));
        assert!(matches!(&tokens[1].value, Token::Comment(_)));
        assert!(matches!(&tokens[2].value, Token::Datum(_)));
    }

    #[test]
    fn test_tokenize_file_with_stations() {
        let input = "#Fulford.dat,A1[f,1173607.995,14346579.967,10000.000],B2;";
        let tokens = tokenize(input).unwrap();

        assert_eq!(tokens.len(), 1);
        if let Token::SurveyFile { path, stations } = &tokens[0].value {
            assert_eq!(path, "Fulford.dat");
            assert_eq!(stations.len(), 2);
            assert_eq!(stations[0].name, "A1");
            assert!(stations[0].fix.is_some());
            assert_eq!(stations[1].name, "B2");
            assert!(stations[1].fix.is_none());
        } else {
            panic!("Expected SurveyFile token");
        }
    }

    #[test]
    fn test_tokenize_project_parameters() {
        let input = "!GAVOTSCXPL;";
        let tokens = tokenize(input).unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(&tokens[0].value, Token::ProjectParameters(f) if f == "GAVOTSCXPL"));
    }

    #[test]
    fn test_tokenize_file_convergence() {
        let input = "%1.234;*-2.5;";
        let tokens = tokenize(input).unwrap();

        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0].value, Token::FileConvergenceEnabled(a) if (a - 1.234).abs() < 0.001));
        assert!(matches!(tokens[1].value, Token::FileConvergenceDisabled(a) if (a - (-2.5)).abs() < 0.001));
    }

    #[test]
    fn test_tokenize_folders() {
        let input = "[Folder-1;\n#cave1.dat;\n];";
        let tokens = tokenize(input).unwrap();

        assert_eq!(tokens.len(), 3);
        assert!(matches!(&tokens[0].value, Token::PushFolder(f) if f == "Folder-1"));
        assert!(matches!(&tokens[1].value, Token::SurveyFile { path, .. } if path == "cave1.dat"));
        assert!(matches!(tokens[2].value, Token::PopFolder));
    }
}
