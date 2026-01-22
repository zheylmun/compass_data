//! [![Static Badge](https://img.shields.io/badge/GitHub-gray?style=for-the-badge&logo=GitHub)](https://github.com/zheylmun/compass_data)
mod common_types;
mod error;
mod parser_utils;
mod project;
mod survey;
pub use common_types::{EastNorthElevation, UtmLocation};
pub use error::Error;
pub use project::{DatFile, Datum, Loaded, Project, Unloaded};
pub use survey::{BackSightCorrectionFactors, CorrectionFactors, Shot, Survey, SurveyParameters};

#[cfg(test)]
mod tests {

    use project::Project;

    use super::*;
    use std::path::{Path, PathBuf};

    fn test_at_path(project_path: &Path) {
        let unloaded_project = Project::read(&project_path).unwrap();
        // Make sure we can load all the project files too
        let _loaded_project = unloaded_project.load_survey_files().unwrap();
    }

    #[test]
    fn parse_compass_sample() {
        let mut sample_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        sample_path.push("test_data/Fulfords.mak");
        let unloaded_project = Project::read(&sample_path).unwrap();
        assert_eq!(unloaded_project.survey_files.len(), 2);
        // Make sure we can load all the project files too
        let _loaded_project = unloaded_project.load_survey_files().unwrap();
    }

    #[test]
    fn parse_environment_samples() {
        if let Ok(sample_dir) = std::env::var("COMPASS_DATA_SAMPLES") {
            println!("Sample directory: {}", sample_dir);
            let sample_dir = std::fs::read_dir(sample_dir).unwrap();
            for file_entry in sample_dir {
                let file_entry = file_entry.unwrap();
                let file_path = file_entry.path();
                if file_path.ends_with("DS_Store") {
                    continue;
                }

                println!("File path: {file_path:?}");

                if file_path.exists()
                    && file_path.is_file()
                    && let Some(extension) = file_path.extension()
                {
                    if extension.to_ascii_lowercase().to_str().unwrap() == "mak" {
                        test_at_path(&file_path);
                    }
                }
                {}
            }
        }
    }
}
