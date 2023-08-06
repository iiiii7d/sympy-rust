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
        impl Object for #ident {
            const CLASS_NAME: &'static str = #class_name;
            fn inner(&self) -> &PyObject {
                &self.0
            }
        }
    };
    out.into()
}
