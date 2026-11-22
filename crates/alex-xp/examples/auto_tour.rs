//! New-project path: inferred locals, duplicate-by-default, no lifetime names.
//!
//! Depend on upstream:
//! `alex-xp = { git = "https://github.com/taiocean1213/Alex-XP" }`

use alex_xp::*;

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    auto_let!(label = String::from("Ada"));
    auto_let!(again = label);
    print_value!(label);
    print_value!(again);

    auto_let_mut!(count = 1);
    auto_edit!(slot = count {
        deref_mutate!(slot = deref_access!(slot) + 1);
    });
    print_value!(count);

    auto_share!(view = label);
    auto_own!(kept = view);
    print_value!(kept);

    auto_fn!(double = |n| n * 2);
    print_value!(double(count));

    auto_box!(boxed = Node { value: 1, next: None });
    auto_give!(root = Node {
        value: 0,
        next: Some(boxed),
    });
    print_value!(root.next.as_ref().unwrap().value);

    auto_let!(farewell = String::from("bye"));
    auto_give!(handed = farewell);
    print_value!(handed);
}
