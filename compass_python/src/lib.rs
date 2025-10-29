use pyo3::{prelude::*, types::PyBytes};
use pyo3_stub_gen::{
    define_stub_info_gatherer,
    derive::{gen_stub_pyclass, gen_stub_pyfunction},
};
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
enum Units {
    Feet,
    Meters,
}

#[pyclass]
#[gen_stub_pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Shot {
    from: String,
    to: String,
    length: f32,
    azimuth: f32,
    depth: f32,
    left: Option<f32>,
    right: Option<f32>,
    up: Option<f32>,
    down: Option<f32>,
    flags: String,
    comment: String,
}

#[pyclass]
#[gen_stub_pyclass]
#[derive(Clone, Debug)]
struct SurveyData {
    survey_date: String,
    units: Units,
    cave_name: String,
    survey_name: String,
    survey_team: String,
    comment: String,
    latitude: f32,
    longitude: f32,
    shots: Vec<Shot>,
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
    m.add_class::<Units>()?;
    m.add_class::<Shot>()?;
    m.add_class::<SurveyData>()?;
    m.add_function(wrap_pyfunction!(convert_xls_json_to_dat, m)?)?;
    Ok(())
}

// Define a function to gather stub information.
define_stub_info_gatherer!(stub_info);
