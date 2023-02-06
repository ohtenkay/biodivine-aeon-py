use crate::bindings::lib_bdd::PyBdd;
use crate::bindings::lib_param_bn::{PyGraphVertices, PySymbolicAsyncGraph, PyVariableId};
use crate::AsNative;
use biodivine_lib_bdd::Bdd;
use biodivine_lib_param_bn::biodivine_std::bitvector::BitVector;
use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::symbolic_async_graph::GraphVertices;
use pyo3::prelude::*;

impl From<PyGraphVertices> for GraphVertices {
    fn from(value: PyGraphVertices) -> Self {
        value.0
    }
}

impl From<GraphVertices> for PyGraphVertices {
    fn from(value: GraphVertices) -> Self {
        PyGraphVertices(value)
    }
}

impl AsNative<GraphVertices> for PyGraphVertices {
    fn as_native(&self) -> &GraphVertices {
        &self.0
    }

    fn as_native_mut(&mut self) -> &mut GraphVertices {
        &mut self.0
    }
}

#[pymethods]
impl PyGraphVertices {
    /// Get a copy of this set in the form of a `Bdd` object.
    pub fn to_bdd(&self) -> PyBdd {
        self.as_native().as_bdd().clone().into()
    }

    /// Populate a new `VertexSet` using a raw `Bdd`.
    pub fn copy_with(&self, bdd: PyBdd) -> Self {
        self.as_native().copy(bdd.into()).into()
    }

    /// Populate a new `VertexSet` using a raw `Bdd` represented as a string.
    pub fn copy_with_raw_string(&self, bdd: String) -> PyResult<Self> {
        Ok(self.as_native().copy(Bdd::from_string(bdd.as_str())).into())
    }

    /// Get an approximate count of elements in this set.
    pub fn cardinality(&self) -> f64 {
        self.as_native().approx_cardinality()
    }

    /// Get an approximate memory consumption of this symbolic set in bytes.
    ///
    /// (real value may be different due to underlying details of OS and allocation mechanisms)
    pub fn symbolic_size(&self) -> usize {
        self.as_native().symbolic_size() * 10
    }

    /// Compute a `.dot` string representing the underlying BDD graph.
    ///
    /// Needs a reference to the underlying symbolic graph to resolve variable names.
    /// If you are ok with an anonymous BDD graph, convert the set to `Bdd` and then use
    /// anonymous `.dot` conversion function on the result.
    pub fn to_dot(&self, graph: &PySymbolicAsyncGraph) -> String {
        self.as_native().to_dot_string(graph.0.symbolic_context())
    }

    /// Compute a union of two `VertexSet` objects.
    pub fn union(&self, other: &Self) -> Self {
        self.as_native().union(other.as_native()).into()
    }

    /// Compute an intersection of two `VertexSet` objects.
    pub fn intersect(&self, other: &Self) -> Self {
        self.as_native().intersect(other.as_native()).into()
    }

    /// Compute a difference of two `VertexSet` objects.
    pub fn minus(&self, other: &Self) -> Self {
        self.as_native().minus(other.as_native()).into()
    }

    /// Returns true if this `VertexSet` set is empty.
    pub fn is_empty(&self) -> bool {
        self.as_native().is_empty()
    }

    /// Returns true if this `VertexSet` set is a subset of the argument `VertexSet`.
    pub fn is_subset(&self, other: &Self) -> bool {
        self.as_native().is_subset(other.as_native())
    }

    /// Pick a single vertex from this `VertexSet` and return in it as a singleton set.
    ///
    /// If the set is empty, returns an empty set.
    pub fn pick_singleton(&self) -> Self {
        self.as_native().pick_singleton().into()
    }

    /// Check if this symbolic set is a subspace (i.e. a hypercube).
    pub fn is_subspace(&self) -> bool {
        self.as_native().is_subspace()
    }

    /// Check if this symbolic set represents a singleton.
    pub fn is_singleton(&self) -> bool {
        self.as_native().is_singleton()
    }

    /// Return a subset of this set where the `variable` is set to the given `value`.
    ///
    /// Note: You can only use numeric IDs, not variable names.
    pub fn fix_network_variable(&self, variable: PyVariableId, value: bool) -> PyGraphVertices {
        self.as_native()
            .fix_network_variable(variable.into(), value)
            .into()
    }

    /// Make a subset of this set where the `variable` is set to the given `value`, and then
    /// erase the value from the set.
    ///
    /// Note: You can only use numeric IDs, not variable names.
    pub fn restrict_network_variable(
        &self,
        variable: PyVariableId,
        value: bool,
    ) -> PyGraphVertices {
        self.as_native()
            .restrict_network_variable(variable.into(), value)
            .into()
    }

    /// Instantiate this `VertexSet` into an explicit list of vertices.
    ///
    /// WARNING: Non-trivial sets may contain many vertices and can easily exhaust computer
    /// memory. Before running this method, make sure the set is sufficiently small.
    ///
    /// Each vertex is returned as a list of Boolean values that can be subsequently interpreted
    /// as symbolic sets using e.g. `SymbolicAsyncGraph`.
    #[pyo3(signature = (limit = None))]
    pub fn list_vertices(&self, limit: Option<usize>) -> Vec<Vec<bool>> {
        self.as_native()
            .materialize()
            .iter()
            .take(limit.unwrap_or(usize::MAX))
            .map(|bv| bv.values())
            .collect()
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("VertexSet(cardinality = {})", self.cardinality()))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
}
