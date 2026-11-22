# Flyweight

Many glyphs need the same font name. `val!` duplicates that `String` into each glyph. The glyphs do not share one allocation.

Alex-XP has no shared-ownership macro. If two owners must point at one heap node, that is `Rc` in ordinary Rust, not `heap!`. `heap!` still has a single owner.

```rust
obj!(Glyph {
    font: String,
    ch: String,
    fn new(font: String, ch: String) -> Self { Self { font, ch } }
});

val!(font = s!("sans"));
give!(left = Glyph::new(s!(font), s!("A")));
give!(right = Glyph::new(s!(font), s!("B")));
```

`left.font` and `right.font` are both `"sans"`. `s!(font)` copies the text into each glyph. `font` itself stays usable because `val!` bound it.
