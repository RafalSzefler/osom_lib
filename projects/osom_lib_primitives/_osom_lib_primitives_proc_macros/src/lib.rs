//! A private crate that exposes macros for osom_lib_primitives crate.
#![deny(warnings)]
#![allow(unused_features)]
#![doc(hidden)]
#![warn(clippy::all, clippy::pedantic)]

use quote::quote;

/// This macro works as `?` operator, but for `CResult` type.
///
/// Use it until `Try` trait is stabilized.
#[proc_macro]
pub fn try_unpack(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expression = match syn::parse::<syn::Expr>(tokens) {
        Ok(ok) => ok,
        Err(err) => {
            return err.into_compile_error().into();
        }
    };

    quote! {
        {
            use ::osom_lib_primitives::cresult::CResult;

            match { #expression } {
                CResult::Ok(ok) => ok,
                CResult::Err(err) => {
                    return CResult::Err(err);
                }
            }
        }
    }
    .into()
}

/// Creates `Length` out of passed expression. If the expression is a literal
/// integer value, then this will check the value at compile time instead
/// of runtime. Otherwise it will produce potential runtime validity checks,
/// depending on compiler's optimizations.
#[proc_macro]
pub fn make_length(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expression = match syn::parse::<syn::Expr>(tokens) {
        Ok(ok) => ok,
        Err(err) => {
            return err.into_compile_error().into();
        }
    };

    let org_expr = unpack_brackets(&expression);
    let (sign, expr) = unpack_negative_sign(org_expr);

    match expr {
        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Int(lit_int) => {
                let Ok(value) = lit_int.base10_parse::<u32>() else {
                    return quote! {
                        compile_error!("Invalid literal integer.");
                    }
                    .into();
                };

                if sign < 0 {
                    return quote! {
                        compile_error!("Length has to be non-negative.");
                    }
                    .into();
                }

                if value == 0 {
                    return quote! {
                        ::osom_lib_primitives::length::Length::ZERO
                    }
                    .into();
                }

                if value == 1 {
                    return quote! {
                        ::osom_lib_primitives::length::Length::ONE
                    }
                    .into();
                }

                quote! {
                    {
                        const {
                            match ::osom_lib_primitives::length::Length::try_from_u32(#value) {
                                Ok(val) => val,
                                Err(_) => panic!("The literal value is above Length::MAX_LENGTH."),
                            }
                        }
                    }
                }
                .into()
            }
            _ => quote! {
                compile_error!("Expected literal integer.");
            }
            .into(),
        },
        _ => quote! {
            {
                ::osom_lib_primitives::length::Length::try_from_u32(#org_expr).unwrap()
            }
        }
        .into(),
    }
}

/// Creates `Offset` out of passed expression. If the expression is a literal
/// integer value, then this will check the value at compile time instead
/// of runtime. Otherwise it will produce potential runtime validity checks,
/// depending on compiler's optimizations.
#[proc_macro]
pub fn make_offset(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expression = match syn::parse::<syn::Expr>(tokens) {
        Ok(ok) => ok,
        Err(err) => {
            return err.into_compile_error().into();
        }
    };

    let org_expr = unpack_brackets(&expression);
    let (sign, expr) = unpack_negative_sign(org_expr);

    match expr {
        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Int(lit_int) => {
                let Ok(value) = lit_int.base10_parse::<i32>() else {
                    return quote! {
                        compile_error!("Invalid literal integer.");
                    }
                    .into();
                };

                let real_value = value * sign;

                if real_value == -1 {
                    return quote! {
                        ::osom_lib_primitives::offset::Offset::MINUS_ONE
                    }
                    .into();
                }

                if real_value == 0 {
                    return quote! {
                        ::osom_lib_primitives::offset::Offset::ZERO
                    }
                    .into();
                }

                if real_value == 1 {
                    return quote! {
                        ::osom_lib_primitives::offset::Offset::ONE
                    }
                    .into();
                }

                quote! {
                        {
                            const {
                                match ::osom_lib_primitives::offset::Offset::try_from_i32(#real_value) {
                                    Ok(val) => val,
                                    Err(_) => panic!("The literal value is out of [Offset::MIN_OFFSET..Offset::MAX_OFFSET] range."),
                                }
                            }
                        }
                    }.into()
            }
            _ => quote! {
                compile_error!("Expected literal integer.");
            }
            .into(),
        },
        _ => quote! {
            {
                ::osom_lib_primitives::offset::Offset::try_from_i32(#org_expr).unwrap()
            }
        }
        .into(),
    }
}

fn unpack_brackets(expr: &syn::Expr) -> &syn::Expr {
    match expr {
        syn::Expr::Paren(expr_paren) => unpack_brackets(&expr_paren.expr),
        expr => expr,
    }
}

fn unpack_negative_sign(expr: &syn::Expr) -> (i32, &syn::Expr) {
    match expr {
        syn::Expr::Unary(expr_unary) => {
            match expr_unary.op {
                syn::UnOp::Neg(_) => {}
                _ => {
                    panic!("Invalid unary expression, expected either positive or negative integer.");
                }
            }
            (-1, &expr_unary.expr)
        }
        expr => (1, expr),
    }
}
