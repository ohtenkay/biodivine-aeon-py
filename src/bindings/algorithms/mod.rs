use pyo3::{types::PyModule, Bound, PyResult};

pub mod cancellation;
pub mod configurable;
pub mod fixed_points;
pub mod percolation;
pub mod reachability;
pub mod trap_spaces;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    fixed_points::register(module)?;
    trap_spaces::register(module)?;
    percolation::register(module)?;
    reachability::register(module)?;
    trap_spaces::register(module)?;

    Ok(())
}
