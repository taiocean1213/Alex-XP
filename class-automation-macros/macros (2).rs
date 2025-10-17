// Rust macros for declaring constants, variables, and performing assignments/borrows with specific semantics.
// Each macro includes a catch-all pattern to provide a custom compile_error! message for invalid syntax,
// and ownership checks where applicable, to aid debugging.

// Macro to declare an immutable variable with no mutable pointer (no_mut_ptr).
// Usage:
// - declare_immut_var!(name, no_mut_ptr); Uninitialized immutable variable, no type.
// - declare_immut_var!(name : type, no_mut_ptr); Uninitialized immutable with type.
// - declare_immut_var!(name = expr, no_mut_ptr); Immutable variable with value.
// - declare_immut_var!(name : type = expr, no_mut_ptr); Immutable variable with type and value.
// - declare_immut_var!(name = &expr, no_mut_ptr); Immutable variable with immutable borrow.
// - declare_immut_var!(name : &type = &expr, no_mut_ptr); Immutable variable with typed immutable borrow.
// Faulty cases:
// - Invalid syntax triggers compile_error! with expected format.
// - Attempting mutable borrows triggers compile_error!.
// - Type mismatches or borrow checker violations trigger standard compiler errors.
#[macro_export]
macro_rules! declare_immut_var {
    ($name:ident, no_mut_ptr) => {
        let $name;
    };
    ($name:ident : $ty:ty, no_mut_ptr) => {
        let $name: $ty;
    };
    ($name:ident = $expr:expr, no_mut_ptr) => {
        let $name = $expr;
    };
    ($name:ident : $ty:ty = $expr:expr, no_mut_ptr) => {
        let $name: $ty = $expr;
    };
    ($name:ident = &$expr:expr, no_mut_ptr) => {
        let $name = &$expr;
    };
    ($name:ident : &$ty:ty = &$expr:expr, no_mut_ptr) => {
        let $name: &$ty = &$expr;
    };
    ($name:ident = &mut $expr:expr, no_mut_ptr) => {
        compile_error!("declare_immut_var! with no_mut_ptr does not allow mutable borrows. Use `&expr` for immutable borrow or declare_immut_var!(name, mut_ptr) for mutable borrow.");
    };
    ($name:ident : &mut $ty:ty = &mut $expr:expr, no_mut_ptr) => {
        compile_error!("declare_immut_var! with no_mut_ptr does not allow mutable borrows. Use `&expr` for immutable borrow or declare_immut_var!(name, mut_ptr) for mutable borrow.");
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for declare_immut_var!. Expected one of:\n- name, no_mut_ptr\n- name : type, no_mut_ptr\n- name = expr, no_mut_ptr\n- name : type = expr, no_mut_ptr\n- name = &expr, no_mut_ptr\n- name : &type = &expr, no_mut_ptr");
    };
}

// Macro to declare an immutable variable with mutable pointer (mut_ptr).
// Usage:
// - declare_immut_var!(name = &mut expr, mut_ptr); Immutable variable with mutable borrow.
// - declare_immut_var!(name : &mut type = &mut expr, mut_ptr); Immutable variable with typed mutable borrow.
// Faulty cases:
// - Invalid syntax or non-mutable borrow triggers compile_error!.
// - Mutable borrow of immutable data triggers standard compiler error via mutability check.
// - Type mismatches or borrow checker violations trigger standard compiler errors.
#[macro_export]
macro_rules! declare_immut_var {
    ($name:ident = &mut $expr:expr, mut_ptr) => {
        // Check if expr is mutable by attempting to borrow it mutably in a dummy scope
        {
            #[allow(unused_mut, unused_variables)]
            let mut __check_mut = &mut $expr;
        }
        let $name = &mut $expr;
    };
    ($name:ident : &mut $ty:ty = &mut $expr:expr, mut_ptr) => {
        // Check if expr is mutable by attempting to borrow it mutably in a dummy scope
        {
            #[allow(unused_mut, unused_variables)]
            let mut __check_mut = &mut $expr;
        }
        let $name: &mut $ty = &mut $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for declare_immut_var!. Expected one of:\n- name = &mut expr, mut_ptr\n- name : &mut type = &mut expr, mut_ptr\nFor non-mutable pointers, use declare_immut_var!(name, no_mut_ptr)");
    };
}

// Macro to declare a mutable variable with no mutable pointer (no_mut_ptr).
// Usage:
// - declare_mut_var!(name, no_mut_ptr); Uninitialized mutable variable, no type.
// - declare_mut_var!(name : type, no_mut_ptr); Uninitialized mutable with type.
// - declare_mut_var!(name = expr, no_mut_ptr); Mutable variable with value.
// - declare_mut_var!(name : type = expr, no_mut_ptr); Mutable variable with type and value.
// - declare_mut_var!(name = &expr, no_mut_ptr); Mutable variable with immutable borrow.
// - declare_mut_var!(name : &type = &expr, no_mut_ptr); Mutable variable with typed immutable borrow.
// Faulty cases:
// - Invalid syntax triggers compile_error!.
// - Attempting mutable borrows triggers compile_error!.
// - Type mismatches or borrow checker violations trigger standard compiler errors.
#[macro_export]
macro_rules! declare_mut_var {
    ($name:ident, no_mut_ptr) => {
        let mut $name;
    };
    ($name:ident : $ty:ty, no_mut_ptr) => {
        let mut $name: $ty;
    };
    ($name:ident = $expr:expr, no_mut_ptr) => {
        let mut $name = $expr;
    };
    ($name:ident : $ty:ty = $expr:expr, no_mut_ptr) => {
        let mut $name: $ty = $expr;
    };
    ($name:ident = &$expr:expr, no_mut_ptr) => {
        let mut $name = &$expr;
    };
    ($name:ident : &$ty:ty = &$expr:expr, no_mut_ptr) => {
        let mut $name: &$ty = &$expr;
    };
    ($name:ident = &mut $expr:expr, no_mut_ptr) => {
        compile_error!("declare_mut_var! with no_mut_ptr does not allow mutable borrows. Use `&expr` for immutable borrow or declare_mut_var!(name, mut_ptr) for mutable borrow.");
    };
    ($name:ident : &mut $ty:ty = &mut $expr:expr, no_mut_ptr) => {
        compile_error!("declare_mut_var! with no_mut_ptr does not allow mutable borrows. Use `&expr` for immutable borrow or declare_mut_var!(name, mut_ptr) for mutable borrow.");
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for declare_mut_var!. Expected one of:\n- name, no_mut_ptr\n- name : type, no_mut_ptr\n- name = expr, no_mut_ptr\n- name : type = expr, no_mut_ptr\n- name = &expr, no_mut_ptr\n- name : &type = &expr, no_mut_ptr");
    };
}

// Macro to declare a mutable variable with mutable pointer (mut_ptr).
// Usage:
// - declare_mut_var!(name = &mut expr, mut_ptr); Mutable variable with mutable borrow.
// - declare_mut_var!(name : &mut type = &mut expr, mut_ptr); Mutable variable with typed mutable borrow.
// Faulty cases:
// - Invalid syntax or non-mutable borrow triggers compile_error!.
// - Mutable borrow of immutable data triggers standard compiler error via mutability check.
// - Type mismatches or borrow checker violations trigger standard compiler errors.
#[macro_export]
macro_rules! declare_mut_var {
    ($name:ident = &mut $expr:expr, mut_ptr) => {
        // Check if expr is mutable by attempting to borrow it mutably in a dummy scope
        {
            #[allow(unused_mut, unused_variables)]
            let mut __check_mut = &mut $expr;
        }
        let mut $name = &mut $expr;
    };
    ($name:ident : &mut $ty:ty = &mut $expr:expr, mut_ptr) => {
        // Check if expr is mutable by attempting to borrow it mutably in a dummy scope
        {
            #[allow(unused_mut, unused_variables)]
            let mut __check_mut = &mut $expr;
        }
        let mut $name: &mut $ty = &mut $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for declare_mut_var!. Expected one of:\n- name = &mut expr, mut_ptr\n- name : &mut type = &mut expr, mut_ptr\nFor non-mutable pointers, use declare_mut_var!(name, no_mut_ptr)");
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
// Expanded to handle optional type annotations for better type checker guidance.
// Usage:
// - ownership_assign!(name = expression);
// - ownership_assign!(name : type = expression);
// This expands to `let name = expression;`, which moves if the type is not Copy.
// Faulty case: Invalid syntax triggers compile_error!.
// If expression is moved and used later, compiler errors with "use of moved value".
#[macro_export]
macro_rules! ownership_assign {
    ($name:ident = $expr:expr) => {
        let $name = $expr;
    };
    ($name:ident : $ty:ty = $expr:expr) => {
        let $name: $ty = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for ownership_assign!. Expected: name (=|: type =) expression");
    };
}

// Macro to assign by copying to a new variable.
// Expanded to handle optional type annotations for better type checker guidance.
// Usage:
// - copy_assign!(name = expression);
// - copy_assign!(name : type = expression);
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
    ($name:ident : $ty:ty = $expr:expr) => {
        let $name: $ty = {
            #[allow(unused)]
            fn __grok_copy_helper<T: Copy>(t: &T) -> T { *t }
            __grok_copy_helper(&$expr)
        };
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for copy_assign!. Expected: name (=|: type =) expression");
    };
}

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

// New macro for reassigning a mutable variable with ownership transfer (move).
// Usage:
// - reassign_ownership!(name = expr);
// This expands to `name = expr;`, assuming name is mutable. Moves if not Copy.
// Faulty case: If name is immutable, compiler errors with "cannot assign to immutable".
// Invalid syntax triggers compile_error!.
#[macro_export]
macro_rules! reassign_ownership {
    ($name:ident = $expr:expr) => {
        $name = $expr;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for reassign_ownership!. Expected: name = expr");
    };
}

// New macro for reassigning a mutable variable by copying.
// Usage:
// - reassign_copy!(name = expr);
// This requires expr to implement Copy and copies without moving.
// Faulty case: If type doesn't implement Copy, compiler errors.
// If name is immutable, compiler errors.
#[macro_export]
macro_rules! reassign_copy {
    ($name:ident = $expr:expr) => {
        $name = {
            #[allow(unused)]
            fn __grok_copy_helper<T: Copy>(t: &T) -> T { *t }
            __grok_copy_helper(&$expr)
        };
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for reassign_copy!. Expected: name = expr");
    };
}

// New macro for explicitly deleting (dropping) a variable.
// Usage: delete_var!(name);
// This expands to `drop(name);`, which drops the variable early.
// Faulty case: Invalid syntax triggers compile_error!.
// If name is already dropped or moved, compiler errors.
#[macro_export]
macro_rules! delete_var {
    ($name:ident) => {
        ::std::mem::drop($name);
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for delete_var!. Expected: name");
    };
}

// New macro for performing arithmetic operations, abstracting expressions.
// Usage: perform_op!(name = expr1 + expr2); // Or other ops like -, *, /
// Expands to name = expr1 + expr2; assuming name is mutable.
// Faulty: Non-mut name or invalid op triggers compiler errors.
#[macro_export]
macro_rules! perform_op {
    ($name:ident = $expr1:expr + $expr2:expr) => {
        $name = $expr1 + $expr2;
    };
    ($name:ident = $expr1:expr - $expr2:expr) => {
        $name = $expr1 - $expr2;
    };
    ($name:ident = $expr1:expr * $expr2:expr) => {
        $name = $expr1 * $expr2;
    };
    ($name:ident = $expr1:expr / $expr2:expr) => {
        $name = $expr1 / $expr2;
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for perform_op!. Expected: name = expr1 op expr2 where op is +, -, *, /");
    };
}

// New macro for printing values, abstracting println!.
// Usage: print_value!(expr);
// Expands to println!("{}", expr);
#[macro_export]
macro_rules! print_value {
    ($expr:expr) => {
        println!("{}", $expr);
    };
    ($($other:tt)*) => {
        compile_error!("Invalid syntax for print_value!. Expected: expr");
    };
}