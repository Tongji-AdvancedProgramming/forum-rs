use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn forum_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input_fn;
    let fname = &sig.ident;
    let inputs = sig.inputs.iter().collect::<Vec<_>>();

    let origin_output = match &sig.output {
        ReturnType::Default => quote! { () }, // 如果没有返回类型，默认为()
        ReturnType::Type(_, type_box) => quote! { #type_box }, // 直接引用返回类型
    };

    let output = quote! { -> Result<crate::response::api_response::ApiResponse<#origin_output>, crate::error::api_error::ApiError> };

    let expanded = quote! {
        #(#attrs)* #vis async fn #fname(#(#inputs),*) #output {
            use crate::response::api_response::ApiResponse;
            use crate::error::api_error::ApiError;

            #block
                .map(ApiResponse::ok)
                .map_err(Into::into)
        }
    };

    TokenStream::from(expanded)
}
