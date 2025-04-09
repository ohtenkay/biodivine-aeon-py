use pyo3::{
    types::{PyModule, PyModuleMethods},
    Bound, PyResult,
};
use regulation_constraint_config::RegulationConstraintConfig;
use regulation_constraint_impl::RegulationConstraint;

mod _impl_pyerr;
mod regulation_constraint_config;
mod regulation_constraint_error;
mod regulation_constraint_impl;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<RegulationConstraint>()?;
    module.add_class::<RegulationConstraintConfig>()?;
    Ok(())
}
