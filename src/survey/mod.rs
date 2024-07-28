use crate::common_types::Date;

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
    fn serialize(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("DECLINATION: {:.2}", self.declination));
        if let Some(correction_factors) = &self.correction_factors {
            result.push_str(&format!(
                "CORRECTIONS: {:.1} {:.1} {:.1}",
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
    pub fn parse_dat_file(input: &str) -> Result<Vec<Self>, String> {
        match parser::parse_dat_file(input) {
            Ok((_, survey)) => Ok(survey),
            Err(e) => Err(e.to_string()),
        }
    }

    #[must_use]
    pub fn serialize(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("{}\r\n", self.cave_name));
        result.push_str(&format!("SURVEY NAME: {}\n", self.name));
        result.push_str(&format!(
            "SURVEY DATE: {} {} {}",
            self.date.month, self.date.day, self.date.year
        ));
        if let Some(comment) = &self.comment {
            result.push_str(&format!(" COMMENT: {comment}\r\n"));
        } else {
            result.push_str("\r\n");
        }
        result.push_str("SURVEY TEAM:\r\n");
        result.push_str(&format!("{}\r\n", self.team));
        result.push_str(&self.parameters.serialize());
        result.push_str("FROM\n");
        for shot in &self.shots {
            result.push_str(&format!(
                "{} {} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2}\n",
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
