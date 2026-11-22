use alex_xp::*;

obj!(Greeter {
    who: String
    fn new(who: String) -> Self {
        Self { who }
    }
});

fn main() {}
