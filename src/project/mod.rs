//! Compass Project
//!
//! This module provides the ability to read, write, and work with Compass project files
//! Compass project files are stored in a flexible makefile format
//! The compass file format is documented here:
//! [Compass Project Documentation](https://www.fountainware.com/compass/HTML_Help/Project_Manager/projectfileformat.htm)
mod parser;

use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use crate::{EastNorthElevation, Error, Survey, UtmLocation};

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
    location: Option<EastNorthElevation>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Unloaded;
#[derive(Clone, Debug, PartialEq)]
pub struct Loaded;

#[derive(Clone, Debug, PartialEq)]
pub struct SurveyFile<S> {
    pub file_path: PathBuf,
    pub project_stations: Vec<Station>,
    surveys: Vec<Survey>,
    state: PhantomData<S>,
}

impl SurveyFile<Unloaded> {
    pub fn load(self, project_path: &Path) -> Result<SurveyFile<Loaded>, Error> {
        let full_path = project_path.join(&self.file_path);
        let file_contents = std::fs::read_to_string(&full_path)
            .map_err(|e| Error::CouldntReadFile(e, self.file_path.clone()))?;
        let surveys = Survey::parse_dat_file(&file_contents)?;
        Ok(SurveyFile {
            file_path: self.file_path,
            project_stations: self.project_stations,
            surveys,
            state: PhantomData,
        })
    }
}

pub struct Project<S> {
    pub file_path: PathBuf,
    pub base_location: UtmLocation,
    pub datum: Datum,
    /// The UTM zone used for fixed stations in the project
    pub utm_zone: Option<u8>,
    pub survey_files: Vec<SurveyFile<S>>,
    state: PhantomData<S>,
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
        let file_contents =
            std::fs::read_to_string(&path).map_err(|e| Error::CouldntReadFile(e, path.clone()))?;
        let (_, project) = parser::parse_compass_project(path, &file_contents)
            .map_err(|e| Error::CouldntParseProject(e.to_string()))?;
        Ok(project)
    }

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
            state: PhantomData::<Loaded>,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{common_types::EastNorthElevation, Error};

    use super::*;
    use std::path::PathBuf;
    #[test]
    fn programatic_creation() {
        let east_north_elevation = EastNorthElevation::from_meters(336_083.0, 3_301_724.0, 6.0);
        let new_project = Project::new(
            "Ginnie.mak",
            UtmLocation {
                east_north_elevation,
                zone: 17,
                convergence_angle: 1.257_286,
            },
            Datum::Wgs1984,
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
