use sympy_rust::{context::Context, core::symbol::symbol::SymbolImpl};

#[test]
fn it_works() {
    let sym = Context::try_with_gil(|ctx| {
        let sym = ctx.symbol("a")?;
        println!("{}", sym.name()?);
        Ok(sym.into_inner())
    })
    .unwrap();
    println!("{}", sym.name().unwrap());
}
