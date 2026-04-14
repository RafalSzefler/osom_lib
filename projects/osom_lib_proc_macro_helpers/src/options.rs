//! Holds the implementation of the options parser.

use std::cmp::Eq;
use std::hash::Hash;
use std::{borrow::Borrow, collections::HashMap};

use proc_macro2::Span;
use syn::{
    Ident, LitStr, Token, Visibility,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

pub use osom_lib_proc_macro_helpers_level2::get_options;

pub enum GetOptionsError {
    MissingKey { key: String, span: Span },
    InvalidValueType { type_: String, span: Span },
}

impl From<GetOptionsError> for syn::Error {
    fn from(value: GetOptionsError) -> Self {
        match value {
            GetOptionsError::MissingKey { key, span } => {
                let message = format!("Key [{key}] is missing.");
                syn::Error::new(span, message)
            }
            GetOptionsError::InvalidValueType { type_, span } => {
                let message = format!("Invalid type [{type_}]. Expected one of: bool, string, ident, vis.");
                syn::Error::new(span, message)
            }
        }
    }
}

pub struct Options {
    map: HashMap<OptionKey, OptionValueWithSpan>,
    span: Span,
}

impl Options {
    #[inline(always)]
    pub fn get<Q: Hash + Eq + ?Sized>(&self, key: &Q) -> Option<&OptionValueWithSpan>
    where
        OptionKey: Borrow<Q>,
    {
        self.map.get(key)
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = (&OptionKey, &OptionValueWithSpan)> {
        self.map.iter()
    }

    #[inline(always)]
    #[must_use]
    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl Parse for Options {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        struct KeyValue {
            pub key: OptionKey,
            pub value: OptionValueWithSpan,
        }

        impl Parse for KeyValue {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                let key = input.parse::<OptionKey>()?;
                let _ = input.parse::<Token![=]>()?;
                let value = input.parse::<OptionValueWithSpan>()?;
                Ok(Self { key, value })
            }
        }

        let span = input.span();
        let mut map = HashMap::new();
        if input.is_empty() {
            return Ok(Options { map, span });
        }

        let first_kvp = input.parse::<KeyValue>()?;
        map.insert(first_kvp.key, first_kvp.value);

        while !input.is_empty() {
            let _ = input.parse::<Token![,]>()?;
            let kvp = input.parse::<KeyValue>()?;
            map.insert(kvp.key, kvp.value);
        }

        Ok(Self { map, span })
    }
}

pub struct OptionKey {
    text: String,
    span: Span,
}

impl PartialEq for OptionKey {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Eq for OptionKey {}

impl Hash for OptionKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.text.hash(state);
    }
}

impl Clone for OptionKey {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            span: self.span,
        }
    }
}

impl OptionKey {
    #[inline(always)]
    #[must_use]
    pub const fn as_str(&self) -> &str {
        self.text.as_str()
    }
}

impl Borrow<str> for OptionKey {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Parse for OptionKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitStr) {
            let span = input.span();
            let value = input.parse::<LitStr>()?;
            let value = value.value();
            return Ok(OptionKey { text: value, span });
        }

        if input.peek(syn::Ident) {
            let span = input.span();
            let value = input.parse::<syn::Ident>()?;
            let value = value.to_string();
            return Ok(OptionKey { text: value, span });
        }

        Err(syn::Error::new(
            input.span(),
            "Key has to be a literal string or ident.",
        ))
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Vis {
    Public,
    PublicCrate,
    PublicSuper,
    Private,
}

pub struct OptionValueWithSpan {
    pub value: OptionValue,
    pub span: Span,
}

impl Parse for OptionValueWithSpan {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let value = input.parse::<OptionValue>()?;
        Ok(Self { value, span })
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum OptionValue {
    Bool(bool),
    Ident(Ident),
    String(String),
    Vis(Vis),
}

impl Parse for OptionValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitBool) {
            let value = input.parse::<syn::LitBool>()?;
            let value = value.value();
            return Ok(OptionValue::Bool(value));
        }

        if input.peek(syn::LitStr) {
            let value = input.parse::<LitStr>()?;
            let value = value.value();
            return Ok(OptionValue::String(value));
        }

        if input.peek(syn::Ident) {
            let value = input.parse::<syn::Ident>()?;
            return Ok(OptionValue::Ident(value));
        }

        if input.peek(Token![priv]) {
            let _ = input.parse::<Token![priv]>()?;
            return Ok(OptionValue::Vis(Vis::Private));
        }

        if input.peek(Token![pub]) {
            let vis = input.parse::<syn::Visibility>()?;
            let span = vis.span();
            let vis = match vis {
                Visibility::Public(_) => Vis::Public,
                Visibility::Restricted(vis_restricted) => {
                    let path = &vis_restricted.path;
                    if path.is_ident("super") {
                        Vis::PublicSuper
                    } else if path.is_ident("crate") {
                        Vis::PublicCrate
                    } else {
                        return Err(syn::Error::new(
                            span,
                            "Visibility can only be pub, pub(crate), pub(super) and priv.",
                        ));
                    }
                }
                Visibility::Inherited => unreachable!(),
            };
            return Ok(OptionValue::Vis(vis));
        }

        Err(syn::Error::new(
            input.span(),
            "Invalid option value. Expected a bool, string, ident or visibility",
        ))
    }
}
