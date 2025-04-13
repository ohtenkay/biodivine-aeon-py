use std::time::Duration;

use biodivine_lib_param_bn::{symbolic_async_graph::SymbolicAsyncGraph, BooleanNetwork};
use pyo3::{pyclass, pymethods, PyResult};

use crate::{
    bindings::{
        algorithms::{
            cancellation::{
                tokens::{CancelTokenPython, CancelTokenTimer},
                CancellationHandler,
            },
            percolation::percolation_error::PercolationError,
        },
        lib_param_bn::{
            boolean_network::BooleanNetwork as BooleanNetworkBinding,
            symbolic::asynchronous_graph::AsynchronousGraph,
        },
    },
    AsNative,
};

#[pyclass(module = "biodivine_aeon", frozen)]
#[derive(Clone)]
pub struct PercolationConfig {
    /// The symbolic graph whose variables will be used for subspace percolation.
    pub graph: SymbolicAsyncGraph,

    /// A `CancellationHandler` that can be used to stop the algorithm externally.
    ///
    /// Default: [CancelTokenNever].
    pub cancellation: Box<dyn CancellationHandler>,
}

impl PercolationConfig {
    /// Create a new "default" [PercolationConfig] from the given [BooleanNetwork].
    pub fn from_boolean_network(bn: &BooleanNetwork) -> Result<Self, PercolationError> {
        let graph = SymbolicAsyncGraph::new(bn).map_err(PercolationError::CreationFailed)?;

        Ok(PercolationConfig {
            graph,
            cancellation: Default::default(),
        })
    }

    /// Create a new "default" [PercolationConfig] for the given [SymbolicAsyncGraph].
    pub fn with_graph(graph: SymbolicAsyncGraph) -> Self {
        PercolationConfig {
            graph,
            cancellation: Default::default(),
        }
    }

    /// Update the `cancellation` property, automatically wrapping the [CancellationHandler]
    /// in a `Box`.
    pub fn with_cancellation<C: CancellationHandler + 'static>(mut self, cancellation: C) -> Self {
        self.cancellation = Box::new(cancellation);
        self
    }
}

#[pymethods]
impl PercolationConfig {
    #[new]
    #[pyo3(signature = (graph, time_limit_millis = None))]
    pub fn new_py(graph: &AsynchronousGraph, time_limit_millis: Option<u64>) -> Self {
        let mut config = PercolationConfig::with_graph(graph.as_native().clone());

        if let Some(millis) = time_limit_millis {
            config = config.with_cancellation(CancelTokenPython::with_inner(CancelTokenTimer::new(
                Duration::from_millis(millis),
            )))
        }

        config
    }

    #[staticmethod]
    #[pyo3(name = "from_boolean_network")]
    pub fn from_boolean_network_py(bn: &BooleanNetworkBinding) -> PyResult<Self> {
        Ok(PercolationConfig::from_boolean_network(bn.as_native())?)
    }

    #[staticmethod]
    #[pyo3(name = "with_graph")]
    pub fn with_graph_py(graph: &AsynchronousGraph) -> Self {
        PercolationConfig::with_graph(graph.as_native().clone())
    }

    #[pyo3(name = "with_time_limit")]
    pub fn with_time_limit_py(&self, millis: u64) -> Self {
        self.clone()
            .with_cancellation(CancelTokenPython::with_inner(CancelTokenTimer::new(
                Duration::from_millis(millis),
            )))
    }
}
