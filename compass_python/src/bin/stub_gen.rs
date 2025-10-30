use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = ospl_rcompass::stub_info()?;
    stub.generate()?;
    Ok(())
}
