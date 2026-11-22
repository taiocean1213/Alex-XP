# Abstract Factory

One factory builds a matching family of products. Adding a new family is a new `class!`. The old products stay.

Both products are traits. The factory's methods return `Box<dyn …>`, which `class!` does not hide.

```rust
iface!(Button {
    fn paint(&self) -> String;
});

iface!(Checkbox {
    fn paint(&self) -> String;
});

class! {
    WinButton : [Button {
        fn paint(&self) -> String { s!("win-button") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

class! {
    WinCheckbox : [Checkbox {
        fn paint(&self) -> String { s!("win-box") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

iface!(WidgetFactory {
    fn button(&self) -> Box<dyn Button>;
    fn checkbox(&self) -> Box<dyn Checkbox>;
});

class! {
    WinFactory : [WidgetFactory {
        fn button(&self) -> Box<dyn Button> { heap!(WinButton::new()) }
        fn checkbox(&self) -> Box<dyn Checkbox> { heap!(WinCheckbox::new()) }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

let factory = WinFactory::new();
```

`factory.button().paint()` is `"win-button"`. `factory.checkbox().paint()` is `"win-box"`.
