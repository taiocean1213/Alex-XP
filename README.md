# Alex-XP

A small set of Rust macros so you can write code with **named ownership** and **OOP-shaped types**, while rustc still checks the real program.

Think of it as spectacles: `interface!` / `class!` look like well-taught OOP; `declare_mut_var!` / `unique_writable_lend!` name move vs copy vs lend. Method signatures still use `&self` / `&mut self`.

A weak LLM can stay inside this vocabulary (snippets in the editor extension help).

## New project

Upstream is [github.com/taiocean1213/Alex-XP](https://github.com/taiocean1213/Alex-XP). From a new crate:

```sh
cargo new hello-alex
cd hello-alex
```

```toml
[dependencies]
alex-xp = { git = "https://github.com/taiocean1213/Alex-XP" }
```

That git dependency resolves once this library is on the default branch of that repo. Until then, depend on a local checkout:

```toml
alex-xp = { path = "C:/Users/ASUS/Downloads/Alex-XP/crates/alex-xp" }
```

Write `src/main.rs`:

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

    var!(count = 1);
    add!(count, 1);
    show!(count);
}
```

```sh
cargo run
```

| You type | It does |
| --- | --- |
| `val!(name = expr)` | Immutable binding, type inferred, source stays |
| `var!(name = expr)` | Same, but mutable |
| `add!(name, n)` | `name = name + n` — the name is written once |
| `set!(name = expr)` | Replace it, source stays |
| `give!(name = expr)` | Take it (files, locks, anything not `Clone`) |
| `s!("Ada")` | A `String` |
| `show!(expr)` | Print |
| `def!(name = \|arg\| expr)` | Function, types inferred |
| `heap!(expr)` | `Box`, for a link in a tree |
| `obj!(Name { field: Type, fn ... });` | Struct and methods, written once, `Clone` |
| `iface!(Name { ... });` | A trait, only when you need `dyn` |

`val!` duplicates (`Clone`). Prefer owned fields (`String`, `i32`) so a class has no lifetime. The longer names (`auto_let!`, `class!`, `declare_*!`) are the same ideas spelled out.

## Explicit vocabulary

The `declare_*` / `*_lend!` macros name the decision yourself when the automatic choice is wrong.

```rust
use alex_xp::*;

interface! {
    Person {
        fn name(&self) -> String;
    }
}

class! {
    ConcretePerson : [Person {
        fn name(&self) -> String {
            self.name.clone()
        }
    }] {
        name: String,
    } {
        fn new(name: String) -> Self {
            Self { name }
        }
    }
}
```

### Ownership vocabulary

| You write | Meaning |
| --- | --- |
| `val!` / `var!` / `set!` / `give!` / `add!` | Short bindings (see above) |
| `declare_immut_var!` / `declare_mut_var!` | Bind, with `no_mut_ptr` or `mut_ptr` |
| `declare_const!` | `const` |
| `ownership_assign!` / `reassign_ownership!` | Move |
| `copy_assign!` / `reassign_copy!` | Copy (`Copy` only) |
| `read_only_lend!` | Inserts `&` |
| `unique_writable_lend!` | Inserts `&mut` |
| `deref_access!` / `deref_mutate!` | Hide `*` |
| `scope_block!` | End lends at `}` |
| `delete_var!` | `drop` |
| `perform_op!` / `print_value!` | Thin ops |
| `drill_mut!` / `drill_read!` | Walk a `Prev` box chain to the leaf |

Bad *syntax* is a custom `compile_error!`. Bad *types and borrows* still come from rustc.

## Examples

From the repo root:

```sh
cargo run -p alex-xp --example auto_tour
cargo run -p alex-xp --example person_student
cargo run -p alex-xp --example ownership_tour
cargo run -p alex-xp --example owned_conversion
cargo run -p alex-xp --example drill_chain
cargo test -p alex-xp
```

`owned_conversion` returns an **owned** output from `&mut` input. There is no leaked `'static` helper in the crate.

## Editor

Unpacked extension: [editors/vscode](editors/vscode). See [editors/vscode/README.md](editors/vscode/README.md) for Cursor install. Highlighting injects into `source.rust`; diagnostics are rust-analyzer plus macro `compile_error!`.
