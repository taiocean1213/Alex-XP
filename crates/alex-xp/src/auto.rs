//! Decisions Python does not ask the programmer to make.
//!
//! * **Types** on locals and closures are inferred.
//! * **Ownership** duplicates (`Clone`) so the source stays usable. Hand-off is
//!   explicit (`auto_give!`). Rust copies `Copy` types inside `Clone`.
//! * **Lifetimes** are not written. A lend lives only in its block. `auto_own!`
//!   turns a lend into owned data (`&str` → `String`) so nothing has to carry `'a`.
//! * **Heap links** (`auto_box!`) build trees with `Box` instead of a lifetime.

/// Duplicate `value`. `Copy` types copy; other `Clone` types clone.
#[doc(hidden)]
pub fn auto_dup<T: Clone>(value: &T) -> T {
    value.clone()
}

/// End a lifetime by storing owned data. Pass a lend (`&str`, `&[T]`, `&String`).
#[doc(hidden)]
pub fn auto_own<T: ToOwned + ?Sized>(value: &T) -> T::Owned {
    value.to_owned()
}

/// Inferred immutable binding. The source stays usable.
///
/// ```ignore
/// auto_let!(label = String::from("Ada"));
/// auto_let!(again = label);
/// ```
#[macro_export]
macro_rules! auto_let {
    ($name:ident = $expr:expr) => {
        let $name = $crate::auto_dup(&$expr);
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for auto_let!. Expected: name = expr");
    };
}

/// Inferred mutable binding. The source stays usable.
#[macro_export]
macro_rules! auto_let_mut {
    ($name:ident = $expr:expr) => {
        let mut $name = $crate::auto_dup(&$expr);
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for auto_let_mut!. Expected: name = expr");
    };
}

/// Inferred binding that takes the value (move). The source cannot be used after.
#[macro_export]
macro_rules! auto_give {
    ($name:ident = $expr:expr) => {
        let $name = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for auto_give!. Expected: name = expr");
    };
}

/// Replace a mutable binding with a duplicate of `expr`.
#[macro_export]
macro_rules! auto_set {
    ($name:ident = $expr:expr) => {
        $name = $crate::auto_dup(&$expr);
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for auto_set!. Expected: name = expr");
    };
}

/// Shared lend. The type and lifetime are inferred; do not write `&` or `'a`.
/// The lend lasts as long as this binding. Use [`auto_own!`] to keep the data.
#[macro_export]
macro_rules! auto_share {
    ($name:ident = $expr:expr) => {
        let $name = &$expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for auto_share!. Expected: name = expr");
    };
}

/// Unique writable lend that ends at the end of the block, so no lifetime escapes.
///
/// ```ignore
/// auto_edit!(slot = count {
///     deref_mutate!(slot = deref_access!(slot) + 1);
/// });
/// ```
#[macro_export]
macro_rules! auto_edit {
    ($name:ident = $place:ident $body:block) => {{
        let $name = &mut $place;
        $body
    }};
    ($($other:tt)*) => {
        compile_error!(
            "Invalid syntax for auto_edit!. Expected: name = place { code }"
        );
    };
}

/// Turn a lend into owned data so the result has no lifetime.
/// `auto_own!(kept = view)` when `view` is `&str` yields a `String`.
#[macro_export]
macro_rules! auto_own {
    ($name:ident = $expr:expr) => {
        let $name = $crate::auto_own($expr);
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for auto_own!. Expected: name = lend");
    };
}

/// Own `expr` on the heap. Use this for recursive links (`Option<Box<Node>>`)
/// instead of a reference and a lifetime.
#[macro_export]
macro_rules! auto_box {
    ($name:ident = $expr:expr) => {
        let $name = ::std::boxed::Box::new($expr);
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for auto_box!. Expected: name = expr");
    };
}

/// Local function whose argument and return types are inferred.
///
/// ```ignore
/// auto_fn!(double = |n| n * 2);
/// ```
#[macro_export]
macro_rules! auto_fn {
    ($name:ident = $expr:expr) => {
        let $name = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for auto_fn!. Expected: name = |args| expr");
    };
}
