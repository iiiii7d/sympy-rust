use sympy_rust::{context::Context, core::symbol::symbol::SymbolImpl};

#[test]
fn it_works() {
    Context::try_with_gil(|ctx| {
        let n = ctx.symbol("a")?.name()?;
        println!("{n}");
        Ok(())
    })
    .unwrap();
}
