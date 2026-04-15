use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    FnArg, Ident, ItemFn, LitStr, Pat, PatType, Token,
};

struct ApiDocArgs {
    group: LitStr,
    summary: LitStr,
    description: Option<LitStr>,
}

impl Parse for ApiDocArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut group = None;
        let mut summary = None;
        let mut description = None;

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
                }) => s.clone(),
                _ => {
                    return Err(syn::Error::new_spanned(
                        &pair.value,
                        "expected string literal",
                    ))
                }
            };

            match key.as_str() {
                "group" => group = Some(value),
                "summary" => summary = Some(value),
                "description" => description = Some(value),
                other => {
                    return Err(syn::Error::new_spanned(
                        &pair.path,
                        format!("unknown attribute: {other}"),
                    ))
                }
            }
        }

        Ok(ApiDocArgs {
            group: group.ok_or_else(|| input.error("missing `group` attribute"))?,
            summary: summary.ok_or_else(|| input.error("missing `summary` attribute"))?,
            description,
        })
    }
}

struct RouteAttrInfo {
    method: String,
    path: String,
    auth: Option<String>,
    path_params: Vec<String>,
    query_params: Vec<String>,
}

fn parse_route_attr(attr: &syn::Attribute) -> Option<RouteAttrInfo> {
    let path_ident = attr.path().get_ident()?;
    let name = path_ident.to_string();
    if !matches!(name.as_str(), "post" | "get" | "put" | "patch" | "delete") {
        return None;
    }

    let method = name.to_uppercase();

    if let syn::Meta::List(meta_list) = &attr.meta {
        let tokens = meta_list.tokens.clone();

        let parser = |input: ParseStream| -> syn::Result<(String, Option<String>)> {
            let path: LitStr = input.parse()?;
            let mut auth = None;

            while !input.is_empty() {
                if input.peek(Token![,]) {
                    let _: Token![,] = input.parse()?;
                }
                if input.is_empty() {
                    break;
                }
                if let Ok(ident) = input.parse::<Ident>() {
                    if ident == "auth" {
                        let _: Token![:] = input.parse()?;
                        let auth_type: Ident = input.parse()?;
                        auth = Some(auth_type.to_string());
                    }
                } else {
                    let _ = input.parse::<proc_macro2::TokenTree>();
                }
            }

            Ok((path.value(), auth))
        };

        match syn::parse::Parser::parse2(parser, tokens) {
            Ok((route_path, auth)) => {
                let (path_params, query_params) = extract_route_params(&route_path);
                Some(RouteAttrInfo {
                    method,
                    path: route_path,
                    auth,
                    path_params,
                    query_params,
                })
            }
            Err(err) => {
                panic!("failed to parse route attribute `{}`: {}", name, err);
            }
        }
    } else {
        None
    }
}

fn extract_route_params(route: &str) -> (Vec<String>, Vec<String>) {
    let mut path_params = Vec::new();
    let mut query_params = Vec::new();

    let (path_part, query_part) = if let Some(idx) = route.find('?') {
        (&route[..idx], Some(&route[idx + 1..]))
    } else {
        (route, None)
    };

    for segment in path_part.split('/') {
        if segment.starts_with('{') && segment.ends_with('}') {
            path_params.push(segment[1..segment.len() - 1].to_string());
        } else if segment.starts_with(':') {
            path_params.push(segment[1..].to_string());
        }
    }

    if let Some(qp) = query_part {
        for param in qp.split('&') {
            let param = param.trim();
            if !param.is_empty() {
                query_params.push(param.to_string());
            }
        }
    }

    (path_params, query_params)
}

fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn type_to_string(ty: &syn::Type) -> String {
    quote!(#ty).to_string().replace(" ", "")
}

fn param_ident(p: &PatType) -> &Ident {
    match &*p.pat {
        Pat::Ident(pat_ident) => &pat_ident.ident,
        other => panic!(
            "#[api_doc]: unsupported parameter pattern `{}`.",
            quote! { #other }
        ),
    }
}

fn extract_doc_comments(function: &ItemFn) -> String {
    let mut docs = Vec::new();
    for attr in &function.attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = &nv.value
                {
                    docs.push(s.value().trim().to_string());
                }
            }
        }
    }
    docs.join("\n")
}

fn extract_return_type(function: &ItemFn) -> String {
    match &function.sig.output {
        syn::ReturnType::Default => "()".to_string(),
        syn::ReturnType::Type(_, ty) => {
            let s = type_to_string(ty);
            if let Some(inner) = s.strip_prefix("Result<") {
                if let Some(inner) = inner.strip_suffix('>') {
                    return inner.to_string();
                }
            }
            s
        }
    }
}

pub fn api_doc_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = match syn::parse2::<ApiDocArgs>(attr) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error(),
    };

    let function = match syn::parse2::<ItemFn>(item.clone()) {
        Ok(f) => f,
        Err(e) => return e.to_compile_error(),
    };

    let route_info = function
        .attrs
        .iter()
        .find_map(parse_route_attr)
        .expect("#[api_doc] requires a route attribute (#[post], #[get], #[put], #[patch], #[delete])");

    let group = &args.group;
    let summary = &args.summary;
    let description = args
        .description
        .as_ref()
        .map(|d| d.value())
        .unwrap_or_else(|| extract_doc_comments(&function));
    let method = &route_info.method;
    let path = &route_info.path;
    let auth = route_info.auth.as_deref().unwrap_or("");

    let fn_params: Vec<&PatType> = function
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat_type) => Some(pat_type),
            _ => None,
        })
        .collect();

    let mut path_param_entries: Vec<TokenStream> = Vec::new();
    let mut query_param_entries: Vec<TokenStream> = Vec::new();
    let mut body_param_entries: Vec<TokenStream> = Vec::new();

    for p in &fn_params {
        let name = param_ident(p).to_string();
        let ty_str = type_to_string(&p.ty);
        let is_opt = is_option_type(&p.ty);

        if route_info.path_params.contains(&name) {
            path_param_entries.push(quote! { (#name, #ty_str) });
        } else if route_info.query_params.contains(&name) {
            query_param_entries.push(quote! { (#name, #ty_str, #is_opt) });
        } else {
            body_param_entries.push(quote! { (#name, #ty_str) });
        }
    }

    let response_type = extract_return_type(&function);

    let output = quote! {
        #[cfg(feature = "server")]
        inventory::submit! {
            crate::common::types::api_doc_meta::ApiEndpointMeta {
                method: #method,
                path: #path,
                group: #group,
                summary: #summary,
                description: #description,
                auth: #auth,
                path_params: &[#(#path_param_entries),*],
                query_params: &[#(#query_param_entries),*],
                body_params: &[#(#body_param_entries),*],
                response_type: #response_type,
            }
        }

        #function
    };

    output
}
