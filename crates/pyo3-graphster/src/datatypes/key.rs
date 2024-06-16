use super::{
    value::convert_pyobject_to_attributevalue, PyBoolean, PyInt16, PyInt32, PyInt64, PyInt8,
    PyString, PyUInt16, PyUInt32, PyUInt64, PyUInt8, PyUsize,
};
use crate::{errors::PyGraphsterError, PyAttributeValue};
use graphster::{
    datatypes::{AttributeKey, AttributeValue},
    errors::GraphsterError,
};
use pyo3::{Bound, FromPyObject, IntoPy, PyAny, PyErr, PyObject, PyResult, Python};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl TryFrom<PyAttributeValue> for PyAttributeKey {
    type Error = PyGraphsterError;

    fn try_from(value: PyAttributeValue) -> Result<Self, Self::Error> {
        match value.into() {
            AttributeValue::Boolean(value) => Ok(AttributeKey::Boolean(value).into()),
            AttributeValue::Int16(value) => Ok(AttributeKey::Int16(value).into()),
            AttributeValue::Int32(value) => Ok(AttributeKey::Int32(value).into()),
            AttributeValue::Int64(value) => Ok(AttributeKey::Int64(value).into()),
            AttributeValue::Int8(value) => Ok(AttributeKey::Int8(value).into()),
            AttributeValue::String(value) => Ok(AttributeKey::String(value).into()),
            AttributeValue::UInt16(value) => Ok(AttributeKey::UInt16(value).into()),
            AttributeValue::UInt32(value) => Ok(AttributeKey::UInt32(value).into()),
            AttributeValue::UInt64(value) => Ok(AttributeKey::UInt64(value).into()),
            AttributeValue::UInt8(value) => Ok(AttributeKey::UInt8(value).into()),
            AttributeValue::Usize(value) => Ok(AttributeKey::Usize(value).into()),
            _ => Err(GraphsterError::ConversionError(
                "Could not convert PyAttributeValue to PyAttributeKey".to_string(),
            )
            .into()),
        }
    }
}

impl<'a> FromPyObject<'a> for PyAttributeKey {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        PyAttributeKey::try_from(
            convert_pyobject_to_attributevalue(ob).map(PyAttributeValue::from)?,
        )
        .map_err(PyErr::from)
    }
}

impl IntoPy<PyObject> for PyAttributeKey {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self.0 {
            AttributeKey::Boolean(value) => PyBoolean(value).into_py(py),
            AttributeKey::Int16(value) => PyInt16(value).into_py(py),
            AttributeKey::Int32(value) => PyInt32(value).into_py(py),
            AttributeKey::Int64(value) => PyInt64(value).into_py(py),
            AttributeKey::Int8(value) => PyInt8(value).into_py(py),
            AttributeKey::String(value) => PyString(value).into_py(py),
            AttributeKey::UInt16(value) => PyUInt16(value).into_py(py),
            AttributeKey::UInt32(value) => PyUInt32(value).into_py(py),
            AttributeKey::UInt64(value) => PyUInt64(value).into_py(py),
            AttributeKey::UInt8(value) => PyUInt8(value).into_py(py),
            AttributeKey::Usize(value) => PyUsize(value).into_py(py),
        }
    }
}
