# Interpreter

A tiny language is a tree of expressions. Numbers and additions both implement `Expr`. Addition holds two `Box<dyn Expr>` children.

```rust
iface!(Expr {
    fn eval(&self) -> i32;
});

class! {
    Num : [Expr {
        fn eval(&self) -> i32 { self.value }
    }] {
        value: i32,
    } {
        fn new(value: i32) -> Self { Self { value } }
    }
}

class! {
    Add : [Expr {
        fn eval(&self) -> i32 { self.left.eval() + self.right.eval() }
    }] {
        left: Box<dyn Expr>,
        right: Box<dyn Expr>,
    } {
        fn new(left: Box<dyn Expr>, right: Box<dyn Expr>) -> Self {
            Self { left, right }
        }
    }
}

give!(sum = Add::new(heap!(Num::new(2)), heap!(Num::new(3))));
```

`sum.eval()` is `5`. The `+` inside `eval` is ordinary Rust.
