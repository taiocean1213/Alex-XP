# Facade

One type is the door to several others. Callers start the computer. They do not turn on each part.

```rust
obj!(Cpu {
    fn new() -> Self { Self {} }
    fn on(&self) -> String { s!("cpu") }
});

obj!(Screen {
    fn new() -> Self { Self {} }
    fn on(&self) -> String { s!("screen") }
});

obj!(Computer {
    fn new() -> Self { Self {} }
    fn start(&self) -> String {
        format!("{} {}", Cpu::new().on(), Screen::new().on())
    }
});

val!(computer = Computer::new());
```

`computer.start()` is `"cpu screen"`. `format!` is ordinary Rust. The parts are constructed inside `start` because this facade does not need to store them.
