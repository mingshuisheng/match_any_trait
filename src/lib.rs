use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprMatch, Pat, PatIdent, PatTupleStruct};
use syn::spanned::Spanned;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum IfType {
    Start,
    Middle,
    Last,
}

#[proc_macro]
pub fn match_any_trait(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as ExprMatch);

    let match_expr = st.expr;
    let arm_len = st.arms.len();
    let arms = st.arms;
    let mut result = proc_macro2::TokenStream::new();

    for (index, arm) in arms.iter().enumerate() {
        let expr = &arm.pat;
        let body = &arm.body;
        let if_type = if index == 0 { IfType::Start } else if index == arm_len - 1 { IfType::Last } else { IfType::Middle };
        match pat_to_token(&match_expr, expr, body, if_type, &mut result) {
            Err(error) => {
                return TokenStream::from(error.to_compile_error())
            }
            _ => {}
        }
    }

    result.into()
}

fn pat_to_token(match_expr: &Box<Expr>, pat: &Pat, body: &Box<Expr>, if_type: IfType, result: &mut proc_macro2::TokenStream) -> syn::Result<()> {
    match pat {
        Pat::TupleStruct(pat) => {
            result.extend(pat_tuple_struct_to_token(match_expr, pat, body, if_type));
            Ok(())
        }
        Pat::Ident(ident) => {
            result.extend(pat_ident_to_token(match_expr, ident, body, if_type));
            Ok(())
        }
        Pat::Or(pat_or) => {
            let mut or_token = proc_macro2::TokenStream::new();
            for (index, case) in pat_or.cases.iter().enumerate() {
                let if_type = if index == 0 && if_type == IfType::Start { IfType::Start } else { IfType::Middle };
                pat_to_token(match_expr, case, body, if_type, &mut or_token)?;
            }
            result.extend(quote!(#or_token));
            Ok(())
        }
        Pat::Const(_) => build_error(pat.span(), "const is not support"),
        Pat::Lit(_) => build_error(pat.span(), "lit is not support"),
        Pat::Macro(_) => build_error(pat.span(), "macro is not support"),
        Pat::Paren(_) => build_error(pat.span(), "paren is not support"),
        Pat::Path(_) => build_error(pat.span(), "path is not support"),
        Pat::Range(_) => build_error(pat.span(), "range is not support"),
        Pat::Reference(_) => build_error(pat.span(), "reference is not support"),
        Pat::Rest(_) => build_error(pat.span(), "rest is not support"),
        Pat::Slice(_) => build_error(pat.span(), "slice is not support"),
        Pat::Struct(_) => build_error(pat.span(), "struct is not support"),
        Pat::Tuple(_) => build_error(pat.span(), "tuple is not support"),
        Pat::Type(_) => build_error(pat.span(), "type is not support"),
        Pat::Verbatim(_) => build_error(pat.span(), "verbatim is not support"),
        Pat::Wild(_) => {
            result.extend(pat_wild_to_token(body));
            Ok(())
        }
        _ => build_error(pat.span(), "unkown expr"),
    }
}

fn build_error<T>(span: proc_macro2::Span, msg: &str) -> syn::Result<T> {
    Err(
        syn::Error::new(span, msg)
    )
}

fn pat_tuple_struct_to_token(match_expr: &Box<Expr>, pat_tuple_struct: &PatTupleStruct, body: &Box<Expr>, if_type: IfType) -> proc_macro2::TokenStream {
    let type_name = &pat_tuple_struct.path;
    let ident = &pat_tuple_struct.elems;
    let ident = ident.first().unwrap();

    let condition = quote!(
        let Some(#ident) = #match_expr.downcast_ref::<#type_name>()
    );

    let if_start = if_type_to_token(if_type);

    combine(if_start, condition, body)
}

fn pat_ident_to_token(match_expr: &Box<Expr>, pat_ident: &PatIdent, body: &Box<Expr>, if_type: IfType) -> proc_macro2::TokenStream {
    let type_name = &pat_ident.ident;
    let condition = quote!(
        let Some(_) = #match_expr.downcast_ref::<#type_name>()
    );
    let if_start = if_type_to_token(if_type);
    combine(if_start, condition, body)
}

fn pat_wild_to_token(body: &Box<Expr>) -> proc_macro2::TokenStream {
    let condition = quote!();
    let if_start = if_type_to_token(IfType::Last);
    combine(if_start, condition, body)
}

fn combine(if_start: proc_macro2::TokenStream, condition: proc_macro2::TokenStream, body: &Box<Expr>) -> proc_macro2::TokenStream {
    quote!(
        #if_start #condition {
            #body
        }
    )
}

fn if_type_to_token(if_type: IfType) -> proc_macro2::TokenStream {
    match if_type {
        IfType::Start =>
            quote!(
                if
            ),
        IfType::Middle =>
            quote!(
                else if
            ),
        IfType::Last => quote!(
            else
        )
    }
}
