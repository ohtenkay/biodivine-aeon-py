use pyo3::{create_exception, exceptions::PyException, PyErr};

use crate::bindings::algorithms::{
    cancellation::CancelledError, fixed_points::fixed_points_error::FixedPointsError,
};

impl From<FixedPointsError> for PyErr {
    fn from(err: FixedPointsError) -> Self {
        match err {
            FixedPointsError::Cancelled(x) => PyErr::new::<CancelledError, _>(format!(
                "Cancelled(partial_result={})",
                x.exact_cardinality()
            )),
            FixedPointsError::CancelledBdd(x) => PyErr::new::<CancelledError, _>(format!(
                "CancelledBdd(partial_result={})",
                x.exact_cardinality()
            )),
        }
    }
}

// TODO: docs - add fourth argument, documentation
create_exception!(fixed_points, BddSizeLimitExceededError, PyException);
