#![warn(
    clippy::as_ptr_cast_mut,
    clippy::as_underscore,
    clippy::bool_to_int_with_if,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::checked_conversions,
    clippy::clear_with_drain,
    clippy::clone_on_ref_ptr,
    clippy::cloned_instead_of_copied,
    clippy::cognitive_complexity,
    clippy::collection_is_never_read,
    clippy::copy_iterator,
    clippy::create_dir,
    clippy::default_trait_access,
    clippy::deref_by_slicing,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::enum_glob_use,
    clippy::equatable_if_let,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filetype_is_file,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::fn_to_numeric_cast_any,
    clippy::from_iter_instead_of_collect,
    clippy::future_not_send,
    clippy::get_unwrap,
    clippy::if_not_else,
    clippy::if_then_some_else_none,
    clippy::implicit_hasher,
    clippy::impl_trait_in_params,
    clippy::imprecise_flops,
    clippy::inconsistent_struct_constructor,
    clippy::index_refutable_slice,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::items_after_statements,
    clippy::iter_not_returning_iterator,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::iter_with_drain,
    clippy::large_digit_groups,
    clippy::large_futures,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::manual_assert,
    clippy::manual_clamp,
    clippy::manual_instant_elapsed,
    clippy::manual_let_else,
    clippy::manual_ok_or,
    clippy::manual_string_new,
    clippy::many_single_char_names,
    clippy::map_err_ignore,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::mismatching_type_param_order,
    clippy::missing_assert_message,
    clippy::missing_const_for_fn,
    clippy::missing_enforced_import_renames,
    clippy::multiple_unsafe_ops_per_block,
    clippy::must_use_candidate,
    clippy::mut_mut,
    clippy::naive_bytecount,
    clippy::needless_bitwise_bool,
    clippy::needless_collect,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::needless_pass_by_value,
    clippy::negative_feature_names,
    clippy::non_ascii_literal,
    clippy::non_send_fields_in_send_ty,
    clippy::or_fun_call,
    clippy::range_minus_one,
    clippy::range_plus_one,
    clippy::rc_buffer,
    clippy::redundant_closure_for_method_calls,
    clippy::redundant_else,
    clippy::redundant_feature_names,
    clippy::redundant_pub_crate,
    clippy::ref_option_ref,
    clippy::ref_patterns,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::return_self_not_must_use,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::semicolon_inside_block,
    clippy::separated_literal_suffix,
    clippy::significant_drop_in_scrutinee,
    clippy::significant_drop_tightening,
    clippy::single_match_else,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::string_slice,
    clippy::struct_excessive_bools,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::trailing_empty_array,
    clippy::trait_duplication_in_bounds,
    clippy::transmute_ptr_to_ptr,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::trivially_copy_pass_by_ref,
    clippy::try_err,
    clippy::type_repetition_in_bounds,
    clippy::unchecked_duration_subtraction,
    clippy::undocumented_unsafe_blocks,
    clippy::unicode_not_nfc,
    clippy::uninlined_format_args,
    clippy::unnecessary_box_returns,
    clippy::unnecessary_join,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unnecessary_struct_initialization,
    clippy::unneeded_field_pattern,
    clippy::unnested_or_patterns,
    clippy::unreadable_literal,
    clippy::unsafe_derive_deserialize,
    clippy::unused_async,
    clippy::unused_peekable,
    clippy::unused_rounding,
    clippy::unused_self,
    clippy::unwrap_in_result,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_bit_mask,
    clippy::verbose_file_reads
)]
#![deny(
    clippy::derive_partial_eq_without_eq,
    clippy::match_bool,
    clippy::mem_forget,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::path_buf_push_overwrite,
    clippy::rc_mutex,
    clippy::wildcard_dependencies
)]

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, DeriveInput, FnArg, ImplItem,
    ImplItemFn, ItemImpl, Pat, ReturnType, Token,
};

fn args(f: &ImplItemFn) -> Vec<Ident> {
    f.sig
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
                Some(i.ident.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
fn remove_ctx(f: &ImplItemFn) -> Punctuated<FnArg, Token![,]> {
    f.sig
        .inputs
        .iter()
        .cloned()
        .filter(|a| {
            let FnArg::Typed(ty) = a else { return true };
            let Pat::Ident(i) = &*ty.pat else { return true };
            &*i.ident.to_string() != "ctx"
        })
        .collect()
}

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
        impl crate::utils::Object for #ident {
            const CLASS_NAME: &'static str = #class_name;
            fn inner(&self) -> &PyObject {
                &self.0
            }
            fn into_inner(self) -> PyObject {
                self.0
            }
        }
        impl Into<PyObject> for #ident {
            fn into(self) -> PyObject {
                self.0
            }
        }
    };
    out.into()
}

#[derive(Debug, FromDeriveInput, Clone)]
#[darling(attributes(config))]
struct ConfigArgs {
    ident: Ident,
}

#[proc_macro_derive(Config, attributes(object))]
pub fn derive_config(item: TokenStream) -> TokenStream {
    let ConfigArgs { ident } =
        ConfigArgs::from_derive_input(&parse_macro_input!(item as DeriveInput)).unwrap();
    let out = quote! {
        impl<'py> crate::utils::Config<'py> for #ident<'py> {
            fn new(ctx: &Context<'py>) -> Self {
                Self(PyDict::new(ctx.gil))
            }
            fn inner(&self) -> &'py PyDict {
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
    let trait_ = &item.trait_.as_ref().unwrap().1;
    let ty = format_ident!("{}", attr.to_string());
    for it in &mut new_item.items {
        let ImplItem::Fn(f) = it else {
            continue;
        };
        let ident = &f.sig.ident;
        let args = args(f);
        let has_self = f.sig.inputs.iter().any(|a| matches!(a, FnArg::Receiver(_)));
        let block = if has_self {
            parse_quote! {{
                let __s = self.to_owned();
                Context::try_with_gil(move |ctx| {
                    __s.with_ctx(&ctx).#ident(#(#args),*)
                })
            }}
        } else {
            parse_quote! {{
                Context::try_with_gil(move |ctx| {
                    Self::#ident(&#(#args),*)
                })
            }}
        };
        f.block = block;
    }
    let new_items = new_item.items;
    let out = quote! {
        #item
        impl #trait_ for #ty {
            #(#new_items)*
        }
    };
    out.into()
}

#[proc_macro_attribute]
pub fn impl_for_non_gil2(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemImpl);
    let mut new_item = item.to_owned();
    let ty = format_ident!("{}", attr.to_string());

    for it in &mut new_item.items {
        let ImplItem::Fn(f) = it else {
            continue;
        };
        let ident = &f.sig.ident;
        f.sig.inputs = remove_ctx(f);
        let args = args(f);
        let block = parse_quote! {{
            Context::try_with_gil(|ctx| {
                Gil::<#ty>::#ident(&ctx, #(#args),*).map(|a| a.into_inner())
            })
        }};
        f.block = block;
    }

    let mut ctx_item = new_item.to_owned();
    for it in &mut ctx_item.items {
        let ImplItem::Fn(f) = it else {
            continue;
        };
        let ident = &f.sig.ident;
        f.sig.inputs.insert(0, parse_quote! {&self});
        let args = args(f);
        let block = parse_quote! {{
            Gil::<#ty>::#ident(self, #(#args),*)
        }};
        f.block = block;
        let out_ty = parse_quote! { PyResult<Gil<#ty>> };
        let ReturnType::Type(_, ty) = &mut f.sig.output else {
            panic!()
        };
        *ty = out_ty;
        f.sig.ident = format_ident!(
            "{}",
            format!(
                "{}{}",
                attr.to_string().to_lowercase(),
                f.sig.ident.to_string().strip_prefix("new").unwrap()
            )
        );
    }

    let new_items = new_item.items;
    let ctx_items = ctx_item.items;
    let out = quote! {
        #item
        impl #ty {
            #(#new_items)*
        }
        impl<'py> Context<'py> {
            #(#ctx_items)*
        }
    };
    out.into()
}
