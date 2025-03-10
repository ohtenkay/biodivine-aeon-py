use pyo3::{
    create_exception,
    exceptions::{
        asyncio::{CancelledError, InvalidStateError},
        PyException,
    },
    PyErr,
};

use crate::bindings::algorithms::reachability::ReachabilityError;

impl From<ReachabilityError> for PyErr {
    fn from(err: ReachabilityError) -> Self {
        match err {
            ReachabilityError::Cancelled(x) => PyErr::new::<CancelledError, _>(format!(
                "Cancelled(partial_result={})",
                x.exact_cardinality()
            )),
            ReachabilityError::StepsLimitExceeded(x) => {
                PyErr::new::<StepsLimitExceededError, _>(format!(
                    "StepsLimitExceeded(partial_result={})",
                    x.exact_cardinality()
                ))
            }
            ReachabilityError::BddSizeLimitExceeded(x) => {
                PyErr::new::<BddSizeLimitExceededError, _>(format!(
                    "BddSizeLimitExceeded(partial_result={})",
                    x.exact_cardinality()
                ))
            }
            ReachabilityError::InvalidSubgraph => {
                PyErr::new::<InvalidStateError, _>("InvalidSubgraph")
            }
        }
    }
}

// TODO: ohtenkay - think about the module name, maybe make it more specific,
// TODO: ohtenkay - add fourth argument, documentation
// TODO: ohtenkay - consider creating all the reachability errors with this macro
create_exception!(bindings, BddSizeLimitExceededError, PyException);
create_exception!(bindings, StepsLimitExceededError, PyException);
