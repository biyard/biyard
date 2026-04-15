use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Token, punctuated::Punctuated, parse::ParseStream};

struct FieldDocArgs {
    en: String,
    ko: String,
}

fn parse_field_doc(attr: &syn::Attribute) -> Option<FieldDocArgs> {
    if !attr.path().is_ident("field_doc") {
        return None;
    }

    let parser = |input: ParseStream| -> syn::Result<FieldDocArgs> {
        let mut en = None;
        let mut ko = None;

        let pairs = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
        for pair in pairs {
            let key = pair
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(&pair.path, "expected identifier"))?
                .to_string();

            let value = match &pair.value {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) => s.value(),
                _ => {
                    return Err(syn::Error::new_spanned(
                        &pair.value,
                        "expected string literal",
                    ))
                }
            };

            match key.as_str() {
                "en" => en = Some(value),
                "ko" => ko = Some(value),
                other => {
                    return Err(syn::Error::new_spanned(
                        &pair.path,
                        format!("unknown field_doc key: {other}"),
                    ))
                }
            }
        }

        Ok(FieldDocArgs {
            en: en.unwrap_or_default(),
            ko: ko.unwrap_or_default(),
        })
    };

    if let syn::Meta::List(meta_list) = &attr.meta {
        syn::parse::Parser::parse2(parser, meta_list.tokens.clone()).ok()
    } else {
        None
    }
}

pub fn api_doc_schema_impl(input: TokenStream) -> TokenStream {
    let derive_input = match syn::parse2::<DeriveInput>(input) {
        Ok(di) => di,
        Err(e) => return e.to_compile_error(),
    };

    let name = &derive_input.ident;
    let name_str = name.to_string();

    // Collect field_doc entries from struct fields (enums have none)
    let doc_entries: Vec<TokenStream> = match &derive_input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => named
                .named
                .iter()
                .filter_map(|field| {
                    let field_name = field.ident.as_ref()?.to_string();
                    let doc = field.attrs.iter().find_map(parse_field_doc)?;
                    let en = &doc.en;
                    let ko = &doc.ko;
                    Some(quote! { (#field_name, #en, #ko) })
                })
                .collect(),
            _ => vec![],
        },
        _ => vec![],
    };

    quote! {
        impl #name {
            pub fn field_docs() -> api_doc_types::FieldDocs {
                &[#(#doc_entries),*]
            }
        }

        #[cfg(feature = "server")]
        inventory::submit! {
            api_doc_types::ApiSchemaEntry {
                type_name: #name_str,
                field_docs: #name::field_docs,
                schema_fn: Some(|| serde_json::to_value(schemars::schema_for!(#name)).unwrap()),
            }
        }
    }
}
