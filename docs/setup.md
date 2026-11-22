# Setup

Create a normal Rust project and depend on this checkout. The git URL `https://github.com/taiocean1213/Alex-XP` is the upstream. It only builds once this crate is on that repo's default branch. Until then, use a path:

```sh
cargo new hello-alex
cd hello-alex
```

```toml
[dependencies]
alex-xp = { path = "C:/Users/ASUS/Downloads/Alex-XP/crates/alex-xp" }
```

`src/main.rs`:

```rust
use alex_xp::*;

obj!(Greeter {
    who: String,
    fn new(who: String) -> Self {
        Self { who }
    }
    fn hello(&self) -> String {
        s!(self.who)
    }
});

fn main() {
    val!(person = Greeter::new(s!("Ada")));
    show!(person.hello());
}
```

```sh
cargo run
```

`use alex_xp::*;` is required in every file that calls the macros. `obj!(...);` ends with a semicolon because the macro uses parentheses and expands to items.

## Editor

Highlighting and snippets are a separate unpacked extension at [editors/vscode](../editors/vscode). Cargo does not install it.

1. Command Palette → **Developer: Install Extension from Location...** and choose `editors/vscode`.
2. If that command is missing, copy the folder to `%USERPROFILE%\.cursor\extensions\alex-xp` and reload.

Type `val` or `obj` in a `.rs` file and accept the snippet. The snippet inserts the parentheses and braces. Typing `{` by itself only inserts the matching `}`, which is Cursor's Rust behavior, not Alex-XP.

## Errors

rust-analyzer is the linter. A bad macro shape becomes `compile_error!` (for example `val!(name = expr)`). A type or borrow mistake is a normal rustc error on the expanded code. Enable checks with:

```json
{
  "rust-analyzer.check.command": "check"
}
```
