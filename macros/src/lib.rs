use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromDeriveInput, Clone)]
#[darling(attributes(object))]
struct ObjectArgs {
    ident: Ident,
    class_name: String,
}

#[proc_macro_derive(Object, attributes(object))]
pub fn derive_object(item: TokenStream) -> TokenStream {
    let ObjectArgs { ident, class_name } =
        ObjectArgs::from_derive_input(&parse_macro_input!(item as DeriveInput)).unwrap();
    let out = quote! {
        impl<'py, G: IsGIL> Object<'py, G> for #ident<'py, G> {
            const CLASS_NAME: &'static str = #class_name;
            fn inner(&self) -> &G::Inner<'py> {
                &self.0
            }
        }
        impl<'py> HasGIL<'py> for #ident<'py, GIL<'py>> {
            type Opp = #ident<'py, ()>;
            fn into_no_gil(self) -> Self::Opp {
                #ident(self.0.into(), ())
            }
        }
        impl NoGIL for #ident<'_> {
            type Opp<'py> = #ident<'py, GIL<'py>>;
            #[allow(clippy::needless_lifetimes)]
            fn into_gil<'py>(self, py: Python<'py>) -> Self::Opp<'py> {
                #ident(self.0.as_ref(py), GIL(py))
            }
        }
    };
    out.into()
}
