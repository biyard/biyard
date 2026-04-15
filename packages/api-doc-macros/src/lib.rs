extern crate proc_macro;

mod api_doc;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn api_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    api_doc::api_doc_impl(attr.into(), item.into()).into()
}
