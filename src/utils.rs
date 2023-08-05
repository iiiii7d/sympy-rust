use std::sync::Arc;
use pyo3::{Py, PyAny, PyResult, Python};

use crate::context::Context;
#[macro_export]
macro_rules! py_dict {
    {$py:expr, $($x:expr => $y:expr),*} => {{
        use pyo3::types::PyDict;
        let d = PyDict::new($py);
        $(d.set_item($x, $y)?;)*
        d
    }}
}

pub trait Object<'py, G: IsGIL + 'py = ()> {
    const CLASS_NAME: &'static str;
    fn inner(&self) -> &G::Inner<'py>;
    fn ctx(&self) -> &Context<G>;
    fn inner_with_gil<'a: 'py, R: 'py, F: FnOnce(&'py PyAny) -> R>(&'a self, f: F) -> R {
        self.ctx().with_gil(|ctx| {
            f(self.inner().ensure_gil(ctx.gil.0))
        })
    }
    fn class<'a: 'py>(ctx: &'a Context<G>) -> PyResult<G::Inner<'py>> {
        ctx.with_gil(|ctx| ctx.sympy().getattr(Self::CLASS_NAME).map(|a| a.into()))
    }
}
pub trait HasGIL<'py>: Clone {
    type Opp;
    fn no_gil(&self) -> Self::Opp {
        self.to_owned().into_no_gil()
    }
    fn into_no_gil(self) -> Self::Opp;
}
pub trait NoGIL: Clone {
    type Opp<'py>;
    fn gil<'py>(&self, ctx: Python<'py>) -> Self::Opp<'py> {
        self.to_owned().into_gil(ctx)
    }
    #[allow(clippy::needless_lifetimes)]
    fn into_gil<'py>(self, ctx: Python<'py>) -> Self::Opp<'py>;
}

pub(crate) trait EnsureGIL<'py>: From<&'py PyAny> {
    fn ensure_gil<'a: 'py>(&'a self, py: Python<'py>) -> &'py PyAny;
}
impl<'py> EnsureGIL<'py> for &'py PyAny {
    fn ensure_gil<'a: 'py>(&'a self, _: Python<'py>) -> &'py PyAny {
        self
    }
}
impl<'py> EnsureGIL<'py> for Py<PyAny> {
    fn ensure_gil<'a: 'py>(&'a self, py: Python<'py>) -> &'py PyAny {
        self.as_ref(py)
    }
}

#[derive(Copy, Clone)]
pub struct GIL<'py>(pub Python<'py>);
pub trait IsGIL: Copy {
    type Inner<'py>: EnsureGIL<'py>;
    fn with_ctx<'py, R: 'py, F: FnOnce(&Context<GIL<'py>>) -> R>(&self, ctx: &Context<Self>, f: F) -> R;
}
impl IsGIL for () {
    type Inner<'py> = Py<PyAny>;
    fn with_ctx<R: 'static, F: FnOnce(&Context<GIL<'py>>) -> R>(&self, ctx: &Context<Self>, f: F) -> R {
        Python::with_gil(|py| {
            let ctx = ctx.gil(py);
            f(&ctx)
        })
    }
}
impl<'py2> IsGIL for GIL<'py2> {
    type Inner<'a> = &'a PyAny;
    fn with_ctx<'py, R: 'py, F: FnOnce(&Context<GIL<'py>>) -> R>(&self, ctx: &Context<Self>, f: F) -> R {
        f(&ctx)
    }
}
