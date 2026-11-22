# Command

Turn a request into an object so the invoker does not know the concrete work. The remote holds `Box<dyn Command>` and only calls `execute`.

```rust
iface!(Command {
    fn execute(&self) -> String;
});

class! {
    Say : [Command {
        fn execute(&self) -> String { self.word.clone() }
    }] {
        word: String,
    } {
        fn new(word: String) -> Self { Self { word } }
    }
}

class! {
    Remote {
        slot: Box<dyn Command>,
    } {
        fn new(slot: Box<dyn Command>) -> Self { Self { slot } }
        fn press(&self) -> String { self.slot.execute() }
    }
}

give!(remote = Remote::new(heap!(Say::new(s!("hello")))));
```

`remote.press()` is `"hello"`. `clone` on the `String` field is ordinary Rust. `class!` does not derive `Clone` for `Say`.
