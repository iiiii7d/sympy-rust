use pyo3::prelude::*;
use crate::symbol::Symbol;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) sympy: Py<PyModule>
}
impl Context {
    pub fn new() -> PyResult<Self> {
        let sympy = Python::with_gil(|py| {
            py.import("sympy").map(|a| a.into())
        })?;
        Ok(Self {
            sympy
        })
    }
    pub fn symbol<T: ToString>(&self, name: T) -> PyResult<Symbol> {
        Symbol::new(self, name)
    }
    pub fn symbol_non_commutative<T: ToString>(&self, name: T) -> PyResult<Symbol> {
        Symbol::new_non_commutative(self, name)
    }
    pub(crate) fn sympy<'py, 'a: 'py>(&'a self, py: Python<'py>) -> &'py PyModule {
        self.sympy.as_ref(py)
    }
}