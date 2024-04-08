mod parser;

pub struct Survey {
    cave_name: String,
    survey_name: String,
    survey_date: String,
    survey_comment: String,
    survey_team: String,
    declination: f64,
    correction_factor_azimuth: f64,
    correction_factor_inclination: f64,
    correction_factor_length: f64,
    back_sight_correction_factor_azimuth: f64,
    back_sight_correction_factor_inclination: f64,
}
