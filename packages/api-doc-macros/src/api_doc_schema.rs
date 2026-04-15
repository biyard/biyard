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

fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn rust_type_to_schema_str(ty: &syn::Type) -> String {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let ident = segment.ident.to_string();
            return match ident.as_str() {
                "String" => "string".to_string(),
                "bool" => "boolean".to_string(),
                "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "isize" => "integer".to_string(),
                "f32" | "f64" => "number".to_string(),
                "Option" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                            return rust_type_to_schema_str(inner);
                        }
                    }
                    "any".to_string()
                }
                "Vec" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                            return format!("{}[]", rust_type_to_schema_str(inner));
                        }
                    }
                    "array".to_string()
                }
                "Partition" | "ProjectPartition" => "string".to_string(),
                _ => "object".to_string(),
            };
        }
    }
    "any".to_string()
}

pub fn api_doc_schema_impl(input: TokenStream) -> TokenStream {
    let derive_input = match syn::parse2::<DeriveInput>(input) {
        Ok(di) => di,
        Err(e) => return e.to_compile_error(),
    };

    let struct_name = &derive_input.ident;
    let struct_name_str = struct_name.to_string();

    let fields = match &derive_input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => &named.named,
            _ => {
                return syn::Error::new_spanned(
                    struct_name,
                    "ApiDocSchema only supports named fields",
                )
                .to_compile_error()
            }
        },
        _ => {
            return syn::Error::new_spanned(struct_name, "ApiDocSchema only supports structs")
                .to_compile_error()
        }
    };

    let mut doc_entries: Vec<TokenStream> = Vec::new();
    let mut meta_entries: Vec<TokenStream> = Vec::new();

    for field in fields {
        let field_name = match &field.ident {
            Some(ident) => ident.to_string(),
            None => continue,
        };

        let type_str = rust_type_to_schema_str(&field.ty);
        let required = !is_option_type(&field.ty);

        let doc = field.attrs.iter().find_map(parse_field_doc);

        if let Some(doc) = doc {
            let en = &doc.en;
            let ko = &doc.ko;
            doc_entries.push(quote! { (#field_name, #en, #ko) });
            meta_entries.push(quote! {
                (#field_name, #type_str, #required, #en, #ko)
            });
        } else {
            meta_entries.push(quote! {
                (#field_name, #type_str, #required, "", "")
            });
        }
    }

    let output = quote! {
        impl #struct_name {
            pub fn field_docs() -> &'static [(&'static str, &'static str, &'static str)] {
                &[#(#doc_entries),*]
            }

            pub fn field_meta() -> &'static [(&'static str, &'static str, bool, &'static str, &'static str)] {
                &[#(#meta_entries),*]
            }
        }

        #[cfg(feature = "server")]
        inventory::submit! {
            api_doc_types::ApiSchemaEntry {
                type_name: #struct_name_str,
                field_meta: #struct_name::field_meta,
            }
        }
    };

    output
}
