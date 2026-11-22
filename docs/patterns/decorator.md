# Decorator

Add behavior around an object that already implements the trait. The wrapper holds `Box<dyn Named>` and calls the inner method.

```rust
iface!(Named {
    fn name(&self) -> String;
});

class! {
    Plain : [Named {
        fn name(&self) -> String { s!("ada") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

class! {
    Loud : [Named {
        fn name(&self) -> String { format!("!!{}", self.inner.name()) }
    }] {
        inner: Box<dyn Named>,
    } {
        fn new(inner: Box<dyn Named>) -> Self { Self { inner } }
    }
}

give!(loud = Loud::new(heap!(Plain::new())));
```

`loud.name()` is `"!!ada"`. You write the forward yourself. `class!` does not copy methods onto the wrapper. `format!` is ordinary Rust.
