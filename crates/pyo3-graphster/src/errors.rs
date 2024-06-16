use graphster::errors::GraphsterError;
use pyo3::{create_exception, exceptions::PyException, PyErr};

create_exception!(_graphster.exceptions, GraphsterBaseError, PyException);
create_exception!(_graphster.exceptions, NodeNotFoundError, GraphsterBaseError);
create_exception!(
    _graphster.exceptions,
    NodeAlreadyExistsError,
    GraphsterBaseError
);
create_exception!(_graphster.exceptions, EdgeNotFoundError, GraphsterBaseError);
create_exception!(_graphster.exceptions, ConversionError, GraphsterBaseError);

#[derive(Debug)]
#[repr(transparent)]
pub struct PyGraphsterError(GraphsterError);

impl From<GraphsterError> for PyGraphsterError {
    fn from(value: GraphsterError) -> Self {
        Self(value)
    }
}

impl From<PyGraphsterError> for PyErr {
    fn from(value: PyGraphsterError) -> Self {
        match value.0 {
            GraphsterError::NodeNotFound { .. } => NodeNotFoundError::new_err(value.0.to_string()),
            GraphsterError::NodeAlreadyExists { .. } => {
                NodeAlreadyExistsError::new_err(value.0.to_string())
            }
            GraphsterError::EdgeNotFound { .. } => EdgeNotFoundError::new_err(value.0.to_string()),
            GraphsterError::ConversionError { .. } => ConversionError::new_err(value.0.to_string()),
        }
    }
}
