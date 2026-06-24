use proc_macro2::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
use syn::{parse::Parse, spanned::Spanned};

#[allow(dead_code)]
struct Config {
    emit_clone: bool,
    crate_path: syn::Path,
    type_name: syn::Path,
}

impl Parse for Config {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let flag: syn::LitBool = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let path: syn::Path = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let type_name: syn::Path = input.parse()?;

        Ok(Self {
            emit_clone: flag.value,
            crate_path: path,
            type_name,
        })
    }
}

pub fn try_clone_impl(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let config: Config = syn::parse2(attr)?;
    let mut result = TokenStream::new();

    item.to_tokens(&mut result);

    let item: syn::Item = syn::parse2(item)?;

    match &item {
        syn::Item::Struct(item_struct) => {
            result.extend(try_clone_struct(&config, item_struct));
        }
        syn::Item::Enum(item_enum) => {
            result.extend(try_clone_enum(&config, item_enum)?);
        }
        err => {
            return Err(syn::Error::new(
                err.span(),
                "try_clone macro can only be used on structs and enums",
            ));
        }
    }

    Ok(result)
}

fn try_clone_struct(config: &Config, item: &syn::ItemStruct) -> TokenStream {
    let mut result = TokenStream::new();

    let crate_path = &config.crate_path;
    let try_clone = quote! {
        #crate_path::TryClone
    };

    let type_name = &config.type_name;
    let ident = &item.ident;
    let generics = &item.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let construct_stream = {
        let mut inner_stream = TokenStream::new();

        match &item.fields {
            syn::Fields::Unit => {
                quote! {
                    #ident,
                }
                .to_tokens(&mut inner_stream);
            }
            syn::Fields::Named(fields) => {
                let mut args_cloner = TokenStream::new();
                for field in &fields.named {
                    let arg_ident = field.ident.as_ref().expect("field name expected");
                    quote_spanned! { field.span() => #arg_ident: #try_clone::try_clone(&self.#arg_ident)?, }
                        .to_tokens(&mut args_cloner);
                }
                quote! {
                    #ident { #args_cloner }
                }
                .to_tokens(&mut inner_stream);
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                let mut args_cloner = TokenStream::new();
                for (counter, field) in fields_unnamed.unnamed.iter().enumerate() {
                    let lit = proc_macro2::Literal::usize_unsuffixed(counter);
                    quote_spanned! { field.span() => #try_clone::try_clone(&self.#lit)?, }.to_tokens(&mut args_cloner);
                }
                quote! {
                    #ident(#args_cloner)
                }
                .to_tokens(&mut inner_stream);
            }
        }
        inner_stream
    };

    result.extend(quote! {
        impl #impl_generics #try_clone for #ident #ty_generics #where_clause {
            type Error = #type_name;

            fn try_clone(&self) -> Result<Self, Self::Error> {
                Ok(#construct_stream)
            }
        }
    });

    if config.emit_clone {
        result.extend(impl_clone(&config.crate_path, &item.ident, &item.generics));
    }

    result
}

fn try_clone_enum(config: &Config, item: &syn::ItemEnum) -> syn::Result<TokenStream> {
    let mut result = TokenStream::new();

    let crate_path = &config.crate_path;
    let try_clone = quote! {
        #crate_path::TryClone
    };

    let type_name = &config.type_name;
    let ident = &item.ident;
    let generics = &item.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let match_stream = {
        let mut inner_match_stream = TokenStream::new();
        if item.variants.is_empty() {
            return Err(syn::Error::new(item.span(), "try_clone does not allow empty enums"));
        }

        for arg in &item.variants {
            let arg_name = &arg.ident;

            match &arg.fields {
                syn::Fields::Unit => {
                    quote! {
                        #ident::#arg_name => #ident::#arg_name,
                    }
                    .to_tokens(&mut inner_match_stream);
                }
                syn::Fields::Named(fields) => {
                    let mut args_selector = TokenStream::new();
                    let mut args_cloner = TokenStream::new();
                    for field in &fields.named {
                        let arg_ident = field.ident.as_ref().expect("field name expected");
                        quote! { #arg_ident, }.to_tokens(&mut args_selector);
                        quote_spanned! { field.span() => #arg_ident: #try_clone::try_clone(#arg_ident)?, }
                            .to_tokens(&mut args_cloner);
                    }
                    quote! {
                        #ident::#arg_name {# args_selector } => #ident::#arg_name { #args_cloner },
                    }
                    .to_tokens(&mut inner_match_stream);
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    let mut args_selector = TokenStream::new();
                    let mut args_cloner = TokenStream::new();
                    for (counter, field) in fields_unnamed.unnamed.iter().enumerate() {
                        let arg_ident = syn::Ident::new(&format!("arg{counter}"), field.span());
                        quote! { #arg_ident, }.to_tokens(&mut args_selector);
                        quote_spanned! { field.span() => #try_clone::try_clone(#arg_ident)?, }
                            .to_tokens(&mut args_cloner);
                    }
                    quote! {
                        #ident::#arg_name(#args_selector) => #ident::#arg_name(#args_cloner),
                    }
                    .to_tokens(&mut inner_match_stream);
                }
            }
        }
        inner_match_stream
    };

    result.extend(quote! {
        impl #impl_generics #try_clone for #ident #ty_generics #where_clause {
            type Error = #type_name;

            fn try_clone(&self) -> Result<Self, Self::Error> {
                Ok(match self {
                    #match_stream
                })
            }
        }
    });

    if config.emit_clone {
        result.extend(impl_clone(&config.crate_path, &item.ident, &item.generics));
    }

    Ok(result)
}

fn impl_clone(crate_path: &syn::Path, ident: &syn::Ident, generics: &syn::Generics) -> TokenStream {
    let mut result = TokenStream::new();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let message = format!("[{ident}::try_clone] should not fail.");
    let try_clone = quote! { #crate_path::TryClone };
    result.extend(quote! {
        impl #impl_generics Clone for #ident #ty_generics #where_clause {
            fn clone(&self) -> Self {
                #try_clone::try_clone(self).expect(#message)
            }
        }
    });
    result
}
