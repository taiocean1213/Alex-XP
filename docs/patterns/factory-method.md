# Factory Method

A creator asks a subclass which product to build. The caller sees `Box<dyn Button>` and does not name the concrete button.

`class!` returns the box from a method. The method signature, including `Box<dyn Button>`, is still Rust.

```rust
iface!(Button {
    fn paint(&self) -> String;
});

class! {
    WinButton : [Button {
        fn paint(&self) -> String { s!("win") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

iface!(Dialog {
    fn create(&self) -> Box<dyn Button>;
});

class! {
    WinDialog : [Dialog {
        fn create(&self) -> Box<dyn Button> { heap!(WinButton::new()) }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

let dialog = WinDialog::new();
let button = dialog.create();
```

`button.paint()` is `"win"`.
