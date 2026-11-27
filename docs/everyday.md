# Everyday macros

These are the names to type. Each one infers local types. rustc still rejects a type that is not `Clone` when you use `val!` or `var!`.

```rust
use alex_xp::*;

fn main() {
    val!(label = s!("Ada"));
    val!(again = label);
    show!(label);
    show!(again);

    var!(count = 1);
    add!(count, 1);
    set!(count = 4);
    show!(count);

    def!(double = |n| n * 2);
    show!(double(count));

    give!(handed = s!("bye"));
    show!(handed);
}
```

| Macro | Expansion idea |
| --- | --- |
| `val!(name = expr)` | Immutable binding. Duplicates `expr` so the source stays. |
| `var!(name = expr)` | Same, and the binding is mutable. |
| `add!(name, n)` | `name += n`. The name is written once. |
| `set!(name = expr)` | Replace a mutable binding with a duplicate of `expr`. |
| `give!(name = expr)` | Move `expr`. Use this when the type is not `Clone`. |
| `s!("Ada")` | A `String`. `s!(other)` stringifies `other`. |
| `show!(expr)` | Print `expr`. |
| `def!(name = \|arg\| expr)` | A closure. Argument and return types are inferred from the call. |
| `heap!(expr)` | `Box::new(expr)`, an expression, so it can nest. |

`def!` infers `n` only after you call `double(count)`. A closure that is never called may still need a type annotation. That annotation is ordinary Rust.
