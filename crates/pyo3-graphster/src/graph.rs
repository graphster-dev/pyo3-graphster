use crate::{datatypes::key::PyAttributeKey, errors::PyGraphsterError, PyAttributeValue};
use graphster::graph::DataGraph;
use pyo3::{pyclass, pymethods, PyResult};
use std::collections::HashMap;

pub type PyAttributes = HashMap<PyAttributeKey, PyAttributeValue>;
pub type PyNodeIndex = PyAttributeKey;

#[repr(transparent)]
#[pyclass]
pub struct PyDataGraph(DataGraph);

#[allow(clippy::new_without_default)]
#[pymethods]
impl PyDataGraph {
    #[new]
    pub fn new() -> Self {
        Self(DataGraph::new())
    }

    #[staticmethod]
    fn from_nodes(nodes: Vec<(PyNodeIndex, PyAttributes)>) -> Self {
        Self(DataGraph::from_nodes(nodes))
    }

    #[staticmethod]
    fn from_nodes_and_edges(
        nodes: Vec<(PyNodeIndex, PyAttributes)>,
        edges: Vec<(PyNodeIndex, PyNodeIndex, PyAttributes)>,
    ) -> PyResult<Self> {
        Ok(Self(
            DataGraph::from_nodes_and_edges(nodes, edges).map_err(PyGraphsterError::from)?,
        ))
    }

    fn add_node(&mut self, node_index: PyNodeIndex, attributes: PyAttributes) -> PyResult<()> {
        Ok(self
            .0
            .add_node(node_index, attributes)
            .map_err(PyGraphsterError::from)?)
    }

    fn add_nodes(&mut self, nodes: Vec<(PyNodeIndex, PyAttributes)>) -> PyResult<()> {
        Ok(self.0.add_nodes(nodes).map_err(PyGraphsterError::from)?)
    }

    fn node_count(&self) -> usize {
        self.0.node_count()
    }

    fn edge_count(&self) -> usize {
        self.0.edge_count()
    }
}
