use std::borrow::Cow;

use duplicate::duplicate_item;
use macros::{impl_for_non_gil, impl_for_non_gil2, Config, Object};
use pyo3::{prelude::*, types::PyDict};

use crate::{
    config_fn,
    context::Context,
    core::wild::{Wild, WildConfig},
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

pub trait SymbolConfigImpl<'py>: Config<'py> {
    config_fn!(commutative, bool);
}
#[duplicate_item(
  StructConfig; [SymbolConfig]; [WildConfig];
)]
impl<'py> SymbolConfigImpl<'py> for StructConfig<'py> {}

pub trait SymbolImpl {
    fn name(&self) -> PyResult<String>;
    fn set_name<T: ToString>(&self, name: T) -> PyResult<()>;
}

#[duplicate_item(
  Struct; [Symbol]; [Wild];
)]
#[impl_for_non_gil(Struct)]
impl<'py, 'a, 'b> SymbolImpl for Gil<'py, 'a, 'b, Struct> {
    fn name(&self) -> PyResult<String> {
        self.get_attr("name")
    }
    fn set_name<T: ToString>(&self, name: T) -> PyResult<()> {
        self.set_attr("name", name.to_string())
    }
}
