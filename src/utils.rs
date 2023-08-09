use std::borrow::Cow;

use pyo3::{
    prelude::*,
    types::{PyDict, PyString, PyTuple},
};

use crate::context::Context;

pub struct Gil<'py, 'a: 'py, 'b: 'py, T: Object + Clone + ?Sized>(
    pub Cow<'b, T>,
    pub &'a Context<'py>,
);

pub trait Object: Clone {
    const CLASS_NAME: &'static str;
    fn inner(&self) -> &PyObject;
    fn into_inner(self) -> PyObject;
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
    pub(crate) fn get_attr<'c: 'py, N: IntoPy<Py<PyString>>, R: FromPyObject<'py>>(
        &'c self,
        name: N,
    ) -> PyResult<R> {
        self.py_inner().getattr(name)?.extract::<R>()
    }
    pub(crate) fn set_attr<'c: 'py, N: IntoPy<Py<PyString>>, V: ToPyObject>(
        &'c self,
        name: N,
        value: V,
    ) -> PyResult<()> {
        self.py_inner().setattr(name, value)
    }
    pub(crate) fn call_method<
        'c: 'py,
        N: IntoPy<Py<PyString>>,
        A: IntoPy<Py<PyTuple>>,
        K: IntoPy<Py<PyDict>>,
        R: FromPyObject<'py>,
    >(
        &'c self,
        name: N,
        args: A,
        kw_args: Option<K>,
    ) -> PyResult<R> {
        let kw_args = kw_args
            .as_ref()
            .map(|a| a.into_py(self.1.gil).as_ref(self.1.gil));
        self.py_inner()
            .call_method(name, args, kw_args)?
            .extract::<R>()
    }
    pub(crate) fn call_method0<'c: 'py, N: IntoPy<Py<PyString>>, R: FromPyObject<'py>>(
        &'c self,
        name: N,
    ) -> PyResult<R> {
        self.py_inner().call_method0(name)?.extract::<R>()
    }
    pub(crate) fn call_method1<
        'c: 'py,
        N: IntoPy<Py<PyString>>,
        A: IntoPy<Py<PyTuple>>,
        R: FromPyObject<'py>,
    >(
        &'c self,
        name: N,
        args: A,
    ) -> PyResult<R> {
        self.py_inner().call_method1(name, args)?.extract::<R>()
    }
    pub(crate) fn call_method2<
        'c: 'py,
        N: IntoPy<Py<PyString>>,
        K: IntoPy<Py<PyDict>>,
        R: FromPyObject<'py>,
    >(
        &'c self,
        name: N,
        kw_args: K,
    ) -> PyResult<R> {
        let kw_args = kw_args.into_py(self.1.gil).as_ref(self.1.gil);
        self.py_inner()
            .call_method(name, (), Some(kw_args))?
            .extract::<R>()
    }
    pub(crate) fn class(ctx: &'a Context<'py>) -> PyResult<&'a PyAny> {
        ctx.sympy().getattr(T::CLASS_NAME)
    }
}

pub trait Config<'py>: Sized {
    fn new(ctx: &Context<'py>) -> Self;
    fn inner(&self) -> &'py PyDict;
}

#[macro_export]
macro_rules! config_fn {
    ($i:ident, $ty:ty) => {
        fn $i(self, v: $ty) -> PyResult<Self> {
            self.inner().set_item(stringify!($i), v)?;
            Ok(self)
        }
    };
}

#[macro_export]
macro_rules! method_dict {
    ($self:ident, $($id:ident),+) => {{
        let kw_args = PyDict::new($self.1.gil);
        if let Some($id) = $id {
            kw_args.set_item(stringify!($id), $id)?;
        }
        kw_args
    }};
}
