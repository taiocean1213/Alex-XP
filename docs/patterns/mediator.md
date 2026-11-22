# Mediator

Colleagues talk through one tower instead of calling each other. The tower is an `obj!`. A plane only receives `&Tower`.

```rust
obj!(Tower {
    fn new() -> Self { Self {} }
    fn announce(&self, from: &str, msg: &str) -> String {
        format!("{from}: {msg}")
    }
});

obj!(Plane {
    name: String,
    fn new(name: String) -> Self { Self { name } }
    fn radio(&self, tower: &Tower, msg: &str) -> String {
        tower.announce(&self.name, msg)
    }
});

val!(tower = Tower::new());
val!(plane = Plane::new(s!("north")));
```

`plane.radio(&tower, "landing")` is `"north: landing"`. `format!` and the `&str` parameters are ordinary Rust.
