use std::time::Duration;

use biodivine_lib_param_bn::symbolic_async_graph::{GraphColoredVertices, SymbolicAsyncGraph};
use pyo3::{pyclass, pymethods, Py};

use crate::{
    bindings::{
        algorithms::cancellation::{
            tokens::{CancelTokenPython, CancelTokenTimer},
            CancellationHandler,
        },
        lib_param_bn::symbolic::{
            asynchronous_graph::AsynchronousGraph, set_colored_vertex::ColoredVertexSet,
        },
    },
    AsNative,
};

#[pyclass(module = "biodivine_aeon", frozen)]
#[derive(Clone)]
pub struct FixedPointsConfig {
    pub graph: SymbolicAsyncGraph,
    pub restriction: GraphColoredVertices,
    pub cancellation: Box<dyn CancellationHandler>,
    // TODO: ohtenkay - discuss the rest of the fields.
}

impl FixedPointsConfig {
    /// Create a new "default" [FixedPointsCongfig] for the given [SymbolicAsyncGraph].
    pub fn with_graph(graph: SymbolicAsyncGraph) -> Self {
        FixedPointsConfig {
            restriction: graph.unit_colored_vertices().clone(),
            cancellation: Default::default(),
            graph,
        }
    }

    /// Update the `restriction` property
    pub fn with_restriction(mut self, restriction: GraphColoredVertices) -> Self {
        self.restriction = restriction;
        self
    }

    /// Update the `cancellation` property, automatically wrapping the [CancellationHandler]
    /// in a `Box`.
    pub fn with_cancellation<C: CancellationHandler + 'static>(mut self, cancellation: C) -> Self {
        self.cancellation = Box::new(cancellation);
        self
    }
}

impl CancellationHandler for FixedPointsConfig {
    fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    fn start_timer(&self) {
        self.cancellation.start_timer()
    }
}

#[pymethods]
impl FixedPointsConfig {
    #[new]
    #[pyo3(signature = (graph, restriction = None, time_limit_millis = None))]
    pub fn new_py(
        graph: Py<AsynchronousGraph>,
        restriction: Option<Py<ColoredVertexSet>>,
        time_limit_millis: Option<u64>,
    ) -> Self {
        let mut config = FixedPointsConfig::with_graph(graph.get().as_native().clone());

        if let Some(restriction) = restriction {
            config = config.with_restriction(restriction.get().as_native().clone())
        }

        if let Some(millis) = time_limit_millis {
            config = config.with_cancellation(CancelTokenPython::with_inner(CancelTokenTimer::new(
                Duration::from_millis(millis),
            )))
        }

        config
    }

    #[staticmethod]
    #[pyo3(name = "with_graph")]
    pub fn with_graph_py(graph: Py<AsynchronousGraph>) -> Self {
        FixedPointsConfig::with_graph(graph.get().as_native().clone())
            .with_cancellation(CancelTokenPython::default())
    }

    // TODO: if we ever move away from abi3-py37, use Duration as an argument
    #[pyo3(name = "with_time_limit")]
    pub fn with_time_limit_py(&self, duration_in_millis: u64) -> Self {
        self.clone()
            .with_cancellation(CancelTokenPython::with_inner(CancelTokenTimer::new(
                Duration::from_millis(duration_in_millis),
            )))
    }
}
