use std::sync::Arc;
use macros::Object;
use pyo3::prelude::*;

use crate::{
    context::Context,
    py_dict,
    utils::{EnsureGIL, HasGIL, IsGIL, NoGIL, Object, GIL},
};

pub trait SymbolImpl {
    fn name(&self) -> PyResult<String>;
    fn set_name<T: ToString>(&self, name: T) -> PyResult<()>;
}


#[derive(Clone, Debug, Object)]
#[object(class_name = "Symbol")]
pub struct SymbolGIL<'py, G: IsGIL + 'py = ()>(G::Inner<'py>, Context<G>);
impl<'py> SymbolGIL<'py, GIL<'py>> {
    pub fn new<'a: 'py, T: ToString>(ctx: &'a Context<GIL<'py>>, name: T) -> PyResult<Self> {
        let res = Self::class(ctx)?.call1((name.to_string(),))?;
        Ok(Self(res, ctx.to_owned()))
    }
    pub fn new_non_commutative<'a: 'py, T: ToString>(
        ctx: &'a Context<GIL<'py>>,
        name: T,
    ) -> PyResult<Self> {
        let res = Self::class(ctx)?.call(
            (name.to_string(),),
            Some(py_dict! {ctx.gil.0, "commutative" => false}),
        )?;
        Ok(Self(res, ctx.to_owned()))
    }
}
impl<'py, G: IsGIL> SymbolImpl for SymbolGIL<'py, G> {
    fn name(&self) -> PyResult<String> {
        self.inner_with_gil(move |a| a.getattr("name")?.extract::<String>())
    }
    fn set_name<T: ToString>(&self, name: T) -> PyResult<()> {
        self.inner_with_gil(move |a| a.setattr("name", name.to_string()))
    }
}
