use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GIL, attributes(gil))]
pub fn derive_gil(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as DeriveInput);
    let gil = &args.ident;
    let no_gil = format_ident!("{}", args.ident.to_string().strip_suffix("GIL").unwrap());
    let out = quote! {
        impl<'a> Object for #gil<'a> {
            type Inner = PyAny;
            fn inner(&self) -> &Self::Inner {
                &self.0
            }
        }
        impl<'a> GIL for #gil<'a> {
            type UnGIL = #no_gil;
            fn un_gil(&self) -> Self::UnGIL {
                #no_gil(self.inner().into())
            }
        }
    };
    out.into()
}

#[proc_macro_derive(NoGIL, attributes(gil))]
pub fn derive_no_gil(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as DeriveInput);
    let no_gil = &args.ident;
    let gil = format_ident!("{}GIL", args.ident);
    let out = quote! {
        impl Object for #no_gil {
            type Inner = Py<PyAny>;
            fn inner(&self) -> &Self::Inner {
                &self.0
            }
        }
        impl NoGIL for #no_gil {
            type GIL<'py> = #gil<'py>;
            fn gil<'py, 'a: 'py>(&'a self, py: Python<'py>) -> Self::GIL<'py> {
                #gil(self.inner().as_ref(py))
            }
        }
    };
    out.into()
}
