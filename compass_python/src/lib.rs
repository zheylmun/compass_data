use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::types::PyDict;
use pyo3::types::PyModule;

use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

/// A Python-exposed function that returns fixed DAT file bytes.
#[gen_stub_pyfunction(module = "ospl_rcompass._rust_lib")]
#[pyfunction]
fn convert_xls_json_to_dat<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyBytes>> {
    let mut bytes = br#"Fulford Cave
SURVEY NAME: SS
SURVEY DATE: 8 28 1988  COMMENT:Surface to shelter
SURVEY TEAM:
Mike Roberts,Ken Kreager,Rick Rhinehart, ,
DECLINATION:   11.18  FORMAT: DDDDUDLRLADN  CORRECTIONS:  0.00 0.00 0.00

        FROM           TO   LENGTH  BEARING      INC     LEFT       UP     DOWN    RIGHT   FLAGS  COMMENTS

          A1          SS1    62.45   104.00    34.50 -9999.00 -9999.00 -9999.00 -9999.00  #|P#
         SS1          SS2    35.35   120.50    22.00 -9999.00 -9999.00 -9999.00 -9999.00  #|P#
         SS2          SS3    25.35   150.50    10.50 -9999.00 -9999.00 -9999.00 -9999.00  #|P#
         SS3          SS4    67.20   117.00    29.50 -9999.00 -9999.00 -9999.00 -9999.00  #|P#
         SS4          SS5    60.10   123.50    16.00 -9999.00 -9999.00 -9999.00 -9999.00  #|P#
         SS5          SS6    54.50   112.00    11.00 -9999.00 -9999.00 -9999.00 -9999.00  #|P#
         SS6          SS7    36.30    89.00    21.00 -9999.00 -9999.00 -9999.00 -9999.00
         SS6          SS8    41.70   333.50    -2.50 -9999.00 -9999.00 -9999.00 -9999.00
"#.to_vec();

    // Append a literal form-feed (0x0C)
    bytes.push(0x0C);

    // Return a PyBytes object (Bound<'py, PyBytes>)
    Ok(PyBytes::new(py, &bytes))
}

/// A Python module implemented in Rust.
#[pymodule]
fn _rust_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert_xls_json_to_dat, m)?)?;
    Ok(())
}

// Define a function to gather stub information.
define_stub_info_gatherer!(stub_info);
