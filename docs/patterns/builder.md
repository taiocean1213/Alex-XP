# Builder

Build a value step by step, then use the finished object. `obj!` methods take `self` and return `Self`. That consumes the builder, so each step is a move, not a clone.

```rust
obj!(Burger {
    cheese: bool,
    fn new() -> Self { Self { cheese: false } }
    fn cheese(self) -> Self { Self { cheese: true } }
});

let burger = Burger::new().cheese();
```

`burger.cheese` is `true`. The field is public because `obj!` makes public fields. `&self` is not used here; `self` is written because the step returns a new `Burger`.
