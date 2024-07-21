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

    #[test]
    fn parse_compass_sample() {
        let input = "../test_data/Fulfords.mak";
        let loaded_project = Project::load_project_file(input).unwrap();
        assert_eq!(loaded_project.survey_data.len(), 2);
    }
}
