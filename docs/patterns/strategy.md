# Strategy

Swap the algorithm without changing the caller. The context stores `Box<dyn SortStrategy>` and calls it.

```rust
iface!(SortStrategy {
    fn label(&self) -> String;
});

class! {
    ByName : [SortStrategy {
        fn label(&self) -> String { s!("name") }
    }] {} {
        fn new() -> Self { Self {} }
    }
}

class! {
    Report {
        strategy: Box<dyn SortStrategy>,
    } {
        fn new(strategy: Box<dyn SortStrategy>) -> Self { Self { strategy } }
        fn sorted(&self) -> String { self.strategy.label() }
    }
}

give!(report = Report::new(heap!(ByName::new())));
```

`report.sorted()` is `"name"`. A second algorithm is another `class!` on `SortStrategy`, passed to `Report::new`. The trait method is still written with `&self`.
