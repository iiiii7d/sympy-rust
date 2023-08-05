use pyo3::prelude::*;

use crate::utils::{HasGIL, IsGIL, NoGIL, GIL};

#[derive(Clone, Debug)]
pub struct Context<G: IsGIL = ()> {
    pub(crate) gil: G,
    pub(crate) sympy: Py<PyModule>,
}

impl<'py> Context<GIL<'py>> {
    pub(crate) fn sympy<'a: 'py>(&'a self) -> &'py PyModule {
        self.sympy.as_ref(self.gil.0)
    }
}
impl<'py, G: IsGIL> Context<G> {
    pub fn with_gil<R: 'py, F: FnOnce(&Context<GIL<'py>>) -> R>(&self, f: F) -> R {
        self.gil.with_ctx(self, f)
    }
}

impl<'py> HasGIL<'py> for Context<GIL<'py>> {
    type Opp = Context<()>;
    fn into_no_gil(self) -> Self::Opp {
        Context {
            gil: (),
            sympy: self.sympy,
        }
    }
}
impl NoGIL for Context {
    type Opp<'py> = Context<GIL<'py>>;
    #[allow(clippy::needless_lifetimes)]
    fn into_gil<'py>(self, py: Python<'py>) -> Self::Opp<'py> {
        Context {
            gil: GIL(py),
            sympy: self.sympy,
        }
    }
}
