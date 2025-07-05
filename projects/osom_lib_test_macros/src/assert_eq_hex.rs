/// Compares two operands by equality. If operands are not equal panics
/// and prints both operands as hex arrays.
///
/// # Arguments
///
/// Both arguments have to be arrays, slices or vectors.
#[macro_export]
macro_rules! assert_eq_hex {
    ($left:expr, $right:expr) => {{
        let left_expr = $crate::traits::AsDisplayAsHex::as_display_as_hex(&($left));
        let right_expr = $crate::traits::AsDisplayAsHex::as_display_as_hex(&($right));
        if left_expr.as_slice() != right_expr.as_slice() {
            panic!(
                r#"assertion `left == right` failed
  left: {left_expr}
 right: {right_expr}"#
            )
        }
    }};
}

/// Compares two operands by equality. If operands are equal panics
/// and prints a single operand as hex array.
///
/// # Arguments
///
/// Both arguments have to be arrays, slices or vectors.
#[macro_export]
macro_rules! assert_neq_hex {
    ($left:expr, $right:expr) => {{
        let left_expr = $crate::traits::AsDisplayAsHex::as_display_as_hex(&($left));
        let right_expr = $crate::traits::AsDisplayAsHex::as_display_as_hex(&($right));
        if left_expr.as_slice() == right_expr.as_slice() {
            panic!(
                r#"assertion `left != right` failed
  value: {left_expr}"#
            )
        }
    }};
}
