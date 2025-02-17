use pyo3::{
    types::{PyModule, PyModuleMethods},
    Bound, PyResult,
};
use reachability_config::ReachabilityConfig;

mod reachability;
mod reachability_config;
mod reachability_error;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<ReachabilityConfig>()?;
    Ok(())
}
