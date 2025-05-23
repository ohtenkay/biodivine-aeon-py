use pyo3::{
    types::{PyModule, PyModuleMethods as _},
    Bound, PyResult,
};

use super::{Percolation, PercolationConfig};

mod _impl_pyerr;
mod percolation_config_python;
mod percolation_impl_python;
mod subspace_representation;

pub use subspace_representation::SubspaceRepresentation;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Percolation>()?;
    module.add_class::<PercolationConfig>()?;

    Ok(())
}
