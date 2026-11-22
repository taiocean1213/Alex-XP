# Singleton

One shared global is a poor fit. Alex-XP has no static instance macro, and a mutable global fights the borrow checker.

Create the one value in `main` (or in the test) and pass it down:

```rust
obj!(Config {
    path: String,
    fn new(path: String) -> Self { Self { path } }
});

fn run(config: &Config) -> String { s!(config.path) }

let config = Config::new(s!("app.toml"));
```

`run(&config)` is `"app.toml"`. Callers receive `&Config`. They do not look up a hidden global.
