# Alex-XP

A first Rust program asks you to decide, on every name, whether the value stays, changes, or moves. Alex-XP gives those decisions short names: `val!`, `var!`, and `give!`. When two types should fit one function, `iface!` and `class!` are the trait and the struct. You still write a `.rs` file. rustc still accepts or rejects the program.

## Run one program

```sh
cargo new hello-alex
cd hello-alex
```

In `Cargo.toml`:

```toml
[dependencies]
alex-xp = { git = "https://github.com/taiocean1213/Alex-XP" }
```

Replace `src/main.rs` with:

```rust
use alex_xp::*;

obj!(Greeter {
    who: String,
    fn new(who: String) -> Self { Self { who } }
    fn hello(&self) -> String { s!(self.who) }
});

fn main() {
    val!(person = Greeter::new(s!("Ada")));
    show!(person.hello());
}
```

```sh
cargo run
```

The program prints:

```text
Ada
```

`obj!` is the type. `val!` keeps a copy, so the greeter you passed in is still there. `show!` prints. That is the whole library in one file.

## Where to go

- [Tutorial](tutorial.md) adds a counter, a value that cannot be copied, and two types behind one interface. Each step shows the lines and the output.
- [Examples](examples.md) are five finished programs. Clone this repository and run them.
- [Setup](setup.md) is the editor snippets and what a bad macro looks like.
- [Guide](index.md) is the reference: everyday macros, ownership, OOP, SOLID, and the 23 design patterns.
