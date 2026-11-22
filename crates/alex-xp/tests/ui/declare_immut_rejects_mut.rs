use alex_xp::*;

fn main() {
    let mut x = 1;
    declare_immut_var!(p = &mut x, no_mut_ptr);
}
