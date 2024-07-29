//! Compass project
//!
//! This module provides the ability to read, write, and work with Compass project files
//! Compass project files are stored in a flexible makefile format
//! The compass file format is documented here:
//! [Compass Project Documentation](https://www.fountainware.com/compass/HTML_Help/Project_Manager/projectfileformat.htm)

mod parser;
use std::path::{Path, PathBuf};

use crate::{EastNorthUp, Error, Survey, UtmLocation};

/// Compass projects can be defined in a variety of geodetic datums.
/// The datum is used to convert between the geodetic coordinates used in the survey data.
/// This enum provides a list of the datums supported by Compass.
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
pub struct Station {
    name: String,
    location: Option<EastNorthUp>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurveyFile {
    pub file_path: String,
    pub project_stations: Vec<Station>,
    pub surveys: Vec<Survey>,
}

pub struct Unloaded;
pub struct Loaded;
pub struct Validated;

pub struct Project<S> {
    pub file_path: PathBuf,
    pub base_location: UtmLocation,
    pub datum: Datum,
    /// The UTM zone used for fixed stations in the project
    pub utm_zone: Option<u8>,
    pub survey_files: Vec<SurveyFile>,
    pub state: S,
}

impl Project<Unloaded> {
    /// Read a Compass project file from disk
    /// The project file is read from disk and parsed into a `ProjectFile` struct,
    /// but this does not parse the referenced survey data files
    /// # Returns
    /// `ProjectFile` representing the contents of the project file
    /// # Errors
    /// - [`Error::ProjectFileNotFound`] If the file does not exist
    /// - [`Error::CouldntReadFile`] If the file cannot be read
    pub fn read(file_path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = file_path.as_ref().to_path_buf();
        if !path.exists() {
            return Err(Error::ProjectFileNotFound(path));
        }
        let file_contents = std::fs::read_to_string(&path)?;
        let (_, project) = parser::parse_compass_project(path, &file_contents)
            .map_err(|e| Error::CouldntParseProject(e.to_string()))?;
        Ok(project)
    }
}

impl Project<Loaded> {
    /// Programmatically create a new compass project
    ///
    #[must_use]
    pub fn new(
        file_path: impl AsRef<Path>,
        base_location: UtmLocation,
        datum: Datum,
        utm_zone: Option<u8>,
    ) -> Self {
        let file_path = file_path.as_ref().to_path_buf();
        Self {
            file_path,
            base_location,
            datum,
            utm_zone,
            survey_files: Vec::new(),
            state: Loaded,
        }
    }

    /// Validate the project
    /// Ensure tha all from-stations in the survey files are present in the project and available for reference
    /// # Errors
    /// - [`Error::StationNotFound`] If a station referenced in a survey file is not available
    pub fn validate(self) -> Result<Project<Validated>, Error> {
        //Validate stations in here
        Ok(Project {
            file_path: self.file_path,
            base_location: self.base_location,
            datum: self.datum,
            utm_zone: self.utm_zone,
            survey_files: self.survey_files,
            state: Validated,
        })
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
        let result = Project::read(&path);
        assert!(result.is_err_and(|err| matches!(err, Error::ProjectFileNotFound(_path))));
    }

    #[test]
    fn parse_compass_sample() {
        let mut sample_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        sample_path.push("test_data/Fulfords.mak");

        let loaded_project = Project::read(&sample_path).unwrap();
        assert_eq!(loaded_project.survey_files.len(), 2);
    }
}
