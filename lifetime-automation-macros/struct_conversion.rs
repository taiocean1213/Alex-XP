// Input struct with mutable fields
struct InputData {
    value: i32,
    numbers: Vec<f64>,
    text: String,
}

// Output struct with immutable fields
struct OutputData<T> {
    processed_value: i32,
    processed_numbers: Vec<f64>,
    result: T, // Generic type for flexible output
}

// Function that takes a mutable reference to InputData and returns an immutable OutputData
fn process_data<T: Default>(input: &mut InputData) -> &'static OutputData<T> {
    // Modify input struct contents
    input.value += 10;
    input.numbers.push(input.value as f64);
    input.text.push_str("_processed");

    // Create output struct
    let output = OutputData {
        processed_value: input.value * 2,
        processed_numbers: input.numbers.iter().map(|&x| x * 1.5).collect(),
        result: T::default(),
    };

    // Return immutable reference to output
    // Note: For simplicity, we're leaking memory here to get a static reference
    // In a real application, you'd want to handle lifetimes properly
    Box::leak(Box::new(output))
}

// Example usage
fn main() {
    // Create mutable input
    let mut input = InputData {
        value: 42,
        numbers: vec![1.0, 2.0, 3.0],
        text: String::from("hello"),
    };

    // Process data with String as the generic type
    let output: &OutputData<String> = process_data(&mut input);

    // Print results
    println!("Input after modification: value={}, numbers={:?}, text={}",
        input.value, input.numbers, input.text);
    println!("Output: value={}, numbers={:?}, result={:?}",
        output.processed_value, output.processed_numbers, output.result);
}