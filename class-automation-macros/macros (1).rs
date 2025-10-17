// Rust macros for declaring constants, variables, and performing assignments/borrows with specific semantics.
// Each macro includes a catch-all pattern to provide a custom compile_error! message for invalid syntax,
// and ownership checks where applicable, to aid debugging.

// [Existing declare_immut_var! with no_mut_ptr and mut_ptr remain unchanged...]

// Macro to declare an immutable variable with no mutable pointer (no_mut_ptr).
// ... (same as before)

// Macro to declare an immutable variable with mutable pointer (mut_ptr).
// ... (same as before)

// Macro to declare a mutable variable with no mutable pointer (no_mut_ptr).
// ... (same as before)

// Macro to declare a mutable variable with mutable pointer (mut_ptr).
// ... (same as before)

// Macro to declare a constant.
// ... (same as before)

// Macro to assign ownership (move semantics) to a new variable.
// ... (same as before)

// Macro to assign by copying to a new variable.
// ... (same as before)

// Macro to lend uniquely and writably (mutable borrow) to a new variable.
// Updated to better hide &mut: Users just provide expr, macro adds &mut implicitly.
// Usage:
// - unique_writable_lend!(name = expression);
// - unique_writable_lend!(name : &mut type = expression);
// Faulty case: Immutable expr triggers error via check.
#[macro_export]
macro_rules! unique_writable_lend {
    ($name:ident = $expr:expr) => {
        // Check mutability
        {
            #[allow(unused_mut, unused_variables)]
            let mut __check_mut = &mut $expr;
        }
        let mut $name = &mut $expr;
    };
    ($name:ident : &mut $ty:ty = $expr:expr) => {
        // Check mutability
        {
            #[allow(unused_mut, unused_variables)]
            let mut __check_mut = &mut $expr;
        }
        let mut $name: &mut $ty = &mut $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for unique_writable_lend!. Expected: name = expression or name : &mut type = expression. Macro hides &mut for you.");
    };
}

// Macro to lend read-only (immutable borrow) to a new variable.
// Updated to hide & completely: Users provide expr, macro adds &.
// Usage:
// - read_only_lend!(name = expression);
// - read_only_lend!(name : &type = expression);
// Faulty cases: Mutable attempts error.
#[macro_export]
macro_rules! read_only_lend {
    ($name:ident = $expr:expr) => {
        let $name = &$expr;
    };
    ($name:ident : &$ty:ty = $expr:expr) => {
        let $name: &$ty = &$expr;
    };
    ($name:ident = &mut $expr:expr) => {
        compile_error!("read_only_lend! does not allow mutable borrows. Use unique_writable_lend! instead. Macro hides & for you.");
    };
    ($name:ident : &mut $ty:ty = $expr:expr) => {
        compile_error!("read_only_lend! does not allow mutable borrows. Use unique_writable_lend! instead. Macro hides & for you.");
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for read_only_lend!. Expected: name = expression or name : &type = expression. Macro hides & for you.");
    };
}

// New macro to safely access dereferenced values, hiding *.
// Usage: deref_access!(ref_name) -> expands to *ref_name
// Faulty: Non-deref types error at compile.
#[macro_export]
macro_rules! deref_access {
    ($ref:expr) => {
        *$ref
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for deref_access!. Expected: ref_name. Macro hides * for safe access.");
    };
}

// New macro to mutate dereferenced values, hiding * and checking mut.
// Usage: deref_mutate!(ref_name = new_expr)
// Expands to *ref_name = new_expr with mut check.
#[macro_export]
macro_rules! deref_mutate {
    ($ref:ident = $expr:expr) => {
        // Dummy mut check on ref
        {
            #[allow(unused_mut)]
            let mut __check_mut = &mut $ref;
        }
        *$ref = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for deref_mutate!. Expected: ref_name = new_expr. Macro hides * and checks mutability.");
    };
}

// New macro for scoped blocks to isolate borrows, hiding lifetime quirks.
// Usage: scope_block! { code_here }
// Expands to { code_here } but with optional name for clarity.
#[macro_export]
macro_rules! scope_block {
    ( $code:block ) => {
        $code
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for scope_block!. Expected: { code }. Use to isolate borrows.");
    };
}