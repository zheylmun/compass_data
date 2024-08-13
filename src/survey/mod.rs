use crate::{common_types::Date, Error};

mod parser;

#[derive(Clone, Debug, PartialEq)]
pub struct CorrectionFactors {
    pub azimuth: f64,
    pub inclination: f64,
    pub length: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BackSightCorrectionFactors {
    pub azimuth: f64,
    pub inclination: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parameters {
    pub declination: f64,
    pub correction_factors: Option<CorrectionFactors>,
    pub backsight_correction_factors: Option<BackSightCorrectionFactors>,
}

impl Parameters {
    // TODO: this is probably the where I need to enforce column widths and such
    // Some general things:
    // 1. overall Rust review
    // 2. look at docs / samples to understand constraints
    fn serialize(&self) -> String {
        let mut result = String::new();
        // TODO: we might be missing FORMAT
        result.push_str(&format!("DECLINATION: {:>4.2}  ", self.declination));
        if let Some(correction_factors) = &self.correction_factors {
            result.push_str(&format!(
                "CORRECTIONS:  {:.2} {:.2} {:.2}",
                correction_factors.azimuth,
                correction_factors.inclination,
                correction_factors.length
            ));
        }
        if let Some(backsight_correction_factors) = &self.backsight_correction_factors {
            result.push_str(&format!(
                "CORRECTIONS2: {:.1} {:.1}",
                backsight_correction_factors.azimuth, backsight_correction_factors.inclination
            ));
        }
        result.push_str("\r\n");
        result
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Shot {
    pub from: String,
    pub to: String,
    pub length: f64,
    pub azimuth: f64,
    pub inclination: f64,
    pub up: f64,
    pub down: f64,
    pub left: f64,
    pub right: f64,
    pub flags: Option<String>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Survey {
    pub cave_name: String,
    pub name: String,
    pub date: Date,
    pub comment: Option<String>,
    pub team: String,
    pub parameters: Parameters,
    pub shots: Vec<Shot>,
}

// TODO: maybe a little format module
fn show_float64(f: f64) -> String {
    format!("{:>4.2}", f)
}

fn show_string(s: String) -> String {
    // way too many copies here
    let mut c = s.clone();
    c.truncate(8);

    return c.trim().to_string();
}

impl Survey {
    /// Parse a survey from a string
    /// # Arguments
    /// input - A string containing the survey data
    /// # Returns
    /// Result containing the parsed survey or an error message
    /// # Errors
    /// If the input is not a valid survey
    pub fn parse_survey(input: &str) -> Result<Self, String> {
        match parser::parse_survey(input) {
            Ok((_, survey)) => Ok(survey),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Parse the contents of a survey.dat file
    /// # Arguments
    /// input - A string containing the contents of the survey.dat file
    /// # Returns
    /// Result containing the parsed survey or an error message
    /// # Errors
    /// If the input is not a valid survey.dat file
    pub fn parse_dat_file(input: &str) -> Result<Vec<Self>, Error> {
        match parser::parse_dat_file(input) {
            Ok((_, survey)) => Ok(survey),
            Err(e) => Err(Error::CouldntParseSurvey(e.to_string())),
        }
    }

    #[must_use]
    pub fn serialize(&self) -> String {
        // Some comments:
        // It looks like Rust's formatting doesn't let you specify what I would
        // call significant digits, only number of digits past the decimal point.
        // I *think* this is an issue since I think we want to 0 pad, but maybe
        // we just need to align on a width, which Rust does let you do
        //
        // Anyway, constraints I know about:
        // Numbers have 4 SD and 2 decimal points
        // Names get truncated to 12 characters or something?
        //
        // All of this is in the sample project / documentation, will get precise answers later
        let mut result = String::new();
        // let truncy = show_string(self.cave_name);
        result.push_str(&format!("{}\r\n", self.cave_name));
        result.push_str(&format!("SURVEY NAME: {}\n", self.name));
        result.push_str(&format!(
            "SURVEY DATE: {} {} {}",
            self.date.month, self.date.day, self.date.year
        ));
        if let Some(comment) = &self.comment {
            result.push_str(&format!("  COMMENT:{comment}\r\n"));
        } else {
            result.push_str("\r\n");
        }
        result.push_str("SURVEY TEAM:\r\n");
        result.push_str(&format!("{}\r\n", self.team));
        // TODO: fuss around with this one
        result.push_str(&self.parameters.serialize());
        result.push_str("\n         FROM           TO   LENGTH  BEARING      INC     LEFT       UP     DOWN    RIGHT   FLAGS  COMMENTS\n\n");
        for shot in &self.shots {
            // TODO: need to align these to end of their column names
            result.push_str(&format!(
                "{:>13}{:>13}{:>9.2}{:>9.2}{:>9.2}{:>9.2}{:>9.2}{:>9.2}{:>9.2}\n",
                shot.from,
                shot.to,
                shot.length,
                shot.azimuth,
                shot.inclination,
                shot.up,
                shot.down,
                shot.left,
                shot.right
            ));
        }
        result.push_str("\x0c\n");
        result
    }
}
