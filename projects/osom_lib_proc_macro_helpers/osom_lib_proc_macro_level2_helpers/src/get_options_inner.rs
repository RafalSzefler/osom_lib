use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    PathSegment, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub fn get_options_inner(item: TokenStream) -> syn::Result<TokenStream> {
    let call_site = Span::call_site();
    let crate_prefix = {
        let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
        let mut segments = Punctuated::new();
        let leading_colon = if crate_name == "osom_lib_proc_macro_helpers" {
            segments.push(PathSegment {
                ident: syn::Ident::new("crate", call_site),
                arguments: syn::PathArguments::None,
            });
            None
        } else {
            segments.push(PathSegment {
                ident: syn::Ident::new("osom_lib_proc_macro_helpers", call_site),
                arguments: syn::PathArguments::None,
            });
            Some(syn::token::PathSep(call_site))
        };

        segments.push(PathSegment {
            ident: syn::Ident::new("options", call_site),
            arguments: syn::PathArguments::None,
        });

        syn::Path {
            leading_colon,
            segments,
        }
    };

    let args = syn::parse2::<Args>(item)?;
    let key = args.key;
    let expr = args.expr;
    let ty = args.type_;
    let ty_str = ty.to_token_stream().to_string();
    let option_ident: syn::Ident;
    let result_type: syn::Path;
    match ty_str.as_str() {
        "bool" | "Bool" => {
            option_ident = syn::Ident::new("bool", call_site);
            result_type = syn::parse2(quote! { bool })?;
        }
        "ident" | "Ident" => {
            option_ident = syn::Ident::new("Ident", call_site);
            result_type = syn::parse2(quote! { ::syn::Ident })?;
        }
        "string" | "String" => {
            option_ident = syn::Ident::new("String", call_site);
            result_type = syn::parse2(quote! { String })?;
        }
        "vis" | "Vis" => {
            option_ident = syn::Ident::new("Vis", call_site);
            result_type = syn::parse2(quote! { #crate_prefix :: Vis })?;
        }
        _ => {
            return Err(syn::Error::new(
                args.type_span,
                "Type has to be one of: bool, ident, string, vis.",
            ));
        }
    }

    Ok(quote! {
        {
            let _olpml2_o: & #crate_prefix ::Options = &{ #expr };
            let _olpml2_result: Result< #result_type >;
            if let Some(s) = _olpml2_o.get( #key ) {
                match &s.value {
                    #crate_prefix ::OptionValue:: #option_ident (val) => {
                        _olpml2_result = Ok(val.clone());
                    },
                    _ => {
                        _olpml2_result = Err(#crate_prefix :: GetOptionsError::InvalidValueType(s.span));
                    }
                }
            } else {
                _olpml2_result = Err(#crate_prefix :: GetOptionsError::MissingKey(_olpml2_o.span.clone()));
            }
            _olpml2_result
        }
    })
}

struct Args {
    pub expr: syn::Expr,
    pub key: syn::LitStr,
    pub type_: syn::Path,
    pub type_span: Span,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr = input.parse::<syn::Expr>()?;
        let _ = input.parse::<Token![,]>()?;
        let key = input.parse::<syn::LitStr>()?;
        let _ = input.parse::<Token![,]>()?;
        let type_span = input.span();
        let type_ = input.parse::<syn::Path>()?;
        Ok(Self {
            expr,
            key,
            type_,
            type_span,
        })
    }
}
