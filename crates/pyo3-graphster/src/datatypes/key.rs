use super::{
    value::convert_pyobject_to_attributevalue, PyBoolean, PyInt128, PyInt16, PyInt32, PyInt64,
    PyInt8, PyString, PyUInt128, PyUInt16, PyUInt32, PyUInt64, PyUInt8, PyUsize,
};
use crate::errors::PyGraphsterError;
use graphster::{datatypes::AttributeKey, from_marker::FromMarker, implement_from_marker};
use pyo3::{Bound, FromPyObject, IntoPy, PyAny, PyObject, PyResult, Python};

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PyAttributeKey(AttributeKey);

impl From<AttributeKey> for PyAttributeKey {
    fn from(value: AttributeKey) -> Self {
        Self(value)
    }
}

impl From<PyAttributeKey> for AttributeKey {
    fn from(value: PyAttributeKey) -> Self {
        value.0
    }
}

implement_from_marker!(AttributeKey, PyAttributeKey);

impl<'a> FromPyObject<'a> for PyAttributeKey {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        Ok(
            AttributeKey::try_from(convert_pyobject_to_attributevalue(ob)?)
                .map_err(PyGraphsterError::from)?
                .into(),
        )
    }
}

impl IntoPy<PyObject> for PyAttributeKey {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self.0 {
            AttributeKey::Boolean(value) => PyBoolean(value).into_py(py),
            AttributeKey::Int128(value) => PyInt128(value).into_py(py),
            AttributeKey::Int16(value) => PyInt16(value).into_py(py),
            AttributeKey::Int32(value) => PyInt32(value).into_py(py),
            AttributeKey::Int64(value) => PyInt64(value).into_py(py),
            AttributeKey::Int8(value) => PyInt8(value).into_py(py),
            AttributeKey::String(value) => PyString(value).into_py(py),
            AttributeKey::UInt128(value) => PyUInt128(value).into_py(py),
            AttributeKey::UInt16(value) => PyUInt16(value).into_py(py),
            AttributeKey::UInt32(value) => PyUInt32(value).into_py(py),
            AttributeKey::UInt64(value) => PyUInt64(value).into_py(py),
            AttributeKey::UInt8(value) => PyUInt8(value).into_py(py),
            AttributeKey::Usize(value) => PyUsize(value).into_py(py),
        }
    }
}
