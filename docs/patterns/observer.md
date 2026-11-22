# Observer

A feed stores listeners and notifies all of them. The feed is `let mut` because `Vec<Box<dyn Listener>>` is not `Clone`, so `var!` cannot bind it.

```rust
iface!(Listener {
    fn on(&self, msg: &str) -> String;
});

class! {
    Log : [Listener {
        fn on(&self, msg: &str) -> String { s!(msg) }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

class! {
    Feed {
        subs: Vec<Box<dyn Listener>>,
    } {
        fn new() -> Self { Self { subs: Vec::new() } }
        fn subscribe(&mut self, sub: Box<dyn Listener>) { self.subs.push(sub); }
        fn publish(&self, msg: &str) -> Vec<String> {
            self.subs.iter().map(|sub| sub.on(msg)).collect()
        }
    }
}

let mut feed = Feed::new();
feed.subscribe(heap!(Log::new()));
```

`feed.publish("hi")` is `vec!["hi"]`. `&mut self` and the `map` are ordinary Rust.
