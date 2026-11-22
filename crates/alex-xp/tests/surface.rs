//! Forms the guide and the examples do not exercise.
//! Each test calls a public macro and checks the value rustc accepted.

use alex_xp::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn short_names_duplicate_strings_and_format() {
    var!(left = s!("a"));
    var!(right = s!("b"));
    set!(left = right);
    add!(right, "!");
    assert_eq!(left, "b");
    assert_eq!(right, "b!");
    assert_eq!(s!(3), "3");
    assert_eq!(s!(true), "true");

    def!(sum = |a, b| a + b);
    assert_eq!(sum(2, 3), 5);

    show!(s!("ok"));
    print_value!(1);
}

#[test]
fn give_takes_a_value_that_is_not_clone() {
    struct Token(i32);
    give!(token = Token(1));
    assert_eq!(token.0, 1);
}

#[test]
fn auto_box_own_and_share() {
    auto_box!(boxed = 5);
    assert_eq!(*boxed, 5);

    let raw: &[i32] = &[1, 2];
    auto_own!(from_slice = raw);
    assert_eq!(from_slice, vec![1, 2]);

    auto_let!(label = s!("Ada"));
    auto_share!(view = label);
    auto_own!(kept = view);
    assert_eq!(kept, "Ada");
    assert_eq!(label, "Ada");
}

#[test]
fn declare_forms_bind_owned_and_shared_values() {
    declare_immut_var!(later : i32, no_mut_ptr);
    later = 1;
    assert_eq!(later, 1);

    declare_immut_var!(typed : i32 = 2, no_mut_ptr);
    assert_eq!(typed, 2);
    declare_immut_var!(plain = 3, no_mut_ptr);
    assert_eq!(plain, 3);
    declare_immut_var!(shared = &plain, no_mut_ptr);
    assert_eq!(*shared, 3);
    declare_immut_var!(shared_typed : &i32 = &plain, no_mut_ptr);
    assert_eq!(*shared_typed, 3);

    declare_mut_var!(slot : i32, no_mut_ptr);
    slot = 4;
    assert_eq!(slot, 4);
    declare_mut_var!(mut_typed : i32 = 5, no_mut_ptr);
    assert_eq!(mut_typed, 5);
    mut_typed = 6;
    assert_eq!(mut_typed, 6);
    declare_mut_var!(mut_plain = 7, no_mut_ptr);
    assert_eq!(mut_plain, 7);
    reassign_ownership!(mut_plain = 8);
    assert_eq!(mut_plain, 8);
    declare_mut_var!(mut_shared = &mut_typed, no_mut_ptr);
    assert_eq!(*mut_shared, 6);
    declare_mut_var!(mut_shared_typed : &i32 = &mut_typed, no_mut_ptr);
    assert_eq!(*mut_shared_typed, 6);

    declare_var!(explicit : i32 = 9);
    assert_eq!(explicit, 9);
    declare_const!(LIMIT : i32 = 10);
    assert_eq!(LIMIT, 10);

    ownership_assign!(named : String = String::from("a"));
    ownership_assign!(moved = named);
    assert_eq!(moved, "a");

    copy_assign!(copied : i32 = explicit);
    assert_eq!(copied, 9);
    assert_eq!(explicit, 9);

    let source = String::from("moved");
    declare_mut_var!(taken = source, mut_ptr);
    assert_eq!(taken, "moved");
    let source = String::from("held");
    declare_immut_var!(held = source, mut_ptr);
    assert_eq!(held, "held");
}

#[test]
fn mut_ptr_forms_write_through_the_lend() {
    declare_mut_var!(n : i32 = 1, no_mut_ptr);
    {
        declare_immut_var!(p = &mut n, mut_ptr);
        deref_mutate!(p = 2);
    }
    {
        declare_immut_var!(p : &mut i32 = &mut n, mut_ptr);
        deref_mutate!(p = 3);
    }
    {
        let existing = &mut n;
        declare_immut_var!(p : &mut i32 = existing, mut_ptr);
        deref_mutate!(p = 4);
    }
    {
        declare_mut_var!(p = &mut n, mut_ptr);
        deref_mutate!(p = 5);
    }
    {
        declare_mut_var!(p : &mut i32 = &mut n, mut_ptr);
        deref_mutate!(p = 6);
    }
    {
        let existing = &mut n;
        declare_mut_var!(p : &mut i32 = existing, mut_ptr);
        deref_mutate!(p = 7);
    }
    {
        let existing = &mut n;
        declare_mut_var!(p = existing, mut_ptr);
        deref_mutate!(p = 8);
    }
    assert_eq!(n, 8);

    scope_block! {
        unique_writable_lend!(typed : &mut i32 = n);
        deref_mutate!(typed = 9);
    }
    assert_eq!(n, 9);
    read_only_lend!(typed_read : &i32 = n);
    assert_eq!(deref_access!(typed_read), 9);
}

#[test]
fn scope_block_ends_the_lend_and_ops_update_in_place() {
    declare_mut_var!(n : i32 = 1, no_mut_ptr);
    scope_block! {
        unique_writable_lend!(w = n);
        deref_mutate!(w = 2);
    }
    scope_block! {
        unique_writable_lend!(w = n);
        deref_mutate!(w = deref_access!(w) + 3);
    }
    assert_eq!(n, 5);
    perform_op!(n = n * 2);
    assert_eq!(n, 10);
    perform_op!(n = n - 4);
    assert_eq!(n, 6);
}

#[test]
fn delete_var_drops_before_the_scope_ends() {
    static DROPS: AtomicUsize = AtomicUsize::new(0);
    struct Bomb;
    impl Drop for Bomb {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Ordering::SeqCst);
        }
    }

    declare_mut_var!(bomb : Bomb = Bomb, no_mut_ptr);
    assert_eq!(DROPS.load(Ordering::SeqCst), 0);
    delete_var!(bomb);
    assert_eq!(DROPS.load(Ordering::SeqCst), 1);
}

#[test]
fn obj_is_clone_and_can_omit_a_return_type() {
    obj!(Counter {
        n: i32,
        fn new(n: i32) -> Self {
            Self { n }
        }
        fn bump(&mut self) {
            self.n += 1
        }
        fn get(&self) -> i32 {
            self.n
        }
    });

    var!(counter = Counter::new(1));
    val!(snapshot = counter);
    counter.bump();
    assert_eq!(counter.get(), 2);
    assert_eq!(snapshot.get(), 1);

    obj!(Unit {
        fn new() -> Self {
            Self {}
        }
        fn tag(&self) -> i32 {
            7
        }
    });
    val!(unit = Unit::new());
    val!(again = unit);
    assert_eq!(unit.tag(), 7);
    assert_eq!(again.tag(), 7);
}

#[test]
fn interface_and_iface_require_the_parent() {
    interface!(Marker);
    interface!(Child, Marker {
        fn id(&self) -> i32;
    });

    class! {
        Thing : [
            Marker {},
            Child {
                fn id(&self) -> i32 {
                    4
                }
            }
        ] {} {
            fn new() -> Self {
                Self {}
            }
        }
    }

    let thing = Thing::new();
    fn assert_marker<T: Marker>(_: &T) {}
    fn assert_child<T: Child>(value: &T) -> i32 {
        value.id()
    }
    assert_marker(&thing);
    assert_eq!(assert_child(&thing), 4);

    iface!(Named {
        fn name(&self) -> String;
    });
    iface!(Student: Named {
        fn school(&self) -> String;
    });

    class! {
        Pupil : [
            Named {
                fn name(&self) -> String {
                    s!("Ada")
                }
            },
            Student {
                fn school(&self) -> String {
                    s!("MIT")
                }
            }
        ] {} {
            fn new() -> Self {
                Self {}
            }
        }
    }

    let pupil = Pupil::new();
    assert_eq!(pupil.name(), "Ada");
    assert_eq!(pupil.school(), "MIT");
    let named: &dyn Named = &pupil;
    assert_eq!(named.name(), "Ada");
}

#[test]
fn drill_walks_more_than_one_link_and_a_bare_leaf() {
    enum NodeData {
        Value(i32),
        Prev(Box<Node>),
    }
    struct Node {
        data: NodeData,
    }

    let mut node = Node {
        data: NodeData::Prev(Box::new(Node {
            data: NodeData::Prev(Box::new(Node {
                data: NodeData::Value(1),
            })),
        })),
    };
    {
        let leaf = drill_mut!(&mut node.data, NodeData::Prev, data);
        if let NodeData::Value(v) = leaf {
            *leaf = NodeData::Value(*v + 10);
        } else {
            panic!("expected a value leaf");
        }
    }
    match drill_read!(&node.data, NodeData::Prev, data) {
        NodeData::Value(v) => assert_eq!(*v, 11),
        NodeData::Prev(_) => panic!("expected a value leaf"),
    }

    let bare = Node {
        data: NodeData::Value(5),
    };
    match drill_read!(&bare.data, NodeData::Prev, data) {
        NodeData::Value(v) => assert_eq!(*v, 5),
        NodeData::Prev(_) => panic!("expected a value leaf"),
    }
}
