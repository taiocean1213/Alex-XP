# OOP

Use `obj!` when the type does not need to be swapped with another type. Use `iface!` and `class!` when a function should accept more than one type.

## One type

`obj!` writes the struct and its methods once. Fields are `name: Type,` with a comma. The struct is public and `Clone`, so `val!` can hold it. Methods are public. You still write `&self` and the types.

```rust
obj!(Greeter {
    who: String,
    fn new(who: String) -> Self {
        Self { who }
    }
    fn hello(&self) -> String {
        s!(self.who)
    }
});

val!(person = Greeter::new(s!("Ada")));
```

`person.hello()` is `"Ada"`.

## A trait and a class

`iface!(Name { ... });` is a trait. `iface!(Child: Parent { ... });` is a trait that requires the parent.

`class!` is the form that implements traits. List every trait you implement, including the parent. rustc rejects a class that implements `Student` and not `Person`.

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

class! {
    ConcreteStudent : [
        Person {
            fn name(&self) -> String {
                self.name.clone()
            }
        },
        Student {
            fn university(&self) -> String {
                self.university.clone()
            }
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

let person = ConcretePerson::new(s!("Alice"));
let student = ConcreteStudent::new(s!("Bob"), s!("MIT"));
let people: Vec<&dyn Person> = vec![&person, &student];
```

Both names are reachable through `&dyn Person`. `class!` uses braces, so this invocation does not need a semicolon after the closing brace. The same program is `cargo run -p alex-xp --example person_student`.

`obj!` does not implement an `iface!`. A method written on `obj!` is only an inherent method.
