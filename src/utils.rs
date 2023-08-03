use pyo3::Python;
#[macro_export]
macro_rules! py_dict {
    {$py:ident, $($x:expr => $y:expr),*} => {{
        use pyo3::types::PyDict;
        let d = PyDict::new($py);
        $(d.set_item($x, $y)?;)*
        d
    }}
}

pub trait Object {
    type Inner;
    fn inner(&self) -> &Self::Inner;
}

pub trait GIL: Object {
    type UnGIL;
    fn un_gil(&self) -> Self::UnGIL;
}
pub trait NoGIL: Object {
    type GIL<'py>
    where
        Self: 'py;
    fn gil<'py, 'a: 'py>(&'a self, py: Python<'py>) -> Self::GIL<'py>;
}
