mod parser;

use crate::{EastNorthUp, UtmLocation};

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
        let file_contents = std::fs::read_to_string(file_path).map_err(|e| e.to_string())?;
        let (_, project) =
            parser::parse_compass_project(&file_contents).map_err(|e| e.to_string())?;
        Ok(project)
    }
}
