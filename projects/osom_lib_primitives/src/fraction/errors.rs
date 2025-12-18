use osom_lib_reprc::macros::reprc;

/// Represents possible erros when interacting with [`Fraction32`][`super::Fraction32`]
/// and [`Fraction64`][`super::Fraction64`].
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[reprc]
#[repr(u8)]
#[must_use]
pub enum FractionError {
    NotAFraction = 0,
}
