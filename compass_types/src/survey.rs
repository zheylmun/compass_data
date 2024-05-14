use crate::common_types::Date;

pub mod parser;

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
pub struct SurveyParameters {
    pub declination: f64,
    pub correction_factors: Option<CorrectionFactors>,
    pub backsight_correction_factors: Option<BackSightCorrectionFactors>,
}

impl SurveyParameters {
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
    pub parameters: SurveyParameters,
    pub shots: Vec<Shot>,
}

impl Survey {
    pub fn serialize(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("{}\r\n", self.cave_name));
        result.push_str(&format!("SURVEY NAME: {}\n", self.name));
        result.push_str(&format!(
            "SURVEY DATE: {} {} {}",
            self.date.month, self.date.day, self.date.year
        ));
        if let Some(comment) = &self.comment {
            result.push_str(&format!(" COMMENT: {}\r\n", comment));
        } else {
            result.push_str("\r\n");
        }
        result.push_str(&format!("SURVEY TEAM:\r\n"));
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

pub fn parse_survey(input: &str) -> Result<Survey, String> {
    match parser::parse_survey(input) {
        Ok((_, survey)) => Ok(survey),
        Err(e) => Err(e.to_string()),
    }
}
