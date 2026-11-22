# Proxy

Control access to an object. This proxy refuses to forward until it is enabled. The real worker still implements the same trait.

```rust
iface!(Worker {
    fn work(&self) -> String;
});

class! {
    Real : [Worker {
        fn work(&self) -> String { s!("done") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

class! {
    Guard : [Worker {
        fn work(&self) -> String {
            if self.enabled { self.inner.work() } else { s!("blocked") }
        }
    }] {
        inner: Box<dyn Worker>,
        enabled: bool,
    } {
        fn new(inner: Box<dyn Worker>, enabled: bool) -> Self { Self { inner, enabled } }
    }
}

give!(blocked = Guard::new(heap!(Real::new()), false));
give!(open = Guard::new(heap!(Real::new()), true));
```

`blocked.work()` is `"blocked"`. `open.work()` is `"done"`. The `if` is ordinary Rust.
