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

fn main() {
    val!(label = s!("Ada"));
    val!(again = label);
    show!(label);
    show!(again);

    var!(count = 1);
    add!(count, 1);
    show!(count);

    def!(double = |n| n * 2);
    show!(double(count));

    val!(person = Greeter::new(s!("Ada")));
    show!(person.hello());

    give!(root = Node {
        value: 0,
        next: Some(heap!(Node { value: 1, next: None })),
    });
    show!(root.next.as_ref().unwrap().value);

    give!(handed = s!("bye"));
    show!(handed);
}
