mod common_types;
mod parser_utils;
mod project;
mod survey;
pub use common_types::{EastNorthUp, UtmLocation};
pub use project::{Datum, Project, SurveyDataFile};
pub use survey::{
    parse_survey, BackSightCorrectionFactors, CorrectionFactors, Shot, Survey, SurveyParameters,
};

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_compass_sample() {
        let mut sample_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        sample_path.push("test_data/Fulfords.mak");
        let canonicalized_path = sample_path.canonicalize().unwrap();

        let loaded_project = Project::load_project(&canonicalized_path).unwrap();
        assert_eq!(loaded_project.survey_data.len(), 2);
    }
}
