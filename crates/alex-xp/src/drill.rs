//! Walk a boxed “previous node” chain to the leaf. Not a general borrow engine.
//!
//! Pattern: `Enum::Prev(Box<Node>)` / `Enum::Value(...)` with a field named
//! `data` (or another `$field`) holding the next enum.

/// Unique writable drill. Returns a `&mut` to the leaf enum.
///
/// ```ignore
/// let leaf = drill_mut!(&mut node.data, NodeData::Prev, data);
/// ```
#[macro_export]
macro_rules! drill_mut {
    ($start:expr, $enum_name:ident :: $variant:ident, $field:ident) => {{
        let mut current = $start;
        while let $enum_name::$variant(prev) = current {
            current = &mut prev.$field;
        }
        current
    }};
    ($($other:tt)*) => {
        compile_error!(
            "Invalid syntax for drill_mut!. Expected: start_expr, Enum::PrevVariant, field_name"
        );
    };
}

/// Shared read-only drill. Returns a `&` to the leaf enum.
#[macro_export]
macro_rules! drill_read {
    ($start:expr, $enum_name:ident :: $variant:ident, $field:ident) => {{
        let mut current = $start;
        while let $enum_name::$variant(prev) = current {
            current = &prev.$field;
        }
        current
    }};
    ($($other:tt)*) => {
        compile_error!(
            "Invalid syntax for drill_read!. Expected: start_expr, Enum::PrevVariant, field_name"
        );
    };
}
