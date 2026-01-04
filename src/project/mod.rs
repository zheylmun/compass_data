//! Compass Project
//!
//! This module provides the ability to read, write, and work with Compass project files
//! Compass project files are stored in a makefile format
//! The compass file format is documented here:
//! [Compass Project Documentation](https://www.fountainware.com/compass/HTML_Help/Project_Manager/projectfileformat.htm)
//!
mod parser;

use crate::{EastNorthElevation, Error, Survey, UtmLocation};
use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};
use uuid::Uuid;

/// Compass projects can be defined in a variety of geodetic datums.
///
/// The datum is used to convert between the geodetic coordinates used in the survey data.
/// This enum provides a list of the datums supported by Compass.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
    WGS1972,
    WGS1984,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Station {
    name: String,
    location: Option<EastNorthElevation>,
}

/// Marker type for survey and project files which have not been fully loaded yet
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Unloaded;
/// Marker type for survey and project files which have been fully loaded
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Loaded;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct DatFile<S> {
    pub file_path: PathBuf,
    pub project_stations: Vec<Station>,
    surveys: Vec<Survey>,
    state: PhantomData<S>,
}

impl DatFile<Unloaded> {
    /// Load the survey data file from disk
    /// Consumes the `SurveyFile<Unloaded>` and returns a `SurveyFile<Loaded>` with the survey data populated
    /// # Returns
    /// `SurveyFile<Loaded>` representing the contents of the project file
    /// # Errors
    /// - [`Error::SurveyFileNotFound`] If the file does not exist
    /// - [`Error::CouldntReadFile`] If the file cannot be read
    pub fn load(self, project_path: &Path) -> Result<DatFile<Loaded>, Error> {
        let full_path = project_path.join(&self.file_path);
        if !full_path.exists() {
            return Err(Error::SurveyFileNotFound(full_path));
        }
        let file_contents = std::fs::read_to_string(&full_path).map_err(Error::CouldntReadFile)?;
        let surveys = Survey::parse_dat_file(&file_contents)?;
        Ok(DatFile {
            file_path: self.file_path,
            project_stations: self.project_stations,
            surveys,
            state: PhantomData,
        })
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Project<S> {
    pub id: Option<Uuid>,
    /// The file path to the project file on disk
    /// This is used to resolve relative paths to survey data files
    /// # Note
    /// igoned for equality checks
    pub file_path: PathBuf,
    pub base_location: UtmLocation,
    pub datum: Datum,
    /// The UTM zone used for fixed stations in the project
    pub utm_zone: Option<u8>,
    pub survey_files: Vec<DatFile<S>>,
    state: PhantomData<S>,
}

impl PartialEq for Project<Unloaded> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.base_location == other.base_location
            && self.datum == other.datum
            && self.utm_zone == other.utm_zone
            && self.survey_files == other.survey_files
    }
}

impl PartialEq for Project<Loaded> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.base_location == other.base_location
            && self.datum == other.datum
            && self.utm_zone == other.utm_zone
            && self.survey_files == other.survey_files
    }
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
        let file_contents = std::fs::read_to_string(&path).map_err(Error::CouldntReadFile)?;
        let (_, project) = parser::parse_compass_project(path, &file_contents)
            .map_err(|e| Error::CouldntParseProject(e.to_string()))?;
        Ok(project)
    }

    /// Read a Compass project's survey data files from disk
    /// The data files are read from the paths specified in the project file
    /// # Returns
    /// [`Project<Loaded>`] representing the project file, complete with survey data
    /// # Errors
    /// - [`Error::SurveyFileNotFound`] If a listed survey file does not exist
    /// - [`Error::CouldntReadFile`] If the file cannot be read
    /// - [`Error::CouldntParseSurvey`] If the survey file cannot be parsed
    #[allow(clippy::missing_panics_doc)]
    pub fn load_survey_files(self) -> Result<Project<Loaded>, Error> {
        let mut survey_files = Vec::new();
        // This unwrap is safe because we know the file path existed to read this project
        // therefore the parent directory must exist
        let project_dir = self.file_path.parent().unwrap();
        for survey_file in self.survey_files {
            let survey_file = survey_file.load(project_dir)?;
            survey_files.push(survey_file);
        }
        Ok(Project {
            id: self.id,
            file_path: self.file_path,
            base_location: self.base_location,
            datum: self.datum,
            utm_zone: self.utm_zone,
            survey_files,
            state: PhantomData::<Loaded>,
        })
    }
}

impl Project<Loaded> {
    /// Programmatically create a new compass project
    #[must_use]
    pub fn new(
        project_id: Option<Uuid>,
        file_path: impl AsRef<Path>,
        base_location: UtmLocation,
        datum: Datum,
        utm_zone: Option<u8>,
    ) -> Self {
        let file_path = file_path.as_ref().to_path_buf();
        Self {
            id: project_id,
            file_path,
            base_location,
            datum,
            utm_zone,
            survey_files: Vec::new(),
            state: PhantomData::<Loaded>,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, common_types::EastNorthElevation};

    use super::*;
    use std::path::PathBuf;
    #[test]
    fn programatic_creation() {
        let east_north_elevation = EastNorthElevation::from_meters(336_083.0, 3_301_724.0, 6.0);
        let new_project = Project::new(
            None,
            "Ginnie.mak",
            UtmLocation {
                east_north_elevation,
                zone: 17,
                convergence_angle: 1.257_286,
            },
            Datum::WGS1984,
            None,
        );
        assert!(new_project.survey_files.is_empty());
    }

    #[test]
    fn bad_path() {
        let path = PathBuf::from("does_not_exist.mak");
        let result = Project::read(&path);
        assert!(result.is_err_and(|err| matches!(err, Error::ProjectFileNotFound(_path))));
    }

    #[test]
    fn parse_and_load_compass_sample() {
        let mut sample_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        sample_path.push("test_data/Fulfords.mak");

        let read_project = Project::read(&sample_path).unwrap();
        assert_eq!(read_project.survey_files.len(), 2);
        let _loaded_project = read_project.load_survey_files().unwrap();
    }
}
