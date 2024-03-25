mod project;
pub use project::{parse_compass_project, Datum, Project, SurveyDataFile, UtmLocation};

const FEET_TO_METERS: f64 = 0.3048;
const METERS_TO_FEET: f64 = 1.0 / FEET_TO_METERS;
