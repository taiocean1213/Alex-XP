//! Named ownership, copy, lend, deref, scope, drop, and thin ops.

/// Copy `expr` only if its type implements `Copy`. Used by `copy_assign!` and
/// `reassign_copy!` so a move cannot sneak through.
#[doc(hidden)]
#[inline]
pub fn __alex_copy<T: Copy>(value: &T) -> T {
    *value
}

/// Immutable binding. `no_mut_ptr` allows owned values and shared lends; `mut_ptr`
/// requires a unique writable lend (`&mut`).
#[macro_export]
macro_rules! declare_immut_var {
    ($name:ident = &mut $expr:expr, no_mut_ptr) => {
        compile_error!(
            "declare_immut_var! with no_mut_ptr does not allow unique writable lends. Use read_only_lend! / `&expr`, or declare_immut_var!(name = &mut expr, mut_ptr)."
        );
    };
    ($name:ident : &mut $ty:ty = &mut $expr:expr, no_mut_ptr) => {
        compile_error!(
            "declare_immut_var! with no_mut_ptr does not allow unique writable lends. Use read_only_lend! / `&expr`, or declare_immut_var!(name = &mut expr, mut_ptr)."
        );
    };
    ($name:ident = &mut $expr:expr, mut_ptr) => {
        let $name = &mut $expr;
    };
    ($name:ident : &mut $ty:ty = &mut $expr:expr, mut_ptr) => {
        let $name: &mut $ty = &mut $expr;
    };
    ($name:ident : &mut $ty:ty = $expr:expr, mut_ptr) => {
        let $name: &mut $ty = $expr;
    };
    ($name:ident = $expr:expr, mut_ptr) => {
        let $name = $expr;
    };
    ($name:ident, no_mut_ptr) => {
        let $name;
    };
    ($name:ident : $ty:ty, no_mut_ptr) => {
        let $name: $ty;
    };
    ($name:ident = &$expr:expr, no_mut_ptr) => {
        let $name = &$expr;
    };
    ($name:ident : &$ty:ty = &$expr:expr, no_mut_ptr) => {
        let $name: &$ty = &$expr;
    };
    ($name:ident = $expr:expr, no_mut_ptr) => {
        let $name = $expr;
    };
    ($name:ident : $ty:ty = $expr:expr, no_mut_ptr) => {
        let $name: $ty = $expr;
    };
    ($name:ident, mut_ptr) => {
        compile_error!(
            "declare_immut_var! with mut_ptr requires a unique writable lend. Expected: name = &mut expr, mut_ptr"
        );
    };
    ($($other:tt)*) => {
        compile_error!(
            "Invalid syntax for declare_immut_var!. Expected one of:\n- name, no_mut_ptr\n- name : type, no_mut_ptr\n- name = expr, no_mut_ptr\n- name : type = expr, no_mut_ptr\n- name = &expr, no_mut_ptr\n- name : &type = &expr, no_mut_ptr\n- name = &mut expr, mut_ptr\n- name : &mut type = &mut expr, mut_ptr"
        );
    };
}

/// Mutable binding. Same pointer flags as [`declare_immut_var!`].
#[macro_export]
macro_rules! declare_mut_var {
    ($name:ident = &mut $expr:expr, no_mut_ptr) => {
        compile_error!(
            "declare_mut_var! with no_mut_ptr does not allow unique writable lends. Use unique_writable_lend! or declare_mut_var!(name = &mut expr, mut_ptr)."
        );
    };
    ($name:ident : &mut $ty:ty = &mut $expr:expr, no_mut_ptr) => {
        compile_error!(
            "declare_mut_var! with no_mut_ptr does not allow unique writable lends. Use unique_writable_lend! or declare_mut_var!(name = &mut expr, mut_ptr)."
        );
    };
    ($name:ident = &mut $expr:expr, mut_ptr) => {
        let mut $name = &mut $expr;
    };
    ($name:ident : &mut $ty:ty = &mut $expr:expr, mut_ptr) => {
        let mut $name: &mut $ty = &mut $expr;
    };
    ($name:ident : &mut $ty:ty = $expr:expr, mut_ptr) => {
        let mut $name: &mut $ty = $expr;
    };
    ($name:ident = $expr:expr, mut_ptr) => {
        let mut $name = $expr;
    };
    ($name:ident, no_mut_ptr) => {
        let mut $name;
    };
    ($name:ident : $ty:ty, no_mut_ptr) => {
        let mut $name: $ty;
    };
    ($name:ident = &$expr:expr, no_mut_ptr) => {
        let mut $name = &$expr;
    };
    ($name:ident : &$ty:ty = &$expr:expr, no_mut_ptr) => {
        let mut $name: &$ty = &$expr;
    };
    ($name:ident = $expr:expr, no_mut_ptr) => {
        let mut $name = $expr;
    };
    ($name:ident : $ty:ty = $expr:expr, no_mut_ptr) => {
        let mut $name: $ty = $expr;
    };
    ($name:ident, mut_ptr) => {
        compile_error!(
            "declare_mut_var! with mut_ptr requires a unique writable lend. Expected: name = &mut expr, mut_ptr"
        );
    };
    ($($other:tt)*) => {
        compile_error!(
            "Invalid syntax for declare_mut_var!. Expected one of:\n- name, no_mut_ptr\n- name : type, no_mut_ptr\n- name = expr, no_mut_ptr\n- name : type = expr, no_mut_ptr\n- name = &expr, no_mut_ptr\n- name : &type = &expr, no_mut_ptr\n- name = &mut expr, mut_ptr\n- name : &mut type = &mut expr, mut_ptr"
        );
    };
}

/// Immutable binding with an explicit type. Prefer [`declare_immut_var!`] when you
/// need pointer flags.
#[macro_export]
macro_rules! declare_var {
    ($name:ident : $ty:ty = $expr:expr) => {
        let $name: $ty = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for declare_var!. Expected: name : type = expression");
    };
}

#[macro_export]
macro_rules! declare_const {
    ($name:ident : $ty:ty = $expr:expr) => {
        const $name: $ty = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for declare_const!. Expected: name : type = expression");
    };
}

#[macro_export]
macro_rules! ownership_assign {
    ($name:ident : $ty:ty = $expr:expr) => {
        let $name: $ty = $expr;
    };
    ($name:ident = $expr:expr) => {
        let $name = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for ownership_assign!. Expected: name = expression or name : type = expression");
    };
}

#[macro_export]
macro_rules! copy_assign {
    ($name:ident : $ty:ty = $expr:expr) => {
        let $name: $ty = $crate::__alex_copy(&$expr);
    };
    ($name:ident = $expr:expr) => {
        let $name = $crate::__alex_copy(&$expr);
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for copy_assign!. Expected: name = expression or name : type = expression");
    };
}

/// Unique writable lend. You write the place; the macro inserts `&mut`.
#[macro_export]
macro_rules! unique_writable_lend {
    ($name:ident : &mut $ty:ty = $expr:expr) => {
        let mut $name: &mut $ty = &mut $expr;
    };
    ($name:ident = $expr:expr) => {
        let mut $name = &mut $expr;
    };
    ($($other:tt)*) => {
        compile_error!(
            "Invalid syntax for unique_writable_lend!. Expected: name = expression or name : &mut type = expression. The macro inserts &mut."
        );
    };
}

/// Shared read-only lend. You write the place; the macro inserts `&`.
#[macro_export]
macro_rules! read_only_lend {
    ($name:ident = &mut $expr:expr) => {
        compile_error!(
            "read_only_lend! does not allow unique writable lends. Use unique_writable_lend! instead. The macro inserts &."
        );
    };
    ($name:ident : &mut $ty:ty = $expr:expr) => {
        compile_error!(
            "read_only_lend! does not allow unique writable lends. Use unique_writable_lend! instead. The macro inserts &."
        );
    };
    ($name:ident : &$ty:ty = $expr:expr) => {
        let $name: &$ty = &$expr;
    };
    ($name:ident = $expr:expr) => {
        let $name = &$expr;
    };
    ($($other:tt)*) => {
        compile_error!(
            "Invalid syntax for read_only_lend!. Expected: name = expression or name : &type = expression. The macro inserts &."
        );
    };
}

#[macro_export]
macro_rules! deref_access {
    ($reference:expr) => {
        *$reference
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for deref_access!. Expected: lent_name");
    };
}

#[macro_export]
macro_rules! deref_mutate {
    ($reference:ident = $expr:expr) => {
        *$reference = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for deref_mutate!. Expected: lent_name = new_expr");
    };
}

/// Isolate lends so they end at the closing brace.
///
/// Call as `scope_block! { ... }`. The braces are the macro delimiters; the
/// contents run in an inner block so lends do not escape.
#[macro_export]
macro_rules! scope_block {
    ($($code:tt)*) => {{
        $($code)*
    }};
}

#[macro_export]
macro_rules! reassign_ownership {
    ($name:ident = $expr:expr) => {
        $name = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for reassign_ownership!. Expected: name = expr");
    };
}

#[macro_export]
macro_rules! reassign_copy {
    ($name:ident = $expr:expr) => {
        $name = $crate::__alex_copy(&$expr);
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for reassign_copy!. Expected: name = expr");
    };
}

#[macro_export]
macro_rules! delete_var {
    ($name:ident) => {
        ::core::mem::drop($name);
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for delete_var!. Expected: name");
    };
}

#[macro_export]
macro_rules! perform_op {
    ($name:ident = $rhs:expr) => {
        $name = $rhs;
    };
    ($($other:tt)*) => {
        compile_error!(
            "Invalid syntax for perform_op!. Expected: name = expr1 + expr2 (or -, *, /)"
        );
    };
}

#[macro_export]
macro_rules! print_value {
    ($expr:expr) => {
        ::std::println!("{}", $expr);
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for print_value!. Expected: expr");
    };
}
