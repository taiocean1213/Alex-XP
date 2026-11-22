use alex_xp::*;

#[test]
fn short_names_and_obj() {
    val!(label = s!("Ada"));
    val!(again = label);
    assert_eq!(label, "Ada");
    assert_eq!(again, "Ada");

    var!(count = 1);
    add!(count, 1);
    assert_eq!(count, 2);
    set!(count = 5);
    assert_eq!(count, 5);

    def!(double = |n| n * 2);
    assert_eq!(double(count), 10);

    give!(handed = s!("bye"));
    assert_eq!(handed, "bye");

    obj!(Greeter {
        who: String,
        fn new(who: String) -> Self {
            Self { who }
        }
        fn hello(&self) -> String {
            s!(self.who)
        }
    });
    val!(person = Greeter::new(s!("Ada")));
    assert_eq!(person.hello(), "Ada");
}

#[test]
fn auto_duplicates_infers_and_owns() {
    auto_let!(label = String::from("Ada"));
    auto_let!(again = label);
    assert_eq!(label, "Ada");
    assert_eq!(again, "Ada");

    auto_let_mut!(count = 1);
    auto_edit!(slot = count {
        deref_mutate!(slot = deref_access!(slot) + 1);
    });
    assert_eq!(count, 2);
    auto_set!(count = 5);
    assert_eq!(count, 5);

    auto_share!(view = label);
    auto_own!(kept = view);
    assert_eq!(kept, "Ada");

    let text: &str = "hi";
    auto_own!(owned = text);
    assert_eq!(owned, "hi");

    auto_fn!(double = |n| n * 2);
    assert_eq!(double(count), 10);

    auto_let!(only = String::from("bye"));
    auto_give!(handed = only);
    assert_eq!(handed, "bye");
}

#[test]
fn copy_and_ops() {
    declare_mut_var!(n : i32 = 1, no_mut_ptr);
    copy_assign!(c = n);
    perform_op!(n = n + c);
    assert_eq!(n, 2);
    assert_eq!(c, 1);
    reassign_copy!(n = 5);
    assert_eq!(n, 5);
}

#[test]
fn lend_and_scope() {
    declare_mut_var!(n : i32 = 1, no_mut_ptr);
    scope_block! {
        unique_writable_lend!(w = n);
        deref_mutate!(w = 9);
    }
    read_only_lend!(r = n);
    assert_eq!(deref_access!(r), 9);
}

#[test]
fn interface_and_class() {
    interface! {
        Named {
            fn name(&self) -> &'static str;
        }
    }

    class! {
        Dog : [Named {
            fn name(&self) -> &'static str {
                "dog"
            }
        }] {
            _mark: (),
        } {
            fn new() -> Self {
                Self { _mark: () }
            }
        }
    }

    let d = Dog::new();
    assert_eq!(d.name(), "dog");
}

#[test]
fn drill_to_leaf() {
    enum NodeData {
        Value(i32),
        Prev(Box<Node>),
    }
    struct Node {
        data: NodeData,
    }

    let mut node = Node {
        data: NodeData::Prev(Box::new(Node {
            data: NodeData::Value(3),
        })),
    };

    {
        let leaf = drill_mut!(&mut node.data, NodeData::Prev, data);
        if let NodeData::Value(v) = leaf {
            *leaf = NodeData::Value(*v + 1);
        }
    }

    match drill_read!(&node.data, NodeData::Prev, data) {
        NodeData::Value(v) => assert_eq!(*v, 4),
        NodeData::Prev(_) => panic!("expected leaf"),
    }
}
