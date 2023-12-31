use std::borrow::Cow;

use macros::{impl_for_non_gil2, Config, Object};
use pyo3::{prelude::*, types::PyDict};

use crate::{
    config_fn,
    context::Context,
    utils::{Config, Gil},
};

#[derive(Clone, Debug, Object)]
#[object(class_name = "Wild")]
pub struct Wild(pub(crate) PyObject);

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

pub trait WildConfigImpl<'py>: Config<'py> {
    config_fn!(exclude, &[impl ToPyObject]); // todo
    config_fn!(properties, &[impl ToPyObject]); // todo
}

impl<'py> WildConfigImpl<'py> for WildConfig<'py> {}
