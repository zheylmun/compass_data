//! [![Static Badge](https://img.shields.io/badge/GitHub-gray?style=for-the-badge&logo=GitHub)](https://github.com/zheylmun/compass_data)
mod common_types;
mod error;
mod parser_utils;
mod project;
mod survey;
pub use common_types::{EastNorthElevation, UtmLocation};
pub use error::Error;
pub use project::{Datum, Project, SurveyFile};
pub use survey::{BackSightCorrectionFactors, CorrectionFactors, Parameters, Shot, Survey};

#[cfg(test)]
mod tests {

    use project::Project;

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_compass_sample() {
        let mut sample_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        sample_path.push("test_data/Fulfords.mak");

        let loaded_project = Project::read(&sample_path).unwrap();
        assert_eq!(loaded_project.survey_files.len(), 2);
    }
}
