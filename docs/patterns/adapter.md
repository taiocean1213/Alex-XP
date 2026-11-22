# Adapter

Wrap a type whose method has the wrong name so callers can use one trait. The adapter is a `class!` that holds the old type and forwards.

```rust
obj!(Legacy {
    fn new() -> Self { Self {} }
    fn old(&self) -> String { s!("old") }
});

iface!(Target {
    fn request(&self) -> String;
});

class! {
    Adapter : [Target {
        fn request(&self) -> String { self.legacy.old() }
    }] {
        legacy: Legacy,
    } {
        fn new(legacy: Legacy) -> Self { Self { legacy } }
    }
}

give!(adapter = Adapter::new(Legacy::new()));
```

`adapter.request()` is `"old"`. `give!` is used because `class!` types are not `Clone`. The forward call `self.legacy.old()` is ordinary Rust.
