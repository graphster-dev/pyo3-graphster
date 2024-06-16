use crate::{
    datatypes::{
        PyBoolean, PyFloat32, PyFloat64, PyInt128, PyInt16, PyInt32, PyInt64, PyInt8, PyUInt128,
        PyUInt16, PyUInt32, PyUInt64, PyUInt8, PyUsize,
    },
    gil_hash_map::GILHashMap,
    Lut, PyGraphsterError,
};
use graphster::{datatypes::AttributeValue, errors::GraphsterError};
use pyo3::{
    types::{PyAnyMethods, PyBool, PyFloat, PyInt},
    Bound, FromPyObject, IntoPy, PyAny, PyObject, PyRef, PyResult, Python,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct PyAttributeValue(AttributeValue);

impl From<AttributeValue> for PyAttributeValue {
    fn from(value: AttributeValue) -> Self {
        Self(value)
    }
}

impl From<PyAttributeValue> for AttributeValue {
    fn from(value: PyAttributeValue) -> Self {
        value.0
    }
}

static ATTRIBUTEVALUE_CONVERSION_LUT: Lut<AttributeValue> = GILHashMap::new();

pub(crate) fn convert_pyobject_to_attributevalue(
    ob: &Bound<'_, PyAny>,
) -> PyResult<AttributeValue> {
    fn convert_bool(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Boolean(
            ob.extract().expect("Extraction must succeed"),
        ))
    }

    fn convert_int(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Int64(
            ob.extract().expect("Extraction must succeed"),
        ))
    }

    fn convert_string(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::String(
            ob.extract().expect("Extraction must succeed"),
        ))
    }

    fn convert_float(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Float64(
            ob.extract().expect("Extraction must succeed"),
        ))
    }

    fn convert_null(_ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Null)
    }

    fn convert_pyboolean(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Boolean(
            ob.extract::<PyRef<PyBoolean>>()
                .expect("Extraction must succeed")
                .0,
        ))
    }

    fn convert_pyfloat32(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Float32(
            ob.extract::<PyRef<PyFloat32>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pyfloat64(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Float64(
            ob.extract::<PyRef<PyFloat64>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pyint128(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Int128(
            ob.extract::<PyRef<PyInt128>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pyint16(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Int16(
            ob.extract::<PyRef<PyInt16>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pyint32(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Int32(
            ob.extract::<PyRef<PyInt32>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pyint64(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Int64(
            ob.extract::<PyRef<PyInt64>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pyint8(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Int8(
            ob.extract::<PyRef<PyInt8>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pystring(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::String(
            ob.extract::<PyRef<crate::datatypes::PyString>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pyuint128(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::UInt128(
            ob.extract::<PyRef<PyUInt128>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pyuint16(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::UInt16(
            ob.extract::<PyRef<PyUInt16>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pyuint32(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::UInt32(
            ob.extract::<PyRef<PyUInt32>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pyuint64(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::UInt64(
            ob.extract::<PyRef<PyUInt64>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pyuint8(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::UInt8(
            ob.extract::<PyRef<PyUInt8>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn convert_pyusize(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Ok(AttributeValue::Usize(
            ob.extract::<PyRef<PyUsize>>()
                .expect("Extraction must succeed")
                .value(),
        ))
    }

    fn throw_error(ob: &Bound<'_, PyAny>) -> PyResult<AttributeValue> {
        Err(
            PyGraphsterError::from(GraphsterError::ConversionError(format!(
                "Could not convert {} to AttributeValue",
                ob
            )))
            .into(),
        )
    }

    let type_pointer = ob.get_type_ptr() as usize;

    Python::with_gil(|py| {
        ATTRIBUTEVALUE_CONVERSION_LUT.map(py, |lut| {
            let conversion_function = lut.entry(type_pointer).or_insert_with(|| {
                if ob.is_instance_of::<PyBool>() {
                    convert_bool
                } else if ob.is_instance_of::<PyInt>() {
                    convert_int
                } else if ob.is_instance_of::<pyo3::types::PyString>() {
                    convert_string
                } else if ob.is_instance_of::<PyFloat>() {
                    convert_float
                } else if ob.is_instance_of::<PyBoolean>() {
                    convert_pyboolean
                } else if ob.is_instance_of::<PyFloat32>() {
                    convert_pyfloat32
                } else if ob.is_instance_of::<PyFloat64>() {
                    convert_pyfloat64
                } else if ob.is_instance_of::<PyInt128>() {
                    convert_pyint128
                } else if ob.is_instance_of::<PyInt16>() {
                    convert_pyint16
                } else if ob.is_instance_of::<PyInt32>() {
                    convert_pyint32
                } else if ob.is_instance_of::<PyInt64>() {
                    convert_pyint64
                } else if ob.is_instance_of::<PyInt8>() {
                    convert_pyint8
                } else if ob.is_instance_of::<crate::datatypes::PyString>() {
                    convert_pystring
                } else if ob.is_instance_of::<PyUInt128>() {
                    convert_pyuint128
                } else if ob.is_instance_of::<PyUInt16>() {
                    convert_pyuint16
                } else if ob.is_instance_of::<PyUInt32>() {
                    convert_pyuint32
                } else if ob.is_instance_of::<PyUInt64>() {
                    convert_pyuint64
                } else if ob.is_instance_of::<PyUInt8>() {
                    convert_pyuint8
                } else if ob.is_instance_of::<PyUsize>() {
                    convert_pyusize
                } else if ob.is_none() {
                    convert_null
                } else {
                    throw_error
                }
            });

            conversion_function(ob)
        })
    })
}

impl<'a> FromPyObject<'a> for PyAttributeValue {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        convert_pyobject_to_attributevalue(ob).map(PyAttributeValue::from)
    }
}

impl IntoPy<PyObject> for PyAttributeValue {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self.0 {
            AttributeValue::Boolean(value) => PyBoolean(value).into_py(py),
            AttributeValue::Float32(value) => PyFloat32(value).into_py(py),
            AttributeValue::Float64(value) => PyFloat64(value).into_py(py),
            AttributeValue::Int128(value) => PyInt128(value).into_py(py),
            AttributeValue::Int16(value) => PyInt16(value).into_py(py),
            AttributeValue::Int32(value) => PyInt32(value).into_py(py),
            AttributeValue::Int64(value) => PyInt64(value).into_py(py),
            AttributeValue::Int8(value) => PyInt8(value).into_py(py),
            AttributeValue::Null => py.None(),
            AttributeValue::String(value) => crate::datatypes::PyString(value).into_py(py),
            AttributeValue::UInt128(value) => PyUInt128(value).into_py(py),
            AttributeValue::UInt16(value) => PyUInt16(value).into_py(py),
            AttributeValue::UInt32(value) => PyUInt32(value).into_py(py),
            AttributeValue::UInt64(value) => PyUInt64(value).into_py(py),
            AttributeValue::UInt8(value) => PyUInt8(value).into_py(py),
            AttributeValue::Usize(value) => PyUsize(value).into_py(py),
        }
    }
}
