use alex_xp::*;

fn main() {
    let mut x = 1;
    read_only_lend!(p = &mut x);
}
