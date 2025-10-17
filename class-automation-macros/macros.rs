// Rust macros for declaring constants, variables, and performing assignments/borrows with specific semantics.
// Each macro includes a catch-all pattern to provide a custom compile_error! message for invalid syntax,
// making debugging easier. For type-related errors (e.g., copying a non-Copy type), the compiler provides
// informative messages.

// Macro to declare a variable with an explicit type and initializer.
// Usage: declare_var!(name : type = expression);
// This expands to `let name: type = expression;`.
// Faulty case: Invalid syntax triggers compile_error!.
// If expression type mismatches, the compiler provides a type mismatch error.
#[macro_export]
macro_rules! declare_var {
    ($name:ident : $ty:ty = $expr:expr) => {
        let $name: $ty = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for declare_var!. Expected: name : type = expression");
    };
}

// Macro to declare a constant.
// Usage: declare_const!(NAME : TYPE = EXPRESSION);
// Faulty case: Invalid syntax triggers compile_error!.
// If EXPRESSION is not evaluable at compile time, the compiler will error with a standard message.
#[macro_export]
macro_rules! declare_const {
    ($name:ident : $ty:ty = $expr:expr) => {
        const $name: $ty = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for declare_const!. Expected: name : type = expression");
    };
}

// Macro to assign ownership (move semantics) to a new variable.
// Usage: ownership_assign!(name = expression);
// This expands to `let name = expression;`, which moves if the type is not Copy.
// Faulty case: Invalid syntax triggers compile_error!.
// If expression is moved and used later, compiler errors with "use of moved value".
#[macro_export]
macro_rules! ownership_assign {
    ($name:ident = $expr:expr) => {
        let $name = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for ownership_assign!. Expected: name = expression");
    };
}

// Macro to assign by copying to a new variable.
// Usage: copy_assign!(name = expression);
// This requires the type of expression to implement Copy and creates a copy without consuming the original.
// Faulty case: Invalid syntax triggers compile_error!.
// If the type does not implement Copy, compiler errors with "the trait `Copy` is not implemented for `Type`".
#[macro_export]
macro_rules! copy_assign {
    ($name:ident = $expr:expr) => {
        let $name = {
            #[allow(unused)]
            fn __grok_copy_helper<T: Copy>(t: &T) -> T { *t }
            __grok_copy_helper(&$expr)
        };
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for copy_assign!. Expected: name = expression");
    };
}

// Macro to lend uniquely and writably (mutable borrow) to a new variable.
// Usage: unique_writable_lend!(name = expression);
// This expands to `let mut name = &mut expression;`.
// Faulty case: Invalid syntax triggers compile_error!.
// If expression is not mutable or already borrowed, compiler provides standard borrow checker errors (e.g., "cannot borrow as mutable").
#[macro_export]
macro_rules! unique_writable_lend {
    ($name:ident = $expr:expr) => {
        let mut $name = &mut $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for unique_writable_lend!. Expected: name = expression");
    };
}

// Macro to lend read-only (immutable borrow) to a new variable.
// Usage: read_only_lend!(name = expression);
// This expands to `let name = &expression;`.
// Faulty case: Invalid syntax triggers compile_error!.
// If already mutably borrowed, compiler provides standard borrow checker errors (e.g., "cannot borrow as immutable because it is also borrowed as mutable").
#[macro_export]
macro_rules! read_only_lend {
    ($name:ident = $expr:expr) => {
        let $name = &$expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for read_only_lend!. Expected: name = expression");
    };
}