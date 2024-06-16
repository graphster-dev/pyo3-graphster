mod edge;

use crate::{datatypes::key::PyAttributeKey, errors::PyGraphsterError, PyAttributeValue};
pub use edge::PyEdgeIndex;
use graphster::graph::DataGraph;
use pyo3::{pyclass, pymethods, PyResult};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;

pub type PyAttributes = HashMap<PyAttributeKey, PyAttributeValue>;
pub type PyNodeIndex = PyAttributeKey;

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyDataGraph(DataGraph);

#[allow(clippy::new_without_default)]
#[pymethods]
impl PyDataGraph {
    #[new]
    fn new() -> Self {
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

    fn add_edge(
        &mut self,
        source_node_index: PyNodeIndex,
        target_node_index: PyNodeIndex,
        attributes: PyAttributes,
    ) -> PyResult<PyEdgeIndex> {
        Ok(self
            .0
            .add_edge(source_node_index, target_node_index, attributes)
            .map_err(PyGraphsterError::from)?
            .into())
    }

    fn add_edges(
        &mut self,
        edges: Vec<(PyNodeIndex, PyNodeIndex, PyAttributes)>,
    ) -> PyResult<Vec<PyEdgeIndex>> {
        Ok(self
            .0
            .add_edges(edges)
            .map_err(PyGraphsterError::from)?
            .into_par_iter()
            .map(|edge_index| edge_index.into())
            .collect())
    }

    fn remove_node(&mut self, node_index: PyNodeIndex) -> PyResult<PyAttributes> {
        Ok(self
            .0
            .remove_node(node_index)
            .map_err(PyGraphsterError::from)?
            .into_par_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect())
    }

    fn remove_edge(&mut self, edge_index: PyEdgeIndex) -> PyResult<PyAttributes> {
        Ok(self
            .0
            .remove_edge(&edge_index.0)
            .map_err(PyGraphsterError::from)?
            .into_par_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect())
    }

    fn node_count(&self) -> usize {
        self.0.node_count()
    }

    fn edge_count(&self) -> usize {
        self.0.edge_count()
    }

    fn node_indices(&self) -> Vec<PyNodeIndex> {
        self.0
            .node_indices()
            .map(|index| index.clone().0.into())
            .collect()
    }

    fn edge_indices(&self) -> Vec<PyEdgeIndex> {
        self.0
            .edge_indices()
            .map(|index| (*index).into())
            .collect()
    }

    fn contains_node(&self, node_index: PyNodeIndex) -> bool {
        self.0.contains_node(node_index)
    }

    fn contains_edge(&self, edge_index: PyEdgeIndex) -> bool {
        self.0.contains_edge(&edge_index.0)
    }

    fn node_attributes(&self, node_index: PyNodeIndex) -> PyResult<PyAttributes> {
        Ok(self
            .0
            .node_attributes(node_index)
            .map_err(PyGraphsterError::from)?
            .into_par_iter()
            .map(|(key, value)| (key.clone().into(), value.clone().into()))
            .collect())
    }

    fn edge_attributes(&self, edge_index: PyEdgeIndex) -> PyResult<PyAttributes> {
        Ok(self
            .0
            .edge_attributes(&edge_index.0)
            .map_err(PyGraphsterError::from)?
            .into_par_iter()
            .map(|(key, value)| (key.clone().into(), value.clone().into()))
            .collect())
    }

    fn incoming_edge_indices(&self, node_index: PyNodeIndex) -> PyResult<Vec<PyEdgeIndex>> {
        Ok(self
            .0
            .incoming_edge_indices(node_index)
            .map_err(PyGraphsterError::from)?
            .map(|index| (*index).into())
            .collect())
    }

    fn outgoing_edge_indices(&self, node_index: PyNodeIndex) -> PyResult<Vec<PyEdgeIndex>> {
        Ok(self
            .0
            .outgoing_edge_indices(node_index)
            .map_err(PyGraphsterError::from)?
            .map(|index| (*index).into())
            .collect())
    }

    fn edges_connecting(
        &self,
        source_node_index: PyNodeIndex,
        target_node_index: PyNodeIndex,
    ) -> PyResult<Vec<PyEdgeIndex>> {
        Ok(self
            .0
            .edges_connecting(source_node_index, target_node_index)
            .map_err(PyGraphsterError::from)?
            .map(|index| (*index).into())
            .collect())
    }
}
