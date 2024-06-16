mod datatypes;
mod errors;
mod gil_hash_map;
mod graph;

pub use datatypes::{
    key::PyAttributeKey, value::PyAttributeValue, PyBoolean, PyFloat32, PyFloat64, PyInt128,
    PyInt16, PyInt32, PyInt64, PyInt8, PyString, PyUInt128, PyUInt16, PyUInt32, PyUInt64, PyUInt8,
    PyUsize,
};
pub use errors::{
    ConversionError, EdgeNotFoundError, GraphsterBaseError, NodeAlreadyExistsError,
    NodeNotFoundError, PyGraphsterError,
};
pub use graph::PyDataGraph;

use gil_hash_map::GILHashMap;
use pyo3::{Bound, PyAny, PyResult};

pub(crate) type Lut<T> = GILHashMap<usize, fn(&Bound<'_, PyAny>) -> PyResult<T>>;
