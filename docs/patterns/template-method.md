# Template Method

The skeleton lives once, on the trait. Steps that vary are implemented by each `class!`. A default method in `interface!` is ordinary Rust inside the trait body.

```rust
interface! {
    Report {
        fn title(&self) -> String;
        fn render(&self) -> String {
            format!("# {}", self.title())
        }
    }
}

class! {
    Note : [Report {
        fn title(&self) -> String { s!("note") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

let note = Note::new();
```

`note.render()` is `"# note"`. `Note` does not implement `render`. The default on `Report` does. `format!` is ordinary Rust. `class!` will not fill in `title` for you.
