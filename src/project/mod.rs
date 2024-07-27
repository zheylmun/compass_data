//! Compass project
//!
//! This module provides the ability to read, write, and work with Compass project files
//! Compass project files are stored in a flexible makefile format
//! The compass file format is documented here:
//! https://www.fountainware.com/compass/HTML_Help/Project_Manager/projectfileformat.htm

mod parser;

use crate::{EastNorthUp, Error, UtmLocation};
use std::path::Path;

/// Compass projects can be defined in a variety of geodetic datums
/// The datum is used to convert between the geodetic coordinates used in the survey data
/// This enum provides a list of the datums supported by Compass
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Datum {
    Adindan,
    Arc1950,
    Arc1960,
    Australian1966,
    Australian1984,
    CampAreaAstro,
    Cape,
    European1950,
    European1979,
    Geodetic1949,
    HongKong1963,
    HuTzuShan,
    Indian,
    NorthAmerican1927,
    NorthAmerican1983,
    Oman,
    OrdinanceSurvey1936,
    Pulkovo1942,
    SouthAmerican1956,
    SouthAmerican1969,
    Tokyo,
    Wgs1972,
    Wgs1984,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectStation {
    name: String,
    location: Option<EastNorthUp>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurveyDataFile {
    file_path: String,
    project_stations: Vec<ProjectStation>,
}

pub struct ProjectFile {
    pub base_location: UtmLocation,
    pub datum: Datum,
    /// Information about the survey data files included in the project
    pub survey_data_files: Vec<SurveyDataFile>,
    /// The UTM zone used for fixed stations in the project
    pub utm_zone: Option<u8>,
}

impl ProjectFile {
    pub fn new(base_location: UtmLocation, datum: Datum, utm_zone: Option<u8>) -> Self {
        Self {
            base_location,
            datum,
            survey_data_files: Vec::new(),
            utm_zone,
        }
    }

    pub fn read(file_path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = file_path.as_ref();
        if !path.exists() {
            return Err(Error::ProjectFileNotFound(path.into()));
        }
        let file_contents = std::fs::read_to_string(path)?;
        let (_, project) = parser::parse_compass_project(&(*file_contents.clone()))
            .map_err(|e| Error::CouldntParseProject(e.to_string()))?;
        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    use super::*;
    use std::path::PathBuf;
    #[test]
    fn bad_path() {
        let path = PathBuf::from("does_not_exist.mak");
        let result = ProjectFile::read(&path);
        assert!(result.is_err_and(|err| matches!(err, Error::ProjectFileNotFound(path))));
    }

    #[test]
    fn parse_compass_sample() {
        let mut sample_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        sample_path.push("test_data/Fulfords.mak");

        let loaded_project = ProjectFile::read(&sample_path).unwrap();
        assert_eq!(loaded_project.survey_data_files.len(), 2);
    }
}
