use std::collections::HashSet;

use biodivine_lib_bdd::{Bdd, BddVariable};
use biodivine_lib_param_bn::{
    symbolic_async_graph::SymbolicAsyncGraph, BooleanNetwork, VariableId,
};
use pyo3::{pyclass, pymethods, PyResult};

use crate::bindings::{
    algorithms::{
        cancellation::CancellationHandler,
        percolation::{
            percolation_config::PercolationConfig, percolation_error::PercolationError,
            subspace_representation::SubspaceRepresentation,
        },
    },
    lib_param_bn::{
        boolean_network::BooleanNetwork as BooleanNetworkBinding,
        symbolic::asynchronous_graph::AsynchronousGraph,
    },
};

#[pyclass(module = "biodivine_aeon", frozen)]
#[derive(Clone)]
pub struct Percolation(PercolationConfig);

impl Percolation {
    pub fn config(&self) -> &PercolationConfig {
        &self.0
    }
}

impl CancellationHandler for Percolation {
    fn is_cancelled(&self) -> bool {
        self.0.cancellation.is_cancelled()
    }

    fn start_timer(&self) {
        self.0.cancellation.start_timer()
    }
}

impl Percolation {
    pub fn from_boolean_network(bn: &BooleanNetwork) -> PyResult<Self> {
        Ok(Percolation(PercolationConfig::from_boolean_network(bn)?))
    }

    pub fn with_graph(graph: SymbolicAsyncGraph) -> Self {
        Percolation(PercolationConfig::with_graph(graph))
    }

    pub fn with_config(config: PercolationConfig) -> Self {
        Percolation(config)
    }
}

impl Percolation {
    /// Performs a percolation of a single subspace.
    ///
    /// Percolation propagates the values of variables that are guaranteed to be constant in the
    /// given subspace. Note that this function will not overwrite values fixed in the original
    /// space if they percolate to a conflicting value. Also note that the result is a subspace
    /// of the original space, i.e. it does not just contain the newly propagated variables.
    ///
    /// This method should technically work on parametrized networks as well, but the constant
    /// check is performed across all interpretations, hence a lot of sub-spaces will not
    /// percolate meaningfully. We recommend using other symbolic methods for such systems.
    pub fn percolate_subspace(
        &self,
        subspace: Vec<(VariableId, bool)>,
    ) -> Result<Vec<(VariableId, bool)>, PercolationError> {
        // TODO: ohtenkay - logging, take inspiration from Reachability
        self.start_timer();

        let graph = &self.config().graph;

        let mut network_variables = vec![
            VariableId::from_index(0);
            graph.symbolic_context().bdd_variable_set().num_vars()
                as usize
        ];

        let state_variables = graph.symbolic_context().state_variables();
        for var in graph.variables() {
            let bdd_var = state_variables[var.to_index()];
            network_variables[bdd_var.to_index()] = var;
        }

        // Variables that have a known fixed value.
        let mut fixed: Vec<Option<bool>> = vec![None; graph.num_vars()];
        for (var, v) in &subspace {
            fixed[var.to_index()] = Some(*v);
        }

        let mut fns: Vec<Option<Bdd>> = vec![None; graph.num_vars()];
        let mut fn_inputs: Vec<Option<HashSet<BddVariable>>> = vec![None; graph.num_vars()];

        let mut restriction = Vec::new();

        let mut done = false;
        while !done {
            done = true;
            for i in 0..graph.num_vars() {
                if fixed[i].is_some() {
                    continue;
                }

                // TODO: ohtenkay - fix
                // fixed = is_cancelled!(self, fixed)?;

                if fns[i].is_none() {
                    let fn_bdd = graph.get_symbolic_fn_update(VariableId::from_index(i));
                    fns[i] = Some(fn_bdd.clone());
                }

                let fn_bdd = fns[i].as_mut().unwrap();

                let value = match (fn_bdd.is_true(), fn_bdd.is_false()) {
                    (true, _) => true,
                    (_, true) => false,
                    _ => {
                        if fn_inputs[i].is_none() {
                            let inputs = fn_bdd.support_set();
                            fn_inputs[i] = Some(inputs);
                        }

                        let inputs = fn_inputs[i].as_mut().unwrap();

                        restriction.clear();
                        for input in inputs.clone() {
                            let var = network_variables[input.to_index()];
                            if let Some(value) = fixed[var.to_index()] {
                                restriction.push((input, value));
                                inputs.remove(&input);
                            }
                        }

                        if restriction.is_empty() {
                            continue;
                        }

                        *fn_bdd = fn_bdd.restrict(&restriction);
                        match (fn_bdd.is_true(), fn_bdd.is_false()) {
                            (true, _) => true,
                            (_, true) => false,
                            _ => continue,
                        }
                    }
                };

                done = false;
                fixed[i] = Some(value);
            }
        }

        let result = fixed
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if let Some(v) = v.as_ref() {
                    let var = VariableId::from_index(i);
                    Some((var, *v))
                } else {
                    None
                }
            })
            .collect();

        Ok(result)
    }
}

#[pymethods]
impl Percolation {
    #[staticmethod]
    #[pyo3(name = "from_boolean_network")]
    pub fn from_boolean_network_py(bn: &BooleanNetworkBinding) -> PyResult<Self> {
        Ok(Percolation(PercolationConfig::from_boolean_network_py(bn)?))
    }

    #[staticmethod]
    #[pyo3(name = "with_graph")]
    pub fn with_graph_py(graph: &AsynchronousGraph) -> Self {
        Percolation(PercolationConfig::with_graph_py(graph))
    }

    #[staticmethod]
    #[pyo3(name = "with_config")]
    pub fn with_config_py(config: PercolationConfig) -> Self {
        Percolation(config)
    }

    #[pyo3(name = "percolate_subspace")]
    pub fn percolate_subspace_py(
        &self,
        subspace: SubspaceRepresentation,
    ) -> PyResult<SubspaceRepresentation> {
        let percolated_subspace = self.percolate_subspace(subspace.into())?;

        Ok(SubspaceRepresentation::from(percolated_subspace))
    }
}
