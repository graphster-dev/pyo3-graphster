mod datatypes;
mod errors;
mod gil_hash_map;
mod graph;

pub use datatypes::{key::PyAttributeKey, value::PyAttributeValue, *};
pub use errors::{
    ConversionError, EdgeNotFoundError, GraphsterBaseError, NodeAlreadyExistsError,
    NodeNotFoundError, PyGraphsterError,
};
pub use graph::PyDataGraph;

use gil_hash_map::GILHashMap;
use pyo3::{Bound, PyAny, PyResult};

pub(crate) type Lut<T> = GILHashMap<usize, fn(&Bound<'_, PyAny>) -> PyResult<T>>;
