mod parser;

use crate::{
    survey::{self, Survey},
    EastNorthUp, UtmLocation,
};

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
pub struct SurveyDataFile {
    file_path: String,
    fixed_stations: Vec<Station>,
    survey_data: Vec<Survey>,
}

pub struct Project {
    pub base_location: UtmLocation,
    pub datum: Datum,
    pub survey_data: Vec<SurveyDataFile>,
    pub utm_zone: Option<u8>,
}

impl Project {
    pub fn new(base_location: UtmLocation, datum: Datum) -> Self {
        Self {
            base_location,
            datum,
            survey_data: Vec::new(),
            utm_zone: None,
        }
    }

    pub fn load_project_file(file_path: &str) -> Result<Self, String> {
        let path = std::path::Path::new(file_path);
        if !path.exists() {
            return Err(format!("File not found: {}", file_path));
        }
        let path_root = path.parent().unwrap();
        let file_contents = std::fs::read_to_string(file_path).map_err(|e| e.to_string())?;
        let (_, mut project) =
            parser::parse_compass_project(&file_contents).map_err(|e| e.to_string())?;
        for file in project.survey_data.iter_mut() {
            let dat_path = path_root.join(&file.file_path);
            let dat_contents = std::fs::read_to_string(&dat_path).map_err(|e| e.to_string())?;
            let (_, mut survey_data) =
                survey::parser::parse_dat_file(&dat_contents).map_err(|e| e.to_string())?;
            file.survey_data.append(&mut survey_data);
        }
        Ok(project)
    }
}
