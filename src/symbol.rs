use pyo3::prelude::*;
use crate::context::Context;
use crate::py_dict;

#[derive(Clone, Debug)]
pub struct Symbol(Py<PyAny>);
impl Symbol {
    pub fn class<'py, 'a: 'py>(py: Python<'py>, ctx: &'a Context) -> PyResult<&'py PyAny> {
        ctx.sympy(py).getattr("Symbol")
    }
    pub fn new_gil<T: ToString>(py: Python, ctx: &Context, name: T) -> PyResult<Self> {
        Ok(Self(Self::class(py, ctx)?.call1((name.to_string(), ))?.into()))
    }
    pub fn new_non_commutative_gil<T: ToString>(py: Python, ctx: &Context, name: T) -> PyResult<Self> {
        Ok(Self(Self::class(py, ctx)?.call((name.to_string(), ), Some(py_dict! {py, "commutative" => false}))?.into()))
    }
    pub fn name_gil(&self, py: Python) -> PyResult<String> {
        self.0.as_ref(py).getattr("name")?.extract::<String>()
    }
    pub fn set_name_gil<T: ToString>(&self, py: Python, name: T) -> PyResult<()> {
        self.0.as_ref(py).setattr("name", name.to_string())
    }
    pub fn new<T: ToString>(ctx: &Context, name: T) -> PyResult<Self> {
        Python::with_gil(|py| {
            Self::new_gil(py, ctx, name)
        })
    } // TODO derive

    pub fn new_non_commutative<T: ToString>(ctx: &Context, name: T) -> PyResult<Self> {
        Python::with_gil(|py| {
            Self::new_non_commutative_gil(py, ctx, name)
        })
    } // TODO derive
    pub fn name(&self) -> PyResult<String> {
        Python::with_gil(|py| {
            self.name_gil(py)
        })
    }
    pub fn set_name<T: ToString>(&self, name: T) -> PyResult<()> {
        Python::with_gil(|py| {
            self.set_name_gil(py, name)
        })
    }
}
