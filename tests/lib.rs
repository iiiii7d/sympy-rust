use pyo3::PyResult;
use sympy_rust::{
    context::Context,
    core::symbol::symbol::{Symbol, SymbolImpl},
};

#[test]
fn it_works() -> PyResult<()> {
    let x = Symbol::new("x")?;
    println!("{}", x.name()?);
    x.set_name("y")?;
    println!("{}", x.name()?);
    Ok(())
}
