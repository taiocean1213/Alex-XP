# Iterator

Walk a sequence without the caller knowing how it is stored. This `obj!` is a cursor. `var!` works because `obj!` is `Clone`. `next` takes `&mut self`, which you still write.

```rust
obj!(Counter {
    at: i32,
    stop: i32,
    fn new(stop: i32) -> Self { Self { at: 0, stop } }
    fn next(&mut self) -> Option<i32> {
        if self.at >= self.stop {
            None
        } else {
            self.at += 1;
            Some(self.at)
        }
    }
});

var!(counter = Counter::new(2));
```

The first `counter.next()` is `Some(1)`. The second is `Some(2)`. The third is `None`. This is not Rust's `Iterator` trait. Implementing that trait would be an ordinary `impl Iterator for Counter` beside the `obj!`.
