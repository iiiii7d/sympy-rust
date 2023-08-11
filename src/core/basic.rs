use std::{borrow::Cow, cmp::Ordering};

use duplicate::duplicate_item;
use macros::{impl_for_non_gil, impl_for_non_gil2, Object};
use pyo3::{
    prelude::*,
    types::{PyDict, PySet, PyTuple},
};

use crate::{
    context::Context,
    core::wild::Wild,
    method_args,
    prelude::{Expr, Symbol},
    utils::{Gil, Object},
};

#[derive(Clone, Debug, Object)]
#[object(class_name = "Basic")]
pub struct Basic(pub(crate) PyObject);

#[impl_for_non_gil2(Basic)]
impl<'py, 'a, 'b> Gil<'py, 'a, 'b, Basic> {
    pub fn new<T: IntoPy<Py<PyTuple>>>(ctx: &'a Context<'py>, args: T) -> PyResult<Self> {
        let res = Self::class(ctx)?.call1(args)?;
        Ok(Self(Cow::Owned(Basic(res.into())), ctx))
    }
}

pub trait BasicImpl: Sized {
    fn args(&self) -> PyResult<Vec<Basic>>;
    fn as_content_primitive(&self, radical: Option<bool>, clear: Option<bool>) -> PyResult<Basic>;
    fn as_dummy(&self) -> PyResult<Basic>;
    fn assumptions0(&self) -> PyResult<Py<PyDict>>;
    fn atoms<B: Into<Basic> + Clone>(&self, types: &[B]) -> PyResult<Py<PySet>>;
    fn canonical_variables(&self) -> PyResult<Py<PyDict>>;
    fn compare<B: Into<Basic>>(&self, other: B) -> PyResult<Ordering>;
    fn count<B: Into<Basic>>(&self, query: B) -> PyResult<usize>; // todo
    fn count_ops(&self, visual: Option<bool>) -> PyResult<usize>; // todo
    fn do_it(&self, hints: Py<PyDict>) -> PyResult<Basic>;
    fn dummy_eq<B: Into<Basic>>(
        &self,
        other: B,
        symbol: Option</*Symbol*/ PyObject>,
    ) -> PyResult<bool>; // todo
    fn find<B: Into<Basic>>(&self, query: B, group: Option<bool>) -> PyResult<PyObject>;
    fn free_symbols(&self) -> PyResult<Py<PySet>>;
    fn func(&self) -> PyResult<PyObject>;
    fn has(&self, patterns: Py<PyTuple>) -> PyResult<bool>;
    fn has_free(&self, patterns: Py<PyTuple>) -> PyResult<bool>;
    fn has_x_free<B: Into<Basic> + Clone>(&self, s: &[B]) -> PyResult<bool>;
    fn is_comparable(&self) -> PyResult<bool>;
    fn match_<B: Into<Basic>>(&self, pattern: B, old: Option<bool>) -> PyResult<Py<PyDict>>;
    fn matches<B: Into<Basic>>(
        &self,
        expr: B,
        repl_dict: Option<bool>,
        old: Option<bool>,
    ) -> PyResult<bool>;
    fn r_call(&self, args: Py<PyTuple>) -> PyResult<Basic>;
    fn refine(&self, assumption: Option<bool>) -> PyResult<Py<PyAny>>;
    fn replace(
        &self,
        query: PyObject,
        value: PyObject,
        map: Option<bool>,
        simultaneous: Option<bool>,
        exact: Option<bool>,
    ) -> PyResult<Basic>;
    fn rewrite(&self, args: Py<PyTuple>, deep: Option<bool>, hints: Py<PyDict>) -> PyResult<Basic>;
    fn simplify(&self, kw_args: Py<PyDict>) -> PyResult<Basic>;
    fn sort_key(&self, order: Option<PyObject>) -> PyResult<PyObject>;
    fn subs(&self, args: Py<PyTuple>, kw_args: Py<PyDict>) -> PyResult<Basic>;
    fn x_replace(&self, rule: Py<PyDict>) -> PyResult<Basic>;
}

#[duplicate_item(
  Struct; [Basic]; [Expr]; [Symbol]; [Wild];
)]
#[impl_for_non_gil(Struct)]
impl<'py, 'a, 'b> BasicImpl for Gil<'py, 'a, 'b, Struct> {
    fn args(&self) -> PyResult<Vec<Basic>> {
        self.get_attr::<_, Vec<PyObject>>("args")
            .map(|a| a.into_iter().map(Basic).collect())
    }
    fn as_content_primitive(&self, radical: Option<bool>, clear: Option<bool>) -> PyResult<Basic> {
        let kw_args = method_args!(dict self, radical, clear);
        self.call_method2("as_content_primitive", kw_args)
            .map(Basic)
    }
    fn as_dummy(&self) -> PyResult<Basic> {
        self.call_method0("as_dummy").map(Basic)
    }
    fn assumptions0(&self) -> PyResult<Py<PyDict>> {
        self.get_attr("assumptions0")
    }
    fn atoms<B: Into<Basic> + Clone>(&self, types: &[B]) -> PyResult<Py<PySet>> {
        let args = method_args!(tuple self, types, Basic);
        self.call_method1("atoms", args)
    }
    fn canonical_variables(&self) -> PyResult<Py<PyDict>> {
        self.get_attr("canonical_variables")
    }
    fn compare<B: Into<Basic>>(&self, other: B) -> PyResult<Ordering> {
        let res: i8 = self.call_method1("compare", (method_args!(conv other, Basic),))?;
        Ok(match res {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            _ => panic!(),
        })
    }
    fn count<B: Into<Basic>>(&self, query: B) -> PyResult<usize> {
        self.call_method1("count", (method_args!(conv query, Basic),))
    }
    fn count_ops(&self, visual: Option<bool>) -> PyResult<usize> {
        let kw_args = method_args!(dict self, visual);
        self.call_method2("count_ops", kw_args)
    }
    fn do_it(&self, hints: Py<PyDict>) -> PyResult<Basic> {
        self.call_method2("doit", hints.as_ref(self.1.gil))
            .map(Basic)
    }
    fn dummy_eq<B: Into<Basic>>(
        &self,
        other: B,
        symbol: Option</*Symbol*/ PyObject>,
    ) -> PyResult<bool> {
        let d = method_args!(dict self, symbol);
        self.call_method("dummy_eq", (method_args!(conv other, Basic),), Some(d))
    }
    fn find<B: Into<Basic>>(&self, query: B, group: Option<bool>) -> PyResult<PyObject> {
        let d = method_args!(dict self, group);
        self.call_method("dummy_eq", (method_args!(conv query, Basic),), Some(d))
    }
    fn free_symbols(&self) -> PyResult<Py<PySet>> {
        self.get_attr("free_symbols")
    }
    fn func(&self) -> PyResult<PyObject> {
        self.get_attr("func")
    }
    fn has(&self, patterns: Py<PyTuple>) -> PyResult<bool> {
        self.call_method1("has", patterns.into_ref(self.1.gil))
    }
    fn has_free(&self, patterns: Py<PyTuple>) -> PyResult<bool> {
        self.call_method1("has_free", patterns.into_ref(self.1.gil))
    }
    fn has_x_free<B: Into<Basic> + Clone>(&self, s: &[B]) -> PyResult<bool> {
        let args = method_args!(tuple self, s, Basic);
        self.call_method1("has_xfree", args)
    }
    fn is_comparable(&self) -> PyResult<bool> {
        self.get_attr("is_comparable")
    }
    fn match_<B: Into<Basic>>(&self, pattern: B, old: Option<bool>) -> PyResult<Py<PyDict>> {
        let d = method_args!(dict self, old);
        self.call_method("match", (method_args!(conv pattern, Basic),), Some(d))
    }
    fn matches<B: Into<Basic>>(
        &self,
        expr: B,
        repl_dict: Option<bool>,
        old: Option<bool>,
    ) -> PyResult<bool> {
        let d = method_args!(dict self, repl_dict, old);
        self.call_method("match", (method_args!(conv expr, Basic),), Some(d))
    }
    fn r_call(&self, args: Py<PyTuple>) -> PyResult<Basic> {
        self.call_method1("rcall", args.as_ref(self.1.gil))
            .map(Basic)
    }
    fn refine(&self, assumption: Option<bool>) -> PyResult<Py<PyAny>> {
        let d = method_args!(dict self, assumption);
        self.call_method2("refine", d)
    }
    fn replace(
        &self,
        query: PyObject,
        value: PyObject,
        map: Option<bool>,
        simultaneous: Option<bool>,
        exact: Option<bool>,
    ) -> PyResult<Basic> {
        let d = method_args!(dict self, map, simultaneous, exact);
        self.call_method("match", (query, value), Some(d))
            .map(Basic)
    }
    fn rewrite(&self, args: Py<PyTuple>, deep: Option<bool>, hints: Py<PyDict>) -> PyResult<Basic> {
        let d = method_args!(dict self, deep);
        d.update(hints.as_ref(self.1.gil).as_mapping())?;
        self.call_method("rewrite", args.as_ref(self.1.gil), Some(d))
            .map(Basic)
    }
    fn simplify(&self, kw_args: Py<PyDict>) -> PyResult<Basic> {
        self.call_method2("simplify", kw_args.as_ref(self.1.gil))
            .map(Basic)
    }
    fn sort_key(&self, order: Option<PyObject>) -> PyResult<PyObject> {
        let d = method_args!(dict self, order);
        self.call_method2("sort_key", d)
    }
    fn subs(&self, args: Py<PyTuple>, kw_args: Py<PyDict>) -> PyResult<Basic> {
        self.call_method(
            "subs",
            args.as_ref(self.1.gil),
            Some(kw_args.as_ref(self.1.gil)),
        )
        .map(Basic)
    }
    fn x_replace(&self, rule: Py<PyDict>) -> PyResult<Basic> {
        self.call_method2("xreplace", rule.as_ref(self.1.gil))
            .map(Basic)
    }
}

pub trait BasicImplGil<'a, 'py>: Sized {
    fn class_key(ctx: &'a Context<'py>) -> PyResult<PyObject>;
    fn from_iter(
        ctx: &'a Context<'py>,
        args: Py<PyAny>,
        assumptions: Option<Py<PyDict>>,
    ) -> PyResult<Self>;
}
#[duplicate_item(
  Struct; [Basic]; [Expr]; [Symbol]; [Wild];
)]
impl<'py, 'a, 'b> BasicImplGil<'a, 'py> for Gil<'py, 'a, 'b, Struct> {
    fn class_key(ctx: &'a Context<'py>) -> PyResult<PyObject> {
        Gil::<Struct>::class(ctx)?
            .call_method0("class_key")?
            .extract()
    }
    fn from_iter(
        ctx: &'a Context<'py>,
        args: Py<PyAny>,
        assumptions: Option<Py<PyDict>>,
    ) -> PyResult<Self> {
        let assumptions = assumptions.map(|a| a.into_ref(ctx.gil));
        Gil::<Struct>::class(ctx)?
            .call_method("fromiter", (args,), assumptions)?
            .extract()
            .map(|a| Struct(a).into_with_ctx(ctx))
    }
}

pub trait BasicImplNoGil: Sized {
    fn class_key() -> PyResult<PyObject>;
    fn from_iter(args: Py<PyAny>, assumptions: Option<Py<PyDict>>) -> PyResult<Self>;
}
#[duplicate_item(
  Struct; [Basic]; [Expr]; [Symbol]; [Wild];
)]
impl BasicImplNoGil for Struct {
    fn class_key() -> PyResult<PyObject> {
        Context::try_with_gil(|ctx| Gil::<Self>::class_key(&ctx))
    }
    fn from_iter(args: Py<PyAny>, assumptions: Option<Py<PyDict>>) -> PyResult<Self> {
        Context::try_with_gil(|ctx| {
            Gil::<Self>::from_iter(&ctx, args, assumptions).map(Gil::into_inner)
        })
    }
}
