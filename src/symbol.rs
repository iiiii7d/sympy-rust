use macros::{NoGIL, GIL};
use pyo3::prelude::*;

use crate::{
    context::Context,
    py_dict,
    utils::{NoGIL, Object, GIL},
};

#[derive(Clone, Debug, GIL)]
pub struct SymbolGIL<'py>(&'py PyAny);
impl<'py> SymbolGIL<'py> {
    pub fn class<'a: 'py>(py: Python<'py>, ctx: &'a Context) -> PyResult<&'py PyAny> {
        ctx.sympy(py).getattr("Symbol")
    }
    pub fn un_gil(&self) -> Symbol {
        Symbol(self.0.into())
    }
    pub fn new<'a: 'py, T: ToString>(py: Python<'py>, ctx: &'a Context, name: T) -> PyResult<Self> {
        let res = Self::class(py, ctx)?.call1((name.to_string(),))?;
        Ok(Self(res))
    }
    pub fn new_non_commutative<'a: 'py, T: ToString>(
        py: Python<'py>,
        ctx: &'a Context,
        name: T,
    ) -> PyResult<Self> {
        let res = Self::class(py, ctx)?.call(
            (name.to_string(),),
            Some(py_dict! {py, "commutative" => false}),
        )?;
        Ok(Self(res))
    }
    pub fn name(&self) -> PyResult<String> {
        self.0.getattr("name")?.extract::<String>()
    }
    pub fn set_name<T: ToString>(&self, name: T) -> PyResult<()> {
        self.0.setattr("name", name.to_string())
    }
}

#[derive(Clone, Debug, NoGIL)]
pub struct Symbol(Py<PyAny>);
impl Symbol {
    pub fn gil<'py, 'a: 'py>(&'a self, py: Python<'py>) -> SymbolGIL<'py> {
        SymbolGIL(self.0.as_ref(py))
    }
    pub fn name(&self) -> PyResult<String> {
        Python::with_gil(|py| self.gil(py).name())
    }
    pub fn set_name<T: ToString>(&self, name: T) -> PyResult<()> {
        Python::with_gil(|py| self.gil(py).set_name(name))
    }
}
