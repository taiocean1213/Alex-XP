use alex_xp::*;

fn main() {
    let token = String::from("a");
    give!(moved = token);
    let _again = token;
    let _ = moved;
}
