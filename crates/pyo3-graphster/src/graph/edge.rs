use graphster::{errors::GraphsterError, graph::EdgeIndex};
use pyo3::{
    types::{PyAnyMethods, PyInt},
    Bound, FromPyObject, IntoPy, PyAny, PyObject, PyRef, PyResult, Python,
};

use crate::{gil_hash_map::GILHashMap, Lut, PyGraphsterError, PyUsize};

#[derive(Debug)]
#[repr(transparent)]
pub struct PyEdgeIndex(pub EdgeIndex);

impl From<EdgeIndex> for PyEdgeIndex {
    fn from(value: EdgeIndex) -> Self {
        Self(value)
    }
}

impl From<PyEdgeIndex> for EdgeIndex {
    fn from(value: PyEdgeIndex) -> Self {
        value.0
    }
}

impl AsRef<EdgeIndex> for PyEdgeIndex {
    fn as_ref(&self) -> &EdgeIndex {
        &self.0
    }
}

static EDGEINDEX_CONVERSION_LUT: Lut<EdgeIndex> = GILHashMap::new();

fn convert_pyobject_to_edgeindex(ob: &Bound<'_, PyAny>) -> PyResult<EdgeIndex> {
    fn convert_int(ob: &Bound<'_, PyAny>) -> PyResult<EdgeIndex> {
        Ok(ob
            .extract::<usize>()
            .expect("Extraction must succeed")
            .into())
    }

    fn convert_pyusize(ob: &Bound<'_, PyAny>) -> PyResult<EdgeIndex> {
        Ok(ob
            .extract::<PyRef<PyUsize>>()
            .expect("Extraction must succeed")
            .value()
            .into())
    }

    fn throw_error(ob: &Bound<'_, PyAny>) -> PyResult<EdgeIndex> {
        Err(
            PyGraphsterError::from(GraphsterError::ConversionError(format!(
                "Could not convert {} into AttributeValue",
                ob
            )))
            .into(),
        )
    }

    let type_pointer = ob.get_type_ptr() as usize;

    Python::with_gil(|py| {
        EDGEINDEX_CONVERSION_LUT.map(py, |lut| {
            let conversion_function = lut.entry(type_pointer).or_insert_with(|| {
                if ob.is_instance_of::<PyInt>() {
                    convert_int
                } else if ob.is_instance_of::<PyUsize>() {
                    convert_pyusize
                } else {
                    throw_error
                }
            });

            conversion_function(ob)
        })
    })
}

impl FromPyObject<'_> for PyEdgeIndex {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        convert_pyobject_to_edgeindex(ob).map(PyEdgeIndex::from)
    }
}

impl IntoPy<PyObject> for PyEdgeIndex {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyUsize::new(self.0.into()).into_py(py)
    }
}
