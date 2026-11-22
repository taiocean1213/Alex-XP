# Chain of Responsibility

Each handler deals with a request or passes it along a `heap!` link. The link is `Option<Box<dyn Support>>`, so the chain has no lifetime parameter.

```rust
iface!(Support {
    fn handle(&self, n: i32) -> String;
});

class! {
    Limit : [Support {
        fn handle(&self, n: i32) -> String {
            if n <= self.max {
                format!("{} handled {}", self.label, n)
            } else if let Some(next) = &self.next {
                next.handle(n)
            } else {
                s!("none")
            }
        }
    }] {
        label: String,
        max: i32,
        next: Option<Box<dyn Support>>,
    } {
        fn new(label: String, max: i32, next: Option<Box<dyn Support>>) -> Self {
            Self { label, max, next }
        }
    }
}

give!(chain = Limit::new(
    s!("small"),
    10,
    Some(heap!(Limit::new(s!("big"), 100, None))),
));
```

`chain.handle(5)` is `"small handled 5"`. `chain.handle(50)` is `"big handled 50"`. The `if let` is ordinary Rust.
