# Ownership

`val!` and `var!` call `Clone`. Numbers copy. `String` clones. The name you read on the right stays usable.

```rust
val!(label = s!("Ada"));
val!(again = label);
```

After this, `label` and `again` are both `"Ada"`.

`give!` moves. The source cannot be used afterward. Use it for values that are not `Clone`, including most `class!` values and any `Box<dyn Trait>`.

```rust
give!(handed = s!("bye"));
```

`add!` and `set!` need a mutable binding from `var!`:

```rust
var!(count = 1);
add!(count, 1);
set!(count = 4);
```

`var!` cannot bind a non-`Clone` value. A list you push into is ordinary `let mut`.

## Trees without lifetimes

A recursive shape stores a `Box`, not a reference, so the struct has no lifetime parameter.

```rust
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

give!(root = Node {
    value: 0,
    next: Some(heap!(Node { value: 1, next: None })),
});
```

`heap!` moves the inner node onto the heap. One owner. No `'a`.

Prefer `String` and `Vec<T>` fields on `obj!` and `class!` for the same reason. A field of type `&str` does not fit: `class!` cannot add a lifetime to the struct, and `obj!` cannot either.

## When you want the decision named

The longer macros (`declare_immut_var!`, `unique_writable_lend!`, `read_only_lend!`, `deref_access!`, `auto_share!`, `auto_edit!`, `auto_own!`) are listed in the [README](../README.md#explicit-vocabulary). They are the same moves, copies, and lends spelled out. New code should stay on `val!`, `var!`, `give!`, and `heap!`.
