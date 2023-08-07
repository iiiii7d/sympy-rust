use std::borrow::Cow;

use macros::{impl_for_non_gil, Object};
use pyo3::prelude::*;

use crate::{
    context::Context,
    py_dict,
    utils::{Gil, Object},
};

pub trait SymbolImpl {
    fn name(&self) -> PyResult<String>;
    fn set_name<T: ToString + ?Sized>(&self, name: &T) -> PyResult<()>;
}

#[derive(Clone, Debug, Object)]
#[object(class_name = "Symbol")]
pub struct Symbol(PyObject);
impl<'py, 'a, 'b> Gil<'py, 'a, 'b, Symbol> {
    pub fn new<T: ToString + ?Sized>(ctx: &'a Context<'py>, name: &T) -> PyResult<Self> {
        let res = Self::class(ctx)?.call1((name.to_string(),))?;
        Ok(Self(Cow::Owned(Symbol(res.into())), ctx))
    }
    pub fn new_non_commutative<T: ToString + ?Sized>(
        ctx: &'a Context<'py>,
        name: &T,
    ) -> PyResult<Self> {
        let res = Self::class(ctx)?.call(
            (name.to_string(),),
            Some(py_dict! {ctx.gil, "commutative" => false}),
        )?;
        Ok(Self(Cow::Owned(Symbol(res.into())), ctx))
    }
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

impl<'py> Context<'py> {
    pub fn symbol<T: ToString + ?Sized>(&self, name: &T) -> PyResult<Gil<Symbol>> {
        Gil::<Symbol>::new(self, name)
    }
    pub fn symbol_non_commutative<T: ToString + ?Sized>(&self, name: &T) -> PyResult<Gil<Symbol>> {
        Gil::<Symbol>::new_non_commutative(self, name)
    }
}
