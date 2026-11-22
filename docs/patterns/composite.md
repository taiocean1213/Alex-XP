# Composite

A group and a leaf share one trait, so a caller can draw either. The group stores `Vec<Box<dyn Graphic>>` and joins the child drawings.

```rust
iface!(Graphic {
    fn draw(&self) -> String;
});

class! {
    Dot : [Graphic {
        fn draw(&self) -> String { s!("dot") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

class! {
    Group : [Graphic {
        fn draw(&self) -> String {
            self.parts.iter().map(|part| part.draw()).collect::<Vec<_>>().join(",")
        }
    }] {
        parts: Vec<Box<dyn Graphic>>,
    } {
        fn new(parts: Vec<Box<dyn Graphic>>) -> Self { Self { parts } }
    }
}

give!(group = Group::new(vec![heap!(Dot::new()), heap!(Dot::new())]));
```

`group.draw()` is `"dot,dot"`. The `map` and `join` are ordinary Rust. `class!` does not generate the loop.
