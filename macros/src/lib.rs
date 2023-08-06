use darling::{ast::NestedMeta, FromAttributes, FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{
    parse::Parse, parse_macro_input, parse_quote, Attribute, Block, DeriveInput, FnArg, ImplItem,
    ItemImpl, Meta, Pat, Type, TypePath,
};

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

#[proc_macro_attribute]
pub fn impl_for_non_gil(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemImpl);
    let mut new_item = item.to_owned();
    let ty = format_ident!("{}", attr.to_string());
    for it in &mut new_item.items {
        let ImplItem::Fn(f) = it else {
            continue;
        };
        let ident = &f.sig.ident;
        let args = f
            .sig
            .inputs
            .iter()
            .filter_map(|a| {
                if let FnArg::Typed(ty) = a {
                    Some(&ty.pat)
                } else {
                    None
                }
            })
            .filter_map(|a| {
                if let Pat::Ident(i) = &**a {
                    Some(&i.ident)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let block = parse_quote! {{
            let s = self.to_owned();
            Context::try_with_gil(move |ctx| {
                s.with_ctx(&ctx).#ident(#(#args),*)
            })
        }};
        f.block = block;
    }
    let new_items = new_item.items;
    let out = quote! {
        #item
        impl SymbolImpl for #ty {
            #(#new_items)*
        }
    };
    out.into()
}
