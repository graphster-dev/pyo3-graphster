use pyo3::{pymodule, types::PyModule, Bound, PyResult, Python};
use pyo3_graphster::{
    ConversionError, EdgeNotFoundError, GraphsterBaseError, NodeAlreadyExistsError,
    NodeNotFoundError, PyBoolean, PyDataGraph, PyFloat32, PyFloat64, PyInt128, PyInt16, PyInt32,
    PyInt64, PyInt8, PyString, PyUInt128, PyUInt16, PyUInt32, PyUInt64, PyUInt8, PyUsize,
};

#[pymodule]
fn _pyo3_graphster(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBoolean>()?;
    m.add_class::<PyFloat32>()?;
    m.add_class::<PyFloat64>()?;
    m.add_class::<PyInt128>()?;
    m.add_class::<PyInt16>()?;
    m.add_class::<PyInt32>()?;
    m.add_class::<PyInt64>()?;
    m.add_class::<PyInt8>()?;
    m.add_class::<PyString>()?;
    m.add_class::<PyUInt128>()?;
    m.add_class::<PyUInt16>()?;
    m.add_class::<PyUInt32>()?;
    m.add_class::<PyUInt64>()?;
    m.add_class::<PyUInt8>()?;
    m.add_class::<PyUsize>()?;

    m.add_class::<PyDataGraph>()?;

    m.add(
        "GraphsterBaseError",
        py.get_type_bound::<GraphsterBaseError>(),
    )?;
    m.add(
        "NodeNotFoundError",
        py.get_type_bound::<NodeNotFoundError>(),
    )?;
    m.add(
        "NodeAlreadyExistsError",
        py.get_type_bound::<NodeAlreadyExistsError>(),
    )?;
    m.add(
        "EdgeNotFoundError",
        py.get_type_bound::<EdgeNotFoundError>(),
    )?;
    m.add("ConversionError", py.get_type_bound::<ConversionError>())?;
    Ok(())
}
