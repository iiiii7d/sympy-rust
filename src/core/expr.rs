use std::borrow::Cow;

use duplicate::duplicate_item;
use macros::{impl_for_non_gil, impl_for_non_gil2, Object};
use pyo3::{prelude::*, types::PyTuple};

use crate::{
    context::Context,
    core::wild::Wild,
    prelude::Symbol,
    utils::{Config, Gil, Object},
};

#[derive(Clone, Debug, Object)]
#[object(class_name = "Expr")]
pub struct Expr(pub(crate) PyObject);

#[impl_for_non_gil2(Expr)]
impl<'py, 'a, 'b> Gil<'py, 'a, 'b, Expr> {
    pub fn new<T: IntoPy<Py<PyTuple>>>(ctx: &'a Context<'py>, args: T) -> PyResult<Self> {
        let res = Self::class(ctx)?.call1(args)?;
        Ok(Self(Cow::Owned(Expr(res.into())), ctx))
    }
}

pub trait ExprImpl {}

#[duplicate_item(
  Struct; [Expr]; [Symbol]; [Wild];
)]
#[impl_for_non_gil(Struct)]
impl<'py, 'a, 'b> ExprImpl for Gil<'py, 'a, 'b, Struct> {}
