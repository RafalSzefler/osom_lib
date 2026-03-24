//! proc-macros that makes working with `#[cfg(...)]` easier.
//!
//! At the moment it provides [`cfg_match`] only.
#![deny(warnings)]
#![allow(unused_features)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::redundant_field_names)]
#![allow(clippy::inline_always)]

use syn::parse_macro_input;

mod cfg_matcher;
mod generator;

/// Matches against various cfg settings.
///
/// # Examples
///
/// ```rust
/// use osom_lib_cfg_ext::cfg_match;
///
/// cfg_match!(
///     (target_os = "windows") => {
///         fn os_name() -> &'static str {
///             "W"
///         }
///     },
///     (target_os = "linux") => {
///         fn os_name() -> &'static str {
///             "L"
///         }
///     },
///     _ => {
///         fn os_name() -> &'static str {
///             "U"
///         }
///     }
/// );
/// ```
///
/// The call above will generate only one `os_name` function, depending on the matched
/// condition. The condition is translated to one of `#[cfg(...)]` attributes.
///
/// Conditions are evaluated one after another. In particular the order doesn't matter
/// if they are independent. However it does matter in general. For example say we have
/// this:
///
/// ```rust
/// use osom_lib_cfg_ext::cfg_match;
///
/// cfg_match!(
///     (A) => {
///         const A: i32 = 1;
///     },
///     (B) => {
///         const B: i32 = 2;
///     }
/// );
/// ```
///
/// Then if `B` implies `A` (as a condition) then `B` arm will never be matched here.
/// For it to be matched you need to reverse the order:
///
/// ```rust
/// use osom_lib_cfg_ext::cfg_match;
///
/// cfg_match!(
///     (B) => {
///         const B: i32 = 2;
///     },
///     (A) => {
///         const A: i32 = 1;
///     }
/// );
/// ```
#[proc_macro]
pub fn cfg_match(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(tokens as cfg_matcher::CfgMatcher);
    generator::generate_from_cfg_match(&item).into()
}

/// Helper when generating code from [`cfg_match`].
#[doc(hidden)]
#[proc_macro]
pub fn identity(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tokens
}
