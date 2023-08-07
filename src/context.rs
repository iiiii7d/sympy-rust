use std::borrow::Cow;

use pyo3::prelude::*;

#[derive(Clone)]
pub struct Context<'py> {
    pub(crate) gil: Python<'py>,
    pub(crate) sympy: Py<PyModule>,
}

impl<'py> Context<'py> {
    pub(crate) fn sympy<'a: 'py>(&'a self) -> &'py PyModule {
        self.sympy.as_ref(self.gil)
    }
    pub fn new(py: Python<'py>) -> PyResult<Self> {
        Ok(Self {
            gil: py,
            sympy: py.import("sympy")?.into(),
        })
    }
    pub fn with_gil<'b, R, F: for<'a> FnOnce(Context<'a>) -> R + 'b>(f: F) -> PyResult<R> {
        Python::with_gil(|py| {
            let ctx = Context::new(py)?;
            Ok(f(ctx))
        })
    }
    pub fn try_with_gil<'b, R, F: for<'a> FnOnce(Context<'a>) -> PyResult<R> + 'b>(
        f: F,
    ) -> PyResult<R> {
        Python::with_gil(|py| {
            let ctx = Context::new(py)?;
            f(ctx)
        })
    }
}
