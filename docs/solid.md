# SOLID

Each letter is one small program. Polymorphism uses `iface!` and `class!`. `obj!` is only used where a single concrete type is enough.

## S — one type, one job

A greeter does not also count. Two `obj!` types:

```rust
obj!(Greeter {
    who: String,
    fn new(who: String) -> Self { Self { who } }
    fn hello(&self) -> String { s!(self.who) }
});

obj!(Counter {
    n: i32,
    fn new() -> Self { Self { n: 0 } }
    fn bump(&mut self) { self.n += 1; }
});

val!(greeter = Greeter::new(s!("Ada")));
var!(counter = Counter::new());
counter.bump();
```

`greeter.hello()` is `"Ada"`. `counter.n` is `1`.

## O — add a type, leave the old one

`Bell` stays as it was. `Siren` is a new `class!` on the same `iface!`.

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
```

The rings are `"ding"` then `"wail"`. `&self` is still written in each method.

## L — a child where a parent is expected

`Student` requires `Person`. A `ConcreteStudent` can sit in `&dyn Person`.

```rust
interface! {
    Person {
        fn name(&self) -> String;
    }
}

interface! {
    Student, Person {
        fn university(&self) -> String;
    }
}

class! {
    ConcreteStudent : [
        Person {
            fn name(&self) -> String { self.name.clone() }
        },
        Student {
            fn university(&self) -> String { self.university.clone() }
        }
    ] {
        name: String,
        university: String,
    } {
        fn new(name: String, university: String) -> Self {
            Self { name, university }
        }
    }
}

let student = ConcreteStudent::new(s!("Bob"), s!("MIT"));
let person: &dyn Person = &student;
```

`person.name()` is `"Bob"`.

## I — small interfaces

Printing and scanning are separate traits. A function that only prints does not ask for scanning.

```rust
iface!(Printer {
    fn print(&self) -> String;
});

iface!(Scanner {
    fn scan(&self) -> String;
});

class! {
    Simple : [Printer {
        fn print(&self) -> String { s!("page") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

class! {
    Combo : [
        Printer {
            fn print(&self) -> String { s!("page") }
        },
        Scanner {
            fn scan(&self) -> String { s!("scan") }
        }
    ] {} {
        fn new() -> Self { Self {} }
    }
}

fn printed(p: &dyn Printer) -> String { p.print() }

let simple = Simple::new();
let combo = Combo::new();
```

`printed(&simple)` and `printed(&combo)` are both `"page"`. `combo.scan()` is `"scan"`.

## D — depend on the trait

The desk holds `Box<dyn Alarm>`, not `Bell`. Swapping the alarm does not change `Desk`.

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
    Desk {
        alarm: Box<dyn Alarm>,
    } {
        fn new(alarm: Box<dyn Alarm>) -> Self { Self { alarm } }
        fn alert(&self) -> String { self.alarm.ring() }
    }
}

give!(desk = Desk::new(heap!(Bell::new())));
```

`desk.alert()` is `"ding"`. `give!` is required because `Desk` is not `Clone`. The field type `Box<dyn Alarm>` is ordinary Rust; `class!` does not invent a lifetime for it, and it does not need one.
