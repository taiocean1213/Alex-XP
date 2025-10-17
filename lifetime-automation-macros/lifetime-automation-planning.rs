// Input struct with mutable fields
struct InputData {
    id: i32,
    name: String,
    value: f64,
}

// Output struct with immutable fields
struct OutputData {
    id: i32,
    name: String,
    value: f64,
}

// Function to convert mutable input reference to immutable output
fn convert_to_immutable(input: &mut InputData) -> &OutputData {
    // Create a new OutputData instance using the input's data
    let output = OutputData {
        id: input.id,
        name: input.name.clone(), // Clone the String to avoid moving
        value: input.value,
    };
    
    // Return a reference to the new OutputData
    // Note: In a real-world scenario, you'd need to manage the lifetime of output.
    // Here, we use a leaked reference for simplicity, but this is not recommended for production.
    Box::leak(Box::new(output))
}

fn main() {
    // Create a mutable instance of InputData
    let mut input = InputData {
        id: 42,
        name: String::from("Test"),
        value: 3.14,
    };
    
    // Convert to immutable OutputData reference
    let output = convert_to_immutable(&mut input);
    
    // Use the immutable output
    println!("Output: id={}, name={}, value={}", output.id, output.name, output.value);
    
    // Demonstrate that input can still be modified
    input.value = 2.718;
    println!("Modified input: value={}", input.value);
}