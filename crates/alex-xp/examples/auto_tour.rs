//! The short path: one name per idea, the value written once.

use alex_xp::*;

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

obj!(Greeter {
    who: String,
    fn new(who: String) -> Self {
        Self { who }
    }
    fn hello(&self) -> String {
        s!(self.who)
    }
});

pub fn run() {
    val!(label = s!("Ada"));
    val!(again = label);
    assert_eq!(label, "Ada");
    assert_eq!(again, "Ada");
    show!(label);
    show!(again);

    var!(count = 1);
    add!(count, 1);
    assert_eq!(count, 2);
    show!(count);

    def!(double = |n| n * 2);
    assert_eq!(double(count), 4);
    show!(double(count));

    val!(person = Greeter::new(s!("Ada")));
    assert_eq!(person.hello(), "Ada");
    show!(person.hello());

    give!(
        root = Node {
            value: 0,
            next: Some(heap!(Node {
                value: 1,
                next: None
            })),
        }
    );
    assert_eq!(root.next.as_ref().unwrap().value, 1);
    show!(root.next.as_ref().unwrap().value);

    give!(handed = s!("bye"));
    assert_eq!(handed, "bye");
    show!(handed);
}

#[allow(dead_code)]
fn main() {
    run();
}
