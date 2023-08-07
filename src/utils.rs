use std::borrow::Cow;

use pyo3::prelude::*;

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

pub struct Gil<'py, 'a: 'py, 'b: 'py, T: Object + Clone + ?Sized>(
    pub Cow<'b, T>,
    pub &'a Context<'py>,
);

pub trait Object: Clone {
    const CLASS_NAME: &'static str;
    fn inner(&self) -> &PyObject;
    fn with_ctx<'py, 'a: 'py, 'b: 'py>(&'b self, ctx: &'a Context<'py>) -> Gil<'py, 'a, 'b, Self> {
        Gil(Cow::Borrowed(self), ctx)
    }
}

impl<'py, 'a, 'b, T: Object + Clone + ?Sized> Gil<'py, 'a, 'b, T> {
    pub const fn inner(&self) -> &Cow<T> {
        &self.0
    }
    pub fn into_inner(self) -> T {
        self.0.into_owned()
    }
    pub fn py_inner<'c: 'py>(&'c self) -> &'py PyAny {
        self.inner().inner().as_ref(self.1.gil)
    }
    pub(crate) fn class(ctx: &'a Context<'py>) -> PyResult<&'a PyAny> {
        ctx.sympy().getattr(T::CLASS_NAME)
    }
}
