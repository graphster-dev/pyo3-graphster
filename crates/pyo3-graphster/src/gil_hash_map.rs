use pyo3::Python;
use std::{cell::UnsafeCell, collections::HashMap};

// Similar implementation to PyO3 and Polars GILOnceCell

#[derive(Debug)]
pub(crate) struct GILHashMap<K, V>(UnsafeCell<Option<HashMap<K, V>>>);

unsafe impl<K: Send + Sync, V: Send + Sync> Sync for GILHashMap<K, V> {}
unsafe impl<K: Send, V: Send> Send for GILHashMap<K, V> {}

impl<K, V> GILHashMap<K, V> {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    pub fn map<F, O>(&self, _py: Python<'_>, mut operation: F) -> O
    where
        F: FnMut(&mut HashMap<K, V>) -> O,
    {
        // SAFETY: the GIL is being held, so no other thread has access.
        let inner = unsafe { &mut *self.0.get() };

        if inner.is_none() {
            *inner = Some(HashMap::new());
        }

        // Technically this operation could temporarily release the GIL, but this is deemed
        // acceptable as it is not a function a user can control
        operation(inner.as_mut().expect("Inner must be some"))
    }
}
