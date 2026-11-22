# State

The same machine behaves differently after you swap its mode. The context holds `Box<dyn Mode>` and replaces it with `&mut self`.

```rust
iface!(Mode {
    fn name(&self) -> String;
});

class! {
    Off : [Mode {
        fn name(&self) -> String { s!("off") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

class! {
    On : [Mode {
        fn name(&self) -> String { s!("on") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

class! {
    Machine {
        mode: Box<dyn Mode>,
    } {
        fn new(mode: Box<dyn Mode>) -> Self { Self { mode } }
        fn label(&self) -> String { self.mode.name() }
        fn set_mode(&mut self, mode: Box<dyn Mode>) { self.mode = mode; }
    }
}

let mut machine = Machine::new(heap!(Off::new()));
```

`machine.label()` is `"off"`. After `machine.set_mode(heap!(On::new()))`, `machine.label()` is `"on"`. `let mut` is required because `Machine` is not `Clone`.
