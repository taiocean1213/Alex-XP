# Alex-XP guide

Alex-XP is a set of Rust macros. You still write `.rs` files. rustc checks the program after the macros expand.

## Limits

These stay written out, because the macros cannot invent them:

- Field types, and `&self` / `&mut self` on methods.
- `class!` has no generics, no lifetime parameter, and no derives. Store owned data (`String`, `i32`, `Box<dyn Trait>`) so the class has no lifetime.
- `val!` and `var!` require `Clone`. A value that is not `Clone` (a file, a lock, a `Box<dyn Trait>`) is bound with `give!` or with `let` / `let mut`.
- Method bodies inside `class!` and `obj!` are ordinary Rust.

`obj!` builds a `Clone` struct and its methods. It does not implement a trait. Polymorphism uses `iface!` / `interface!` and `class!`.

## Chapters

- [Setup](setup.md)
- [Everyday macros](everyday.md)
- [Ownership](ownership.md)
- [OOP](oop.md)
- [SOLID](solid.md)

## Patterns

Creational

- [Factory Method](patterns/factory-method.md)
- [Abstract Factory](patterns/abstract-factory.md)
- [Builder](patterns/builder.md)
- [Prototype](patterns/prototype.md)
- [Singleton](patterns/singleton.md)

Structural

- [Adapter](patterns/adapter.md)
- [Bridge](patterns/bridge.md)
- [Composite](patterns/composite.md)
- [Decorator](patterns/decorator.md)
- [Facade](patterns/facade.md)
- [Flyweight](patterns/flyweight.md)
- [Proxy](patterns/proxy.md)

Behavioral

- [Chain of Responsibility](patterns/chain.md)
- [Command](patterns/command.md)
- [Interpreter](patterns/interpreter.md)
- [Iterator](patterns/iterator.md)
- [Mediator](patterns/mediator.md)
- [Memento](patterns/memento.md)
- [Observer](patterns/observer.md)
- [State](patterns/state.md)
- [Strategy](patterns/strategy.md)
- [Template Method](patterns/template-method.md)
- [Visitor](patterns/visitor.md)

People changing the crate follow [CONTRIBUTING.md](../CONTRIBUTING.md).
