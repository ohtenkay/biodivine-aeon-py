use std::fmt::Debug;

use biodivine_lib_bdd::Bdd;
use biodivine_lib_param_bn::symbolic_async_graph::GraphColoredVertices;
use thiserror::Error;

use crate::bindings::algorithms::cancellation::CancellationError;

// TODO: ohtenkay - impl Debug for FixedPointsError
/// An error returned by a [FixedPoints] procedure.
// TODO: ohtenkay - by default GraphColoredVertices is restriction, in the future, consider making it more precise
#[derive(Error, Debug)]
pub enum FixedPointsError {
    #[error("operation cancelled")]
    CancelledBdd(Bdd),
    #[error("operation cancelled")]
    Cancelled(GraphColoredVertices),
    // #[error("BDD size limit exceeded")]
    // BddSizeLimitExceeded(GraphColoredVertices),
}

impl From<CancellationError<GraphColoredVertices>> for FixedPointsError {
    fn from(value: CancellationError<GraphColoredVertices>) -> Self {
        FixedPointsError::Cancelled(value.into_partial_data())
    }
}

// TODO: discuss - in order to create GraphColoredVertices from Bdd, we need to
// use the SymbolicAsyncGraph. This would require storing it in the CancellationError, if we want
// to avoid creating it before calling every is_cancelled!() macro.
impl From<CancellationError<Bdd>> for FixedPointsError {
    fn from(value: CancellationError<Bdd>) -> Self {
        FixedPointsError::CancelledBdd(value.into_partial_data())
    }
}

// impl<T> From<CancellationError<T>> for FixedPointsError
// where
//     T: Sized + Debug + 'static,
//     T: !GraphColoredVertices,
// {
//     fn from(_: CancellationError<T>) -> Self {
//         FixedPointsError::CancelledEmpty
//     }
// }
