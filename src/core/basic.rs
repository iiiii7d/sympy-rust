use std::{borrow::Cow, cmp::Ordering, collections::HashSet};

use duplicate::duplicate_item;
use macros::{impl_for_non_gil, impl_for_non_gil2, Config, Object};
use pyo3::{
    prelude::*,
    types::{PyDict, PySet, PyTuple},
};

use crate::{
    config_fn,
    context::Context,
    core::wild::{Wild, WildConfig},
    method_dict,
    prelude::{Expr, Symbol},
    utils::{Config, Gil, Object},
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
    fn class_key(ctx: &Context) -> PyResult<PyObject>;
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
    fn from_iter(
        ctx: &Context,
        args: Py<PyAny>,
        assumptions: Option<Py<PyDict>>,
    ) -> PyResult<Basic>;
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
//#[impl_for_non_gil(Struct)]
impl<'py, 'a, 'b> BasicImpl for Gil<'py, 'a, 'b, Struct> {
    fn args(&self) -> PyResult<Vec<Basic>> {
        self.get_attr::<_, Vec<PyObject>>("args")
            .map(|a| a.into_iter().map(Basic).collect())
    }
    fn as_content_primitive(&self, radical: Option<bool>, clear: Option<bool>) -> PyResult<Basic> {
        let kw_args = method_dict!(self, radical, clear);
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
        let args = types
            .iter()
            .cloned()
            .map(|a| Basic::try_from(a).unwrap().into_inner())
            .collect::<Vec<PyObject>>();
        self.call_method1("atoms", PyTuple::new(self.1.gil, args))
    }
    fn canonical_variables(&self) -> PyResult<Py<PyDict>> {
        self.get_attr("canonical_variables")
    }
    fn class_key(ctx: &Context) -> PyResult<PyObject> {
        Self::class(ctx)?.call_method0("class_key")?.extract()
    }
    fn compare<B: Into<Basic>>(&self, other: B) -> PyResult<Ordering> {
        let res: i8 =
            self.call_method1("compare", (Basic::try_from(other).unwrap().into_inner(),))?;
        Ok(match res {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            _ => panic!(),
        })
    }
    fn count<B: Into<Basic>>(&self, query: B) -> PyResult<usize> {
        self.call_method1("count", (Basic::try_from(query).unwrap().into_inner(),))
    }
    fn count_ops(&self, visual: Option<bool>) -> PyResult<usize> {
        let kw_args = PyDict::new(self.1.gil);
        if let Some(radical) = visual {
            kw_args.set_item("visual", radical)?;
        }
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
        let d = PyDict::new(self.1.gil);
        if let Some(symbol) = symbol {
            d.set_item("symbol", symbol)?;
        }
        self.call_method2("dummy_eq", d)
    }
    fn find<B: Into<Basic>>(&self, query: B, group: Option<bool>) -> PyResult<PyObject> {
        let d = PyDict::new(self.1.gil);
        if let Some(group) = group {
            d.set_item("group", group)?;
        }
        self.call_method(
            "dummy_eq",
            (Basic::try_from(query).unwrap().into_inner(),),
            Some(d),
        )
    }
    fn free_symbols(&self) -> PyResult<Py<PySet>> {
        self.get_attr("free_symbols")
    }
    fn from_iter(
        ctx: &Context,
        args: Py<PyAny>,
        assumptions: Option<Py<PyDict>>,
    ) -> PyResult<Basic> {
        let assumptions = assumptions.map(|a| a.into_ref(ctx.gil));
        Self::class(ctx)?
            .call_method("fromiter", (args,), assumptions)?
            .extract()
            .map(Basic)
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
        let s = s
            .iter()
            .cloned()
            .map(|a| Basic::try_from(a).unwrap().into_inner())
            .collect::<Vec<PyObject>>();
        self.call_method1("has_xfree", PyTuple::new(self.1.gil, s))
    }
    fn is_comparable(&self) -> PyResult<bool> {
        self.get_attr("is_comparable")
    }
    fn match_<B: Into<Basic>>(&self, pattern: B, old: Option<bool>) -> PyResult<Py<PyDict>> {
        let d = PyDict::new(self.1.gil);
        if let Some(old) = old {
            d.set_item("old", old)?;
        }
        self.call_method(
            "match",
            (Basic::try_from(pattern).unwrap().into_inner(),),
            Some(d),
        )
    }
    fn matches<B: Into<Basic>>(
        &self,
        expr: B,
        repl_dict: Option<bool>,
        old: Option<bool>,
    ) -> PyResult<bool> {
        let d = PyDict::new(self.1.gil);
        if let Some(repl_dict) = repl_dict {
            d.set_item("repl_dict", repl_dict)?;
        }
        if let Some(old) = old {
            d.set_item("old", old)?;
        }
        self.call_method(
            "match",
            (Basic::try_from(expr).unwrap().into_inner(),),
            Some(d),
        )
    }
    fn r_call(&self, args: Py<PyTuple>) -> PyResult<Basic> {
        self.call_method1("rcall", args.as_ref(self.1.gil))
            .map(Basic)
    }
    fn refine(&self, assumption: Option<bool>) -> PyResult<Py<PyAny>> {
        let d = PyDict::new(self.1.gil);
        if let Some(assumption) = assumption {
            d.set_item("assumption", assumption)?;
        }
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
        let d = PyDict::new(self.1.gil);
        if let Some(map) = map {
            d.set_item("map", map)?;
        }
        if let Some(simultaneous) = simultaneous {
            d.set_item("simultaneous", simultaneous)?;
        }
        if let Some(exact) = exact {
            d.set_item("exact", exact)?;
        }
        self.call_method("match", (query, value), Some(d))
            .map(Basic)
    }
    fn rewrite(&self, args: Py<PyTuple>, deep: Option<bool>, hints: Py<PyDict>) -> PyResult<Basic> {
    }
    fn simplify(&self, kw_args: Py<PyDict>) -> PyResult<Basic> {}
    fn sort_key(&self, order: Option<PyObject>) -> PyResult<PyObject> {}
    fn subs(&self, args: Py<PyTuple>, kw_args: Py<PyDict>) -> PyResult<Basic> {}
    fn x_replace(&self, rule: Py<PyDict>) -> PyResult<Basic> {}
}
