//! Short names for the same decisions. This is the surface to type.
//!
//! `val!` / `var!` infer the type and duplicate. `add!` updates in place.
//! `s!` builds a `String`. `obj!` is a class written once.

#[macro_export]
macro_rules! val {
    ($name:ident = $expr:expr) => {
        $crate::auto_let!($name = $expr);
    };
    ($($other:tt)*) => {
        compile_error!("val!(name = expr)");
    };
}

#[macro_export]
macro_rules! var {
    ($name:ident = $expr:expr) => {
        $crate::auto_let_mut!($name = $expr);
    };
    ($($other:tt)*) => {
        compile_error!("var!(name = expr)");
    };
}

#[macro_export]
macro_rules! set {
    ($name:ident = $expr:expr) => {
        $crate::auto_set!($name = $expr);
    };
    ($($other:tt)*) => {
        compile_error!("set!(name = expr)");
    };
}

#[macro_export]
macro_rules! give {
    ($name:ident = $expr:expr) => {
        $crate::auto_give!($name = $expr);
    };
    ($($other:tt)*) => {
        compile_error!("give!(name = expr)");
    };
}

/// `add!(count, 1)` is `count = count + 1`. The name is written once.
#[macro_export]
macro_rules! add {
    ($name:ident, $expr:expr) => {
        $name = $name + $expr;
    };
    ($($other:tt)*) => {
        compile_error!("add!(name, amount)");
    };
}

/// `s!("Ada")` or `s!(other)` — a `String`, inferred at the call.
#[macro_export]
macro_rules! s {
    ($expr:expr) => {
        ::std::string::ToString::to_string(&$expr)
    };
}

#[macro_export]
macro_rules! show {
    ($expr:expr) => {
        $crate::print_value!($expr);
    };
    ($($other:tt)*) => {
        compile_error!("show!(expr)");
    };
}

#[macro_export]
macro_rules! def {
    ($name:ident = $expr:expr) => {
        $crate::auto_fn!($name = $expr);
    };
    ($($other:tt)*) => {
        compile_error!("def!(name = |arg| expr)");
    };
}

/// Heap link as an expression, so it nests: `Some(heap!(node))`.
#[macro_export]
macro_rules! heap {
    ($expr:expr) => {
        ::std::boxed::Box::new($expr)
    };
}
