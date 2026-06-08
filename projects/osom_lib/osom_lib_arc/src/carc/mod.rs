//! This module holds the [`CArc`] and [`CWeak`] types and their implementations.
mod internal;
mod layout;

mod carc;
pub use carc::*;

mod weak;
pub use weak::*;
