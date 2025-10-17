// Input struct with mutable fields, now recursive with an optional child
struct InputData {
    value: i32,
    numbers: Vec<f64>,
    text: String,
    child: Option<Box<InputData>>, // Recursive field using Box for indirection
}

// Output struct with immutable fields, also recursive
struct OutputData<T> {
    processed_value: i32,
    processed_numbers: Vec<f64>,
    result: T,
    child_output: Option<Box<OutputData<T>>>, // Recursive output
}

// Recursive function that takes a mutable reference to InputData and returns an owned OutputData
fn process_data<T: Default>(input: &mut InputData) -> OutputData<T> {
    // Modify current node's contents
    input.value += 10;
    input.numbers.push(input.value as f64);
    input.text.push_str("_processed");

    // Process child recursively if it exists
    let child_output = if let Some(child) = &mut input.child {
        Some(Box::new(process_data(child)))
    } else {
        None
    };

    // Create and return owned output struct for this node
    OutputData {
        processed_value: input.value * 2,
        processed_numbers: input.numbers.iter().map(|&x| x * 1.5).collect(),
        result: T::default(),
        child_output,
    }
}

// Example usage
fn main() {
    // Create mutable input with a child for recursion
    let mut child_input = InputData {
        value: 10,
        numbers: vec![0.5, 1.0],
        text: String::from("child"),
        child: None,
    };

    let mut input = InputData {
        value: 42,
        numbers: vec![1.0, 2.0, 3.0],
        text: String::from("parent"),
        child: Some(Box::new(child_input)),
    };

    // Process data recursively with String as the generic type
    let output: OutputData<String> = process_data(&mut input);

    // Print results (you can add recursive printing if needed)
    println!("Input after modification: value={}, numbers={:?}, text={}",
        input.value, input.numbers, input.text);
    println!("Output: value={}, numbers={:?}, result={:?}",
        output.processed_value, output.processed_numbers, output.result);
}