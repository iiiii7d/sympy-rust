#[macro_export]
macro_rules! py_dict {
    {$py:ident, $($x:expr => $y:expr),*} => {{
        use pyo3::types::PyDict;
        let d = PyDict::new($py);
        $(d.set_item($x, $y)?;)*
        d
    }}
}