use std::borrow::Cow;

use macros::{impl_for_non_gil, impl_for_non_gil2, Config, Object};
use pyo3::{prelude::*, types::PyDict};

use crate::{
    config_fn,
    context::Context,
    core::symbol::symbol::SymbolImpl,
    utils::{Config, Gil, Object},
};

#[derive(Clone, Debug, Object)]
#[object(class_name = "Wild")]
pub struct Wild(PyObject);

#[impl_for_non_gil2(Wild)]
impl<'py, 'a, 'b> Gil<'py, 'a, 'b, Wild> {
    pub fn new<T: ToString + ?Sized>(ctx: &'a Context<'py>, name: &T) -> PyResult<Self> {
        let res = Self::class(ctx)?.call1((name.to_string(),))?;
        Ok(Self(Cow::Owned(Wild(res.into())), ctx))
    }
    pub fn new_config<
        'f,
        T: ToString + ?Sized,
        F: for<'c> FnOnce(WildConfig<'c>) -> WildConfig<'c> + 'f,
    >(
        ctx: &'a Context<'py>,
        name: &T,
        config: F,
    ) -> PyResult<Self> {
        let res =
            Self::class(ctx)?.call((name.to_string(),), Some(config(WildConfig::new(ctx)).0))?;
        Ok(Self(Cow::Owned(Wild(res.into())), ctx))
    }
}
#[derive(Copy, Clone, Config)]
pub struct WildConfig<'py>(pub(crate) &'py PyDict);
impl<'py> WildConfig<'py> {
    config_fn!(exclude, &[impl ToPyObject]); // todo
    config_fn!(properties, &[impl ToPyObject]); // todo
    config_fn!(commutative, bool);
}

#[impl_for_non_gil(Wild)]
impl<'py, 'a, 'b> SymbolImpl for Gil<'py, 'a, 'b, Wild> {
    fn name(&self) -> PyResult<String> {
        self.py_inner().getattr("name")?.extract::<String>()
    }
    fn set_name<T: ToString + ?Sized>(&self, name: &T) -> PyResult<()> {
        self.py_inner().setattr("name", name.to_string())
    }
}
