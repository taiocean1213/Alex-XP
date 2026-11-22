# Visitor

`class!` cannot do generic double dispatch. There is no way to add a new operation across several classes without editing each `class!` or using generics the macro does not accept.

The working form is an enum and a `match`. Each new operation is a new function. Each new shape is a new enum variant, and every function must handle it.

```rust
enum Shape {
    Circle(i32),
    Square(i32),
}

fn area(shape: &Shape) -> i32 {
    match shape {
        Shape::Circle(radius) => radius * radius,
        Shape::Square(side) => side * side,
    }
}

let circle = Shape::Circle(3);
let square = Shape::Square(3);
```

`area(&circle)` is `9`. `area(&square)` is `9`. The `match` is ordinary Rust. A visitor trait with an `accept` method on each `class!` can be written by hand, and each class still lists the call. That repeats the same limitation, so this page uses the enum.
