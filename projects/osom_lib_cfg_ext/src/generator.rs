use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::cfg_matcher::CfgMatcher;

pub fn generate_from_cfg_match(inner: &CfgMatcher) -> TokenStream {
    let mut ts = TokenStream::new();
    let mut seen_conditions = Vec::with_capacity(inner.conditional_arms.len() + 2);

    for item in &inner.conditional_arms {
        let item_cond = &item.condition.tokens;
        let real_conditions: Vec<TokenStream> = seen_conditions
            .iter()
            .map(|x| quote! { not(#x) }.to_token_stream())
            .chain([item_cond.clone()])
            .collect();
        seen_conditions.push(item_cond.clone());
        let cfg_conds = join(&real_conditions);
        let body = &item.body.tokens;
        quote! {
            #[cfg(all(#cfg_conds))]
            ::osom_lib_cfg_ext::identity!( #body );
        }
        .to_tokens(&mut ts);
    }

    if let Some(item) = &inner.any_body {
        if seen_conditions.is_empty() {
            let body = &item.tokens;
            quote! {
                ::osom_lib_cfg_ext::identity!( #body );
            }
            .to_tokens(&mut ts);
            return ts;
        }

        let real_conditions: Vec<TokenStream> = seen_conditions
            .iter()
            .map(|x| quote! { not(#x) }.to_token_stream())
            .collect();
        let cfg_conds = join(&real_conditions);
        let body = &item.tokens;
        quote! {
            #[cfg(all(#cfg_conds))]
            ::osom_lib_cfg_ext::identity!( #body );
        }
        .to_tokens(&mut ts);
    }

    ts
}

fn join(v: &[TokenStream]) -> TokenStream {
    let mut result = TokenStream::new();
    let mut iv = v.iter();
    let first = iv.next().unwrap();
    first.to_tokens(&mut result);
    for item in iv {
        quote! { , #item }.to_tokens(&mut result);
    }
    result
}
