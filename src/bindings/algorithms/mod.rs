use pyo3::{types::PyModule, Bound, PyResult};

pub mod cancellation;
pub mod fixed_points;
pub mod reachability;
pub mod regulation_constraint;
pub mod trap_spaces;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    reachability::register(module)?;
    fixed_points::register(module)?;
    trap_spaces::register(module)?;
    regulation_constraint::register(module)?;
    Ok(())
}
