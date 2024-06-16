use pyo3::{pyclass, pymethods};

macro_rules! implement_pymethods {
    ($struct:ty, $inner:ty) => {
        #[pymethods]
        impl $struct {
            #[new]
            pub fn new(value: $inner) -> Self {
                Self(value)
            }

            pub fn value(&self) -> $inner {
                self.0
            }
        }
    };
}

pub(crate) mod key;
pub(crate) mod value;

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyBoolean(bool);
implement_pymethods!(PyBoolean, bool);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyFloat32(f32);
implement_pymethods!(PyFloat32, f32);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyFloat64(f64);
implement_pymethods!(PyFloat64, f64);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyInt128(i128);
implement_pymethods!(PyInt128, i128);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyInt16(i16);
implement_pymethods!(PyInt16, i16);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyInt32(i32);
implement_pymethods!(PyInt32, i32);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyInt64(i64);
implement_pymethods!(PyInt64, i64);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyInt8(i8);
implement_pymethods!(PyInt8, i8);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyString(String);
#[pymethods]
impl PyString {
    #[new]
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> String {
        self.0.clone()
    }
}

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyUInt128(u128);
implement_pymethods!(PyUInt128, u128);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyUInt16(u16);
implement_pymethods!(PyUInt16, u16);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyUInt32(u32);
implement_pymethods!(PyUInt32, u32);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyUInt64(u64);
implement_pymethods!(PyUInt64, u64);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyUInt8(u8);
implement_pymethods!(PyUInt8, u8);

#[pyclass]
#[derive(Debug)]
#[repr(transparent)]
pub struct PyUsize(usize);
implement_pymethods!(PyUsize, usize);
