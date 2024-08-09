mod ast;
mod generator;

use proc_macro::TokenStream;
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, Error, Item, ItemFn, Meta, Token,
};

use crate::ast::{SdfOperatorFn, SdfOperatorKind, SdfBindgenConfig};

#[proc_macro_attribute]
pub fn sdf(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args with Punctuated::<Meta, Token![,]>::parse_terminated);
    let input = parse_macro_input!(input as Item);

    match input {
        Item::Fn(item_fn) => sdf_operator_fn(args, item_fn),
        _ => Error::new(input.span(), "macro supports only functions")
            .into_compile_error()
            .into(),
    }
}

fn sdf_operator_fn(args: Punctuated<Meta, Token![,]>, func: ItemFn) -> TokenStream {
    let bindgen_config = match SdfBindgenConfig::from_ast(&args) {
        Ok(config) => config,
        Err(e) => return e.into_compile_error().into(),
    };

    let operator = match SdfOperatorKind::from_ast(&args) {
        Ok(kind) => kind,
        Err(e) => return e.into_compile_error().into(),
    };

    let sdf_fn = match SdfOperatorFn::from_ast(&func, operator) {
        Ok(sdf_fn) => sdf_fn,
        Err(e) => return e.into_compile_error().into(),
    };

    generator::generate_operator(&sdf_fn, &bindgen_config).into()
}
