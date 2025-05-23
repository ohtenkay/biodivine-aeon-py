use pyo3::{create_exception, exceptions::PyException, PyErr};

use crate::bindings::algorithms::{
    cancellation::CancelledError, graph_representation::CreationFailedError,
};

use super::FixedPointsError;

impl From<FixedPointsError> for PyErr {
    fn from(err: FixedPointsError) -> Self {
        match err {
            FixedPointsError::CreationFailed(error) => {
                PyErr::new::<CreationFailedError, _>(format!("Config creation failed: {}", error))
            }
            FixedPointsError::Cancelled(bdd) => PyErr::new::<CancelledError, _>(format!(
                "Cancelled: partial_result={}",
                bdd.exact_cardinality()
            )),
            FixedPointsError::BddSizeLimitExceeded(bdd) => {
                PyErr::new::<BddSizeLimitExceededError, _>(format!(
                    "BDD size limit exceeded: {}",
                    bdd.exact_cardinality()
                ))
            }
        }
    }
}

create_exception!(fixed_points, BddSizeLimitExceededError, PyException);
