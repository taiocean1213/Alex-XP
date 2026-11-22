# Alex-XP editor support

This unpacked VS Code / Cursor extension injects keyword highlighting into Rust files and adds snippets for the Alex-XP macros.

## Install in Cursor (or VS Code)

1. Open the Command Palette.
2. Run **Developer: Install Extension from Location...** (Cursor) or **Extensions: Install from VSIX...** if you package one.
3. If your build has no “from location” command:
   - Copy or symlink this folder into your extensions directory, then reload:
     - Windows: `%USERPROFILE%\.cursor\extensions\alex-xp`
     - macOS/Linux: `~/.cursor/extensions/alex-xp`
4. Reload the window.

You can also open this repo and use **Extensions: Load Extension from Folder** if your Cursor version lists it.

## What you get

- Highlighting of `class!`, `interface!`, `declare_*!`, `*_lend!`, `no_mut_ptr` / `mut_ptr`, and the rest of the v1 vocabulary.
- Nested `source.rust` highlighting inside `class! { ... }` and `interface! { ... }`.
- Snippets for every legal v1 form (type `declare_immut_var` or `interface` in a `.rs` file).

## Diagnostics (the linter)

There is no extra LSP. **rust-analyzer** plus our `compile_error!` messages *are* the linter.

Enable rust-analyzer in Cursor, then save a `.rs` file that depends on `alex-xp`. Invalid DSL shape shows an Alex-XP message (for example `read_only_lend! does not allow unique writable lends`). Type and borrow mistakes still come from rustc.

If checks are quiet, set:

```json
{
  "rust-analyzer.check.command": "check"
}
```
