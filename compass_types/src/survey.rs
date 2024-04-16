use crate::common_types::Date;

mod parser;

pub struct CorrectionFactors {
    pub azimuth: f64,
    pub inclination: f64,
    pub length: f64,
}

pub struct BackSightCorrectionFactors {
    pub azimuth: f64,
    pub inclination: f64,
}

pub struct SurveyParameters {
    pub declination: f64,
    pub correction_factors: Option<CorrectionFactors>,
    pub backsight_correction_factors: Option<BackSightCorrectionFactors>,
}

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

pub struct Survey {
    pub cave_name: String,
    pub name: String,
    pub date: Date,
    pub comment: Option<String>,
    pub team: String,
    pub parameters: SurveyParameters,
}
