//! Named ownership, copy, lend, deref, scope, drop, and ops.

use alex_xp::*;

fn main() {
    declare_const!(ANSWER : i32 = 42);
    declare_mut_var!(count : i32 = 1, no_mut_ptr);
    declare_immut_var!(label : &str = "n", no_mut_ptr);

    copy_assign!(snapshot = count);
    perform_op!(count = count + snapshot);
    print_value!(count);
    print_value!(ANSWER);
    print_value!(label);

    scope_block! {
        unique_writable_lend!(slot = count);
        deref_mutate!(slot = 10);
        print_value!(deref_access!(slot));
    }

    read_only_lend!(view = count);
    print_value!(deref_access!(view));

    declare_mut_var!(owned : String = String::from("hello"), no_mut_ptr);
    print_value!(owned);
    reassign_ownership!(owned = String::from("world"));
    print_value!(owned);
    delete_var!(owned);

    declare_mut_var!(x : i32 = 3, no_mut_ptr);
    unique_writable_lend!(p = x);
    declare_immut_var!(q = p, mut_ptr);
    deref_mutate!(q = 7);
    print_value!(x);
}

fn _uninitialized_then_assign() {
    declare_mut_var!(later : i32, no_mut_ptr);
    reassign_ownership!(later = 1);
    reassign_copy!(later = 2);
    let _ = later;
}
