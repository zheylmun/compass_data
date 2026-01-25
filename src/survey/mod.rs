use std::fmt::Write;

use chrono::{Datelike, NaiveDate};

use crate::Error;

mod parser;

/// Bearing Units are used to represent heading measurements
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BearingUnits {
    Degrees,
    Quads,
    Grads,
}

/// Length Units are used to represent distance measurements
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LengthUnits {
    DecimalFeet,
    FeetAndInches,
    Meters,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum InclinationUnits {
    Degrees,
    PercentGrade,
    DegreesAndMinutes,
    Grads,
    DepthGauge,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PassageDimension {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ShotItem {
    Length,
    Azimuth,
    Inclination,
    BackAzimuth,
    BackInclination,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum RedundantBackSight {
    RedundantBacksight,
    NoRedundantBacksight,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LRUDAssociation {
    FromStation,
    ToStation,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SurveyFormat11 {
    bearing_units: BearingUnits,
    length_units: LengthUnits,
    passage_units: LengthUnits,
    inclination_units: InclinationUnits,
    passage_dimension_order: [PassageDimension; 4],
    shot_item_order: [ShotItem; 3],
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SurveyFormat12 {
    bearing_units: BearingUnits,
    length_units: LengthUnits,
    passage_units: LengthUnits,
    inclination_units: InclinationUnits,
    passage_dimension_order: [PassageDimension; 4],
    shot_item_order: [ShotItem; 3],
    backsight: RedundantBackSight,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SurveyFormat13 {
    bearing_units: BearingUnits,
    length_units: LengthUnits,
    passage_units: LengthUnits,
    inclination_units: InclinationUnits,
    passage_dimension_order: [PassageDimension; 4],
    shot_item_order: [ShotItem; 3],
    backsight: RedundantBackSight,
    lrud_association: LRUDAssociation,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SurveyFormat15 {
    bearing_units: BearingUnits,
    length_units: LengthUnits,
    passage_units: LengthUnits,
    inclination_units: InclinationUnits,
    passage_dimension_order: [PassageDimension; 4],
    shot_item_order: [ShotItem; 5],
    backsight: RedundantBackSight,
    lrud_association: LRUDAssociation,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SurveyFormat {
    None,
    Format11(SurveyFormat11),
    Format12(SurveyFormat12),
    Format13(SurveyFormat13),
    Format15(SurveyFormat15),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct CorrectionFactors {
    pub azimuth: f64,
    pub inclination: f64,
    pub length: f64,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BackSightCorrectionFactors {
    pub azimuth: f64,
    pub inclination: f64,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SurveyParameters {
    pub declination: f64,
    pub format_parameters: SurveyFormat,
    pub correction_factors: Option<CorrectionFactors>,
    pub backsight_correction_factors: Option<BackSightCorrectionFactors>,
}

impl SurveyParameters {
    fn serialize(&self) -> String {
        let mut result = String::new();
        let _ = write!(result, "DECLINATION:   {:>4.2}  ", self.declination);
        // TODO: serialize format_parameters when needed
        if let Some(cf) = &self.correction_factors {
            let _ = write!(
                result,
                "CORRECTIONS:  {:.2} {:.2} {:.2}",
                cf.azimuth, cf.inclination, cf.length
            );
        }
        if let Some(bcf) = &self.backsight_correction_factors {
            let _ = write!(
                result,
                "CORRECTIONS2: {:.1} {:.1}",
                bcf.azimuth, bcf.inclination
            );
        }
        result.push_str("\r\n");
        result
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Survey {
    pub cave_name: String,
    pub name: String,
    pub date: NaiveDate,
    pub comment: Option<String>,
    pub team: String,
    pub parameters: SurveyParameters,
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
    pub fn parse_dat_file(input: &str) -> Result<Vec<Self>, Error> {
        match parser::parse_dat_file(input) {
            Ok((_, survey)) => Ok(survey),
            Err(e) => Err(Error::CouldntParseSurvey(e.to_string())),
        }
    }

    #[must_use]
    pub fn serialize(&self) -> String {
        let mut result = String::new();
        let _ = writeln!(result, "{}\r", self.cave_name);
        let _ = writeln!(result, "SURVEY NAME: {}\r", self.name);
        let _ = write!(
            result,
            "SURVEY DATE: {} {} {}",
            self.date.month(),
            self.date.day0() + 1,
            self.date.year_ce().1,
        );
        if let Some(comment) = &self.comment {
            let _ = writeln!(result, "  COMMENT:{comment}\r");
        } else {
            result.push_str("\r\n");
        }
        result.push_str("SURVEY TEAM:\r\n");
        let _ = writeln!(result, "{}\r", self.team);
        result.push_str(&self.parameters.serialize());
        result.push_str("\n        FROM           TO   LENGTH  BEARING      INC     LEFT       UP     DOWN    RIGHT   FLAGS  COMMENTS\r\n\r\n");
        for shot in &self.shots {
            let _ = writeln!(
                result,
                "{:>12}{:>13}{:>9.2}{:>9.2}{:>9.2}{:>9.2}{:>9.2}{:>9.2}{:>9.2}\r",
                shot.from,
                shot.to,
                shot.length,
                shot.azimuth,
                shot.inclination,
                shot.up,
                shot.down,
                shot.left,
                shot.right
            );
        }
        result.push_str("\x0c\r\n");
        result
    }
}
