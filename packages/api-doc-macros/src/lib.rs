extern crate proc_macro;

mod api_doc;
mod api_doc_schema;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn api_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    api_doc::api_doc_impl(attr.into(), item.into()).into()
}

#[proc_macro_derive(ApiDocSchema, attributes(field_doc))]
pub fn api_doc_schema_derive(input: TokenStream) -> TokenStream {
    api_doc_schema::api_doc_schema_impl(input.into()).into()
}

