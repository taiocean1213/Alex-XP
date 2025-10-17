fn recursive_function(value: &mut i32, depth: usize) {
    if depth == 0 {
        return;
    }

    *value += 1; // Mutate the value
    println!("Depth: {}, Value: {}", depth, *value);

    // Recursive call with the same mutable reference
    recursive_function(value, depth - 1);
}

fn main() {
    let mut number = 0;
    recursive_function(&mut number, 5);
    println!("Final Value: {}", number);
}
