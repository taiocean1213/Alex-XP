use alex_xp::*;

struct Token(i32);

fn main() {
    let token = Token(1);
    val!(copied = token);
    let _ = copied;
}
