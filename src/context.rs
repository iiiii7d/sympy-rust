use pyo3::prelude::*;

use crate::symbol::{Symbol, SymbolGIL};

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) sympy: Py<PyModule>,
}
impl Context {
    pub fn new() -> PyResult<Self> {
        let sympy = Python::with_gil(|py| py.import("sympy").map(|a| a.into()))?;
        Ok(Self { sympy })
    }
    pub fn symbol<T: ToString>(&self, name: T) -> PyResult<Symbol> {
        Python::with_gil(|py| self.symbol_gil(py, name).map(|a| a.un_gil()))
    }
    pub fn symbol_gil<'py, 'a: 'py, T: ToString>(
        &'a self,
        py: Python<'py>,
        name: T,
    ) -> PyResult<SymbolGIL<'py>> {
        SymbolGIL::new(py, self, name)
    }
    pub fn symbol_non_commutative<T: ToString>(&self, name: T) -> PyResult<Symbol> {
        Python::with_gil(|py| {
            self.symbol_non_commutative_gil(py, name)
                .map(|a| a.un_gil())
        })
    }
    pub fn symbol_non_commutative_gil<'py, 'a: 'py, T: ToString>(
        &'a self,
        py: Python<'py>,
        name: T,
    ) -> PyResult<SymbolGIL<'py>> {
        SymbolGIL::new(py, self, name)
    }
    pub(crate) fn sympy<'py, 'a: 'py>(&'a self, py: Python<'py>) -> &'py PyModule {
        self.sympy.as_ref(py)
    }
}
