use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::collections::HashSet;
use syn::{Attribute, ItemEnum, ItemStruct, spanned::Spanned};

pub fn reprc_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    match reprc_impl_internal(attr, item) {
        Ok(ts) => ts,
        Err(err) => err.to_compile_error(),
    }
}

#[allow(clippy::needless_pass_by_value)]
fn reprc_impl_internal(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(syn::Error::new(attr.span(), "reprc does not accept arguments"));
    }

    let item = syn::parse2::<syn::Item>(item)?;

    match &item {
        syn::Item::Enum(item_enum) => process_item(item_enum),
        syn::Item::Struct(item_struct) => process_item(item_struct),
        _ => Err(syn::Error::new(
            item.span(),
            "reprc macro attribute works with structs and enums only.",
        )),
    }
}

fn reprc_attr() -> syn::Attribute {
    syn::parse_quote! { #[repr(C)] }
}

trait ItemInfo {
    fn attrs(&mut self) -> &mut Vec<Attribute>;
    fn generic(&self) -> &syn::Generics;
    fn dependent_types(&self) -> impl Iterator<Item = &syn::Type>;
    fn ident(&self) -> &syn::Ident;
}

impl ItemInfo for ItemEnum {
    fn attrs(&mut self) -> &mut Vec<Attribute> {
        &mut self.attrs
    }

    fn generic(&self) -> &syn::Generics {
        &self.generics
    }

    fn dependent_types(&self) -> impl Iterator<Item = &syn::Type> {
        self.variants.iter().flat_map(|x| x.fields.iter().map(|y| &y.ty))
    }

    fn ident(&self) -> &syn::Ident {
        &self.ident
    }
}

impl ItemInfo for ItemStruct {
    fn attrs(&mut self) -> &mut Vec<Attribute> {
        &mut self.attrs
    }

    fn generic(&self) -> &syn::Generics {
        &self.generics
    }

    fn dependent_types(&self) -> impl Iterator<Item = &syn::Type> {
        self.fields.iter().map(|x| &x.ty)
    }

    fn ident(&self) -> &syn::Ident {
        &self.ident
    }
}

#[allow(clippy::unnecessary_wraps)]
fn process_item<T: ItemInfo + ToTokens + Clone>(item: &T) -> syn::Result<TokenStream> {
    let mut item = item.clone();
    let has_repr = item.attrs().iter().any(|x| x.meta.path().is_ident("repr"));
    if !has_repr {
        item.attrs().push(reprc_attr());
    }

    let mut result = TokenStream::new();
    item.to_tokens(&mut result);

    let mut to_check = TokenStream::new();
    let mut seen: HashSet<String> = [
        "bool", "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "isize", "usize", "()",
    ]
    .iter()
    .map(std::string::ToString::to_string)
    .collect();

    for ty in item.dependent_types() {
        let ty_name = quote! { #ty }.to_token_stream().to_string();
        if seen.contains(&ty_name) {
            continue;
        }
        seen.insert(ty_name);

        quote! {
            let _ = <#ty as ::osom_lib_reprc::traits::ReprC>::CHECK;
        }
        .to_tokens(&mut to_check);
    }

    let (impl_g, ty_g, where_) = item.generic().split_for_impl();

    let ident = item.ident();
    if to_check.is_empty() {
        quote! {
            unsafe impl #impl_g ::osom_lib_reprc::traits::ReprC for #ident #ty_g #where_ {
                const CHECK: () = ();
            }
        }
        .to_tokens(&mut result);
    } else {
        quote! {
            unsafe impl #impl_g ::osom_lib_reprc::traits::ReprC for #ident #ty_g #where_ {
                const CHECK: () = {
                    #to_check
                };
            }
        }
        .to_tokens(&mut result);
    }

    Ok(result)
}
