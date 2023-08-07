use std::borrow::Cow;

use macros::{impl_for_non_gil, impl_for_non_gil2, Config, Object};
use pyo3::{prelude::*, types::PyDict};

use crate::{
    config_fn,
    context::Context,
    utils::{Config, Gil, Object},
};
#[derive(Clone, Debug, Object)]
#[object(class_name = "Symbol")]
pub struct Symbol(PyObject);

#[impl_for_non_gil2(Symbol)]
impl<'py, 'a, 'b> Gil<'py, 'a, 'b, Symbol> {
    pub fn new<T: ToString + ?Sized>(ctx: &'a Context<'py>, name: &T) -> PyResult<Self> {
        let res = Self::class(ctx)?.call1((name.to_string(),))?;
        Ok(Self(Cow::Owned(Symbol(res.into())), ctx))
    }
    pub fn new_config<
        'f,
        T: ToString + ?Sized,
        F: for<'c> FnOnce(SymbolConfig<'c>) -> SymbolConfig<'c> + 'f,
    >(
        ctx: &'a Context<'py>,
        name: &T,
        config: F,
    ) -> PyResult<Self> {
        let res =
            Self::class(ctx)?.call((name.to_string(),), Some(config(SymbolConfig::new(ctx)).0))?;
        Ok(Self(Cow::Owned(Symbol(res.into())), ctx))
    }
}

#[derive(Copy, Clone, Config)]
pub struct SymbolConfig<'py>(pub(crate) &'py PyDict);
impl<'py> SymbolConfig<'py> {
    config_fn!(commutative, bool);
}

pub trait SymbolImpl {
    fn name(&self) -> PyResult<String>;
    fn set_name<T: ToString + ?Sized>(&self, name: &T) -> PyResult<()>;
}

#[impl_for_non_gil(Symbol)]
impl<'py, 'a, 'b> SymbolImpl for Gil<'py, 'a, 'b, Symbol> {
    fn name(&self) -> PyResult<String> {
        self.py_inner().getattr("name")?.extract::<String>()
    }
    fn set_name<T: ToString + ?Sized>(&self, name: &T) -> PyResult<()> {
        self.py_inner().setattr("name", name.to_string())
    }
}
