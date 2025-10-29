use pyo3::{prelude::*, types::PyBytes};
use pyo3_stub_gen::{
    define_stub_info_gatherer,
    derive::{gen_stub_pyclass, gen_stub_pyfunction},
};
use serde::{Deserialize, Serialize};

#[pyclass]
#[gen_stub_pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Shot {
    pub from: String,
    pub to: String,
    pub length: f32,
    pub azimuth: f32,
    pub depth: f32,
    pub left: Option<f32>,
    pub right: Option<f32>,
    pub up: Option<f32>,
    pub down: Option<f32>,
    pub flags: String,
    pub comment: String,
}

#[pyclass]
#[gen_stub_pyclass]
#[derive(Clone, Debug)]
pub struct SurveyData {
    pub survey_date: String,
    pub units: String,
    pub cave_name: String,
    pub survey_name: String,
    pub survey_team: String,
    pub comment: String,
    pub shots: Vec<Shot>,
}

/// A Python-exposed function that returns fixed DAT file bytes.
#[gen_stub_pyfunction(module = "ospl_rcompass._rust_lib")]
#[pyfunction]
fn convert_xls_json_to_dat<'py>(
    py: Python<'py>,
    data: SurveyData,
) -> PyResult<Bound<'py, PyBytes>> {
    // Figure out the magnetic declination based on the survey date and location
    let parameters = compass_data::Parameters {
        declination: 0.0,
        correction_factors: None,
        backsight_correction_factors: None,
    };
    let date = chrono::NaiveDate::parse_from_str(&data.survey_date, "%Y-%m-%d").unwrap();
    let comment = match data.comment.is_empty() {
        true => None,
        false => Some(data.comment),
    };

    let compass_survey = compass_data::Survey {
        cave_name: data.cave_name,
        name: data.survey_name,
        date,
        team: data.survey_team,
        comment,
        parameters,
        shots: vec![],
    };
    let serialized = compass_survey.serialize();
    let bytes = serialized.as_bytes();
    // Return a PyBytes object (Bound<'py, PyBytes>)
    Ok(PyBytes::new(py, &bytes))
}

/// A Python module implemented in Rust.
#[pymodule]
fn _rust_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Shot>()?;
    m.add_class::<SurveyData>()?;
    m.add_function(wrap_pyfunction!(convert_xls_json_to_dat, m)?)?;
    Ok(())
}

// Define a function to gather stub information.
define_stub_info_gatherer!(stub_info);
