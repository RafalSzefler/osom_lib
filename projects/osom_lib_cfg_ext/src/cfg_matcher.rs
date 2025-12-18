#![allow(dead_code)]
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Token, braced, parenthesized, parse::Parse};

pub struct CfgMatchArmCondition {
    pub tokens: TokenStream,
}

impl Parse for CfgMatchArmCondition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ts = TokenStream::new();
        if input.peek(Token![_]) {
            let _ = input.parse::<Token![_]>().unwrap();
            return Ok(CfgMatchArmCondition { tokens: ts });
        }

        let content;
        let _ = parenthesized!(content in input);
        while !content.is_empty() {
            let remaining_tokens = content.parse::<proc_macro2::TokenTree>()?;
            remaining_tokens.to_tokens(&mut ts);
        }
        Ok(CfgMatchArmCondition { tokens: ts })
    }
}

pub struct CfgMatchArmBody {
    pub tokens: TokenStream,
}

impl Parse for CfgMatchArmBody {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let _ = braced!(content in input);
        let mut ts = TokenStream::new();
        while !content.is_empty() {
            let remaining_tokens = content.parse::<proc_macro2::TokenTree>()?;
            remaining_tokens.to_tokens(&mut ts);
        }
        Ok(CfgMatchArmBody { tokens: ts })
    }
}

pub struct CfgMatchArm {
    pub condition: CfgMatchArmCondition,
    pub body: CfgMatchArmBody,
    pub span: Span,
}

impl Parse for CfgMatchArm {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let condition = input.parse::<CfgMatchArmCondition>()?;
        let _ = input.parse::<Token![=]>()?;
        let _ = input.parse::<Token![>]>()?;
        let body = input.parse::<CfgMatchArmBody>()?;
        while input.parse::<Token![,]>().is_ok() {}
        Ok(CfgMatchArm {
            condition: condition,
            body: body,
            span,
        })
    }
}

impl Parse for CfgMatcher {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut arms = Vec::new();
        let mut any_body = None;
        while !input.is_empty() {
            let arm = input.parse::<CfgMatchArm>()?;
            if arm.condition.tokens.is_empty() {
                if any_body.is_none() {
                    any_body = Some(arm.body);
                } else {
                    return Err(syn::Error::new(
                        arm.span,
                        "There can be only one [_] condition in cfg_match.",
                    ));
                }
            } else {
                if any_body.is_some() {
                    return Err(syn::Error::new(
                        arm.span,
                        "The [_] condition has to be last in cfg_match.",
                    ));
                }
                arms.push(arm);
            }
        }

        Ok(CfgMatcher {
            conditional_arms: arms,
            any_body: any_body,
        })
    }
}

pub struct CfgMatcher {
    pub conditional_arms: Vec<CfgMatchArm>,
    pub any_body: Option<CfgMatchArmBody>,
}
