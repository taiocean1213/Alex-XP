# Prototype

Copy an existing object instead of calling the constructor again. `obj!` derives `Clone`, and `val!` clones.

```rust
obj!(Sheep {
    name: String,
    fn new(name: String) -> Self { Self { name } }
});

val!(original = Sheep::new(s!("dolly")));
val!(copy = original);
```

`copy.name` and `original.name` are both `"dolly"`. Later changes to one binding do not change the other, because `val!` duplicated the sheep.
