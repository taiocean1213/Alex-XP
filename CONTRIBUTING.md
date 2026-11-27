# Contributing to Alex-XP

Thanks for helping. Alex-XP is a Rust macro crate plus an unpacked editor extension. rustc still checks the program after the macros expand.

Contributions are licensed under MIT OR Apache-2.0, the same terms as the crate. By opening a pull request you agree that your contribution can be used under those licenses.

## Setup

Install [rustup](https://rustup.rs). This repo pins the compiler in `rust-toolchain.toml` so compile-fail snapshots match one rustc. The first `cargo` command in the clone installs that toolchain, including rustfmt and clippy.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Those three commands are what CI runs. `cargo test` also compiles the examples through `crates/alex-xp/tests/programs.rs`. To watch one example:

```sh
cargo run -p alex-xp --example auto_tour
```

`Cargo.lock` is gitignored. This is a library, so each build resolves dependencies from `Cargo.toml`.

## Layout

| Path | What it is |
| --- | --- |
| `crates/alex-xp/src` | The macros. Short names live in `short.rs`. |
| `crates/alex-xp/tests` | Runtime checks, guide samples, compile-fail tests, editor alignment. |
| `crates/alex-xp/examples` | Programs you can `cargo run`. |
| `docs` | The guide. Samples are copied into `tests/guide.rs` so they compile in CI. |
| `editors/vscode` | Highlighting and snippets. Cargo does not install this. |

A bad macro *shape* is a `compile_error!` that names the expected form. A bad type or borrow stays a normal rustc error. `obj!` and `iface!` are `local_inner_macros`, so their errors use `::core::compile_error!`.

## Add a macro

1. Put the `#[macro_export]` in the module that matches the idea: `short.rs`, `auto.rs`, `ownership.rs`, `oop.rs`, or `drill.rs`.
2. Reject a bad shape with `compile_error!` and the form you expect.
3. Add a test that calls the macro and checks the value rustc accepted.
4. Add a compile-fail file under `crates/alex-xp/tests/ui`. Refresh its snapshot only for that case, then read the diff:

   ```sh
   $env:TRYBUILD = "overwrite"
   cargo test -p alex-xp --test ui
   ```

   On Unix, `TRYBUILD=overwrite cargo test -p alex-xp --test ui`. Leave `TRYBUILD` unset for a normal run.
5. Add the name to the list in `crates/alex-xp/tests/editor.rs`, to the grammar in `editors/vscode/syntaxes/alex-xp.injection.json`, and as a snippet prefix in `editors/vscode/snippets/alex-xp.json`. The editor test fails when a public macro is missing from either file. The snippet prefix for `s!` is `s!`; every other prefix is the macro name without `!`.
6. If callers should type it, add a row to the README table and a sample under `docs/`. Copy that sample into `tests/guide.rs`.

## Change the guide

A new pattern page goes in `docs/patterns/` and is linked from `docs/index.md`. The fenced sample also belongs in `tests/guide.rs`. The guide states limits the macros do not paper over (field types, `&self` / `&mut self`, no generics on `class!`). A page that needs one of those limits should say so and show the form that compiles today.

## Pull requests

Keep a pull request to one behavior when you can. Say what a caller can write afterward. Fill in the checklist: format, clippy, tests, and the editor files when a macro changed.

Record user-visible changes under `Unreleased` in [CHANGELOG.md](CHANGELOG.md).

## Issues

Use the bug or feature template. A compile error report needs the macro call, `rustc --version`, and the rustc output. Feature ideas should show the call you want to write and what rustc should still reject.

Questions about how a current macro behaves belong on the guide first: [docs/index.md](docs/index.md).
