# Tutorial

Start from the [greeting](welcome.md). This page adds three ideas to that program. The last listing is the whole file. `cargo run` prints:

```text
Ada
2
4
Ada
Ada
7
ding
wail
```

## A number you can change

`var!` is a `val!` you may write again. `add!` and `set!` need that.

```rust
var!(count = 1);
add!(count, 1);
show!(count);
set!(count = 4);
show!(count);
```

Those lines print `2`, then `4`. The name is written once on the `add!` line.

## A name you keep, and one you take

`val!` duplicates. After the second line, both names are `"Ada"`.

```rust
val!(label = s!("Ada"));
val!(again = label);
show!(label);
show!(again);
```

`val!` asks for `Clone`. A ticket, a file, or a lock is not `Clone`. Bind it with `give!`. The source cannot be used after.

```rust
struct Ticket(i32);

give!(ticket = Ticket(7));
show!(ticket.0);
```

That prints `7`.

## Two types, one function

`obj!` is one concrete type. `iface!` is a trait. `class!` implements it. A `class!` value is not `Clone`, so it is bound with `let`.

```rust
iface!(Alarm {
    fn ring(&self) -> String;
});

class! {
    Bell : [Alarm {
        fn ring(&self) -> String { s!("ding") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

class! {
    Siren : [Alarm {
        fn ring(&self) -> String { s!("wail") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

let bell = Bell::new();
let siren = Siren::new();
let alarms: Vec<&dyn Alarm> = vec![&bell, &siren];
show!(alarms[0].ring());
show!(alarms[1].ring());
```

Those lines print `ding`, then `wail`. The vector does not name `Bell` or `Siren`.

## The whole file

```rust
use alex_xp::*;

struct Ticket(i32);

obj!(Greeter {
    who: String,
    fn new(who: String) -> Self { Self { who } }
    fn hello(&self) -> String { s!(self.who) }
});

iface!(Alarm {
    fn ring(&self) -> String;
});

class! {
    Bell : [Alarm {
        fn ring(&self) -> String { s!("ding") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

class! {
    Siren : [Alarm {
        fn ring(&self) -> String { s!("wail") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

fn main() {
    val!(person = Greeter::new(s!("Ada")));
    show!(person.hello());

    var!(count = 1);
    add!(count, 1);
    show!(count);
    set!(count = 4);
    show!(count);

    val!(label = s!("Ada"));
    val!(again = label);
    show!(label);
    show!(again);

    give!(ticket = Ticket(7));
    show!(ticket.0);

    let bell = Bell::new();
    let siren = Siren::new();
    let alarms: Vec<&dyn Alarm> = vec![&bell, &siren];
    show!(alarms[0].ring());
    show!(alarms[1].ring());
}
```

Next: [Examples](examples.md) if you want finished programs, or the [guide](index.md) if you want every macro.
