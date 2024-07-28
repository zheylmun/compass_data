use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Project file not found: {0}")]
    ProjectFileNotFound(PathBuf),
    #[error("Error reading file: {0}")]
    CouldntReadFile(#[from] std::io::Error),
    #[error("Error parsing project file: {0}")]
    CouldntParseProject(String),
    #[error("Error parsing survey data file: {0}")]
    CouldntParseSurveyData(String),
    #[error("Error parsing Survey: {0}")]
    CouldntParseSurvey(String),
}
