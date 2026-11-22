# Memento

Save a snapshot and restore it later. `obj!` is `Clone`. `val!` takes the snapshot. Writing the editor afterward does not change the saved copy.

```rust
obj!(Editor {
    text: String,
    fn new(text: String) -> Self { Self { text } }
    fn write(&mut self, text: String) { self.text = text; }
});

var!(editor = Editor::new(s!("a")));
val!(saved = editor);
editor.write(s!("b"));
```

`editor.text` is `"b"`. `saved.text` is `"a"`. Restoring is another `set!` or a move of `saved` back if you no longer need the snapshot. `val!` duplicated the editor, so both bindings are real values.
