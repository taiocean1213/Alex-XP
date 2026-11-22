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

Write `src/main.rs` with the automatic vocabulary (no local types, no `&`, no `'a`):

```rust
use alex_xp::*;

fn main() {
    auto_let!(label = String::from("Ada"));
    auto_let!(again = label);
    print_value!(label);

    auto_let_mut!(count = 1);
    auto_edit!(slot = count {
        deref_mutate!(slot = deref_access!(slot) + 1);
    });

    auto_share!(view = label);
    auto_own!(kept = view);
    print_value!(kept);
}
```

```sh
cargo run
```

`auto_let!` duplicates (`Clone`, which copies `Copy` types) so the source stays usable. `auto_give!` is the explicit hand-off when a value cannot be duplicated (files, locks). `auto_share!` lends without writing `&`. `auto_edit!` lends uniquely and ends that lend at the closing brace. `auto_own!` turns `&str` / `&[T]` into `String` / `Vec<T>` so the result has no lifetime. `auto_box!` stores a value in `Box` for recursive structures. `auto_fn!` is a local closure with inferred argument and return types.

Class fields and `&self` / `&mut self` stay written out. A struct cannot infer them. Prefer owned field types (`String`, `Vec<T>`, `i32`) so the class has no lifetime parameter.

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
| `auto_let!` / `auto_let_mut!` | Infer the type and duplicate (`Clone`) |
| `auto_set!` | Duplicate into an existing mutable binding |
| `auto_give!` | Infer the type and take ownership |
| `auto_share!` | Lend without writing `&` or a lifetime |
| `auto_edit!` | Unique lend that ends with the block |
| `auto_own!` | Lend → owned (`&str` becomes `String`) |
| `auto_box!` | Heap link without a lifetime |
| `auto_fn!` | Closure with inferred argument and return types |
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

## Historical sketches

The folders `class-automation-macros/`, `lifetime-automation-macros/`, and `mut-reference-drilling/` are the original standalone `rustc` sketches. The product surface is `crates/alex-xp`.
