use std::collections::{HashMap, HashSet};

use biodivine_lib_bdd::{Bdd, BddVariable};
use biodivine_lib_param_bn::VariableId;
use pyo3::prelude::*;

use crate::{
    bindings::{
        algorithms::{
            cancellation::CancellationHandler, percolation::percolation_config::PercolationConfig,
        },
        lib_param_bn::{
            symbolic::asynchronous_graph::AsynchronousGraph,
            variable_id::VariableId as VariableIdBinding,
        },
    },
    is_cancelled, AsNative,
};

use super::percolation_error::PercolationError;

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
    // TODO: discuss - what should be the result of this function?
    pub fn percolate_subspace(&self) -> Result<Vec<Option<bool>>, PercolationError> {
        // TODO: ohtenkay - logging

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
        for (var, v) in &self.config().subspace {
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

                is_cancelled!(self)?;

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

        Ok(fixed)
    }
}

#[pymethods]
impl Percolation {
    // TODO: ohtenkay - creation methods

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
    #[staticmethod]
    pub fn percolate_subspace_py(
        py: Python,
        graph: &AsynchronousGraph,
        space: &Bound<'_, PyAny>,
    ) -> PyResult<HashMap<VariableIdBinding, bool>> {
        // TODO: Logging?
        let native_graph = graph.as_native();
        let state_variables = native_graph.symbolic_context().state_variables();
        let mut network_variables = vec![
            VariableId::from_index(0);
            native_graph
                .symbolic_context()
                .bdd_variable_set()
                .num_vars() as usize
        ];
        for var in native_graph.variables() {
            let bdd_var = state_variables[var.to_index()];
            network_variables[bdd_var.to_index()] = var;
        }

        let initial_space = graph.resolve_subspace_valuation(space)?;

        // Variables that have a known fixed value.
        let mut fixed: Vec<Option<bool>> = vec![None; native_graph.num_vars()];
        for (var, v) in &initial_space {
            fixed[var.to_index()] = Some(*v);
        }

        let mut fns: Vec<Option<Bdd>> = vec![None; native_graph.num_vars()];
        let mut fn_inputs: Vec<Option<HashSet<BddVariable>>> = vec![None; native_graph.num_vars()];

        let mut restriction = Vec::new();

        let mut done = false;
        while !done {
            done = true;
            for i in 0..native_graph.num_vars() {
                if fixed[i].is_some() {
                    continue;
                }

                py.check_signals()?;

                if fns[i].is_none() {
                    let fn_bdd = native_graph.get_symbolic_fn_update(VariableId::from_index(i));
                    fns[i] = Some(fn_bdd.clone());
                }

                let fn_bdd = fns[i].as_mut().unwrap();

                let value = if fn_bdd.is_false() {
                    false
                } else if fn_bdd.is_true() {
                    true
                } else {
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
                    if fn_bdd.is_true() {
                        true
                    } else if fn_bdd.is_false() {
                        false
                    } else {
                        continue;
                    }
                };

                done = false;
                fixed[i] = Some(value);
            }
        }

        let mut result = HashMap::new();
        for (i, v) in fixed.iter().enumerate() {
            if let Some(v) = v.as_ref() {
                let var = VariableIdBinding::new(i);
                result.insert(var, *v);
            }
        }

        Ok(result)
    }
}
