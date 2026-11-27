# Examples

These run from a clone of this repository, at the repository root. They are not part of the `hello-alex` crate in the [tutorial](tutorial.md).

```sh
git clone https://github.com/taiocean1213/Alex-XP
cd Alex-XP
```

## A short tour

```sh
cargo run -p alex-xp --example auto_tour
```

Prints the greeting, a doubled count, and a one-node tree:

```text
Ada
Ada
2
4
Ada
1
bye
```

`1` is the child node's value. `bye` is a value taken with `give!`.

## A person and a student

```sh
cargo run -p alex-xp --example person_student
```

```text
Person name: Alice
Student name: Bob
Student university: MIT
Polymorphic name: Alice
Polymorphic name: Bob
```

`Student` requires `Person`. Both values sit in one `&dyn Person` list. The same program is written out on the [OOP](oop.md) page.

## Ownership, spelled out

```sh
cargo run -p alex-xp --example ownership_tour
```

```text
2
42
n
10
10
hello
world
7
```

This is the long vocabulary: copy, lend, drop, and a write through `&mut`. The [ownership](ownership.md) page is the map. New code can stay on `val!`, `var!`, `give!`, and `heap!`.

## An owned result

```sh
cargo run -p alex-xp --example owned_conversion
```

```text
Input after recursion: value=13, numbers=[1.0, 2.0, 3.0, 13.0], text=hello_processed
Output: value=26, numbers=[1.5, 3.0, 4.5, 19.5], result=6
```

The function takes `&mut` input and returns an owned value. Nothing in the crate leaks a `'static` borrow to do that.

## A chain of boxes

```sh
cargo run -p alex-xp --example drill_chain
```

```text
Initial lowest child's value: 16
Updated lowest child's value: 100
```

`drill_read!` and `drill_mut!` walk a `Prev` box chain to the leaf.

Every example is also compiled by `cargo test -p alex-xp`.
