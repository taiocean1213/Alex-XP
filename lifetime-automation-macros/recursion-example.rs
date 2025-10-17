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

// Recursive function that takes a mutable reference to InputData and returns an owned OutputData
// Base case: if value <= 0, return default output
// Recursive case: decrement value, recurse, and accumulate results
fn process_data_recursive<T: Default + std::ops::Add<Output = T> + std::convert::From<i32>>(input: &mut InputData) -> OutputData<T> {
    if input.value <= 0 {
        // Base case: still modify input
        input.value += 10;
        input.numbers.push(input.value as f64);
        input.text.push_str("_processed");

        OutputData {
            processed_value: input.value * 2,
            processed_numbers: input.numbers.iter().map(|&x| x * 1.5).collect(),
            result: T::default(),
        }
    } else {
        // Recursive case
        let original_value = input.value;

        // Modify current input
        input.value += 10;
        input.numbers.push(input.value as f64);
        input.text.push_str("_processed");

        // Create recursive input with decremented original value
        let mut recursive_input = InputData {
            value: original_value - 1, // Decrement based on original
            numbers: input.numbers.clone(),
            text: input.text.clone(),
        };

        // Recurse
        let recursive_output: OutputData<T> = process_data_recursive(&mut recursive_input);

        // Create current output and accumulate result (sum of original values)
        OutputData {
            processed_value: input.value * 2,
            processed_numbers: input.numbers.iter().map(|&x| x * 1.5).collect(),
            result: recursive_output.result + T::from(original_value),
        }
    }
}

// Example usage
fn main() {
    // Create mutable input
    let mut input = InputData {
        value: 3, // Small value to limit recursion depth
        numbers: vec![1.0, 2.0, 3.0],
        text: String::from("hello"),
    };

    // Process data recursively with i32 as the generic type
    let output: OutputData<i32> = process_data_recursive(&mut input);

    // Print results (input modified only at top level)
    println!("Input after recursion: value={}, numbers={:?}, text={}",
        input.value, input.numbers, input.text);
    println!("Output: value={}, numbers={:?}, result={:?}",
        output.processed_value, output.processed_numbers, output.result);
}