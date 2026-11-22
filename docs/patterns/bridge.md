# Bridge

Split the abstraction from the implementation so each can vary. Two `iface!`s. The abstraction holds `Box<dyn Engine>`.

```rust
iface!(Engine {
    fn go(&self) -> String;
});

class! {
    Electric : [Engine {
        fn go(&self) -> String { s!("zap") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

iface!(Switch {
    fn flip(&self) -> String;
});

class! {
    Toggle : [Switch {
        fn flip(&self) -> String { self.engine.go() }
    }] {
        engine: Box<dyn Engine>,
    } {
        fn new(engine: Box<dyn Engine>) -> Self { Self { engine } }
    }
}

give!(toggle = Toggle::new(heap!(Electric::new())));
```

`toggle.flip()` is `"zap"`. A second engine is another `class!` on `Engine`. `Toggle` does not change. `Box<dyn Engine>` is still written in the field.
