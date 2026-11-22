//! Mutable input → **owned** output. No leaked `'static` references.

struct InputData {
    value: i32,
    numbers: Vec<f64>,
    text: String,
}

struct OutputData<T> {
    processed_value: i32,
    processed_numbers: Vec<f64>,
    result: T,
}

fn process_data_recursive<T>(input: &mut InputData) -> OutputData<T>
where
    T: Default + std::ops::Add<Output = T> + From<i32>,
{
    if input.value <= 0 {
        input.value += 10;
        input.numbers.push(input.value as f64);
        input.text.push_str("_processed");

        OutputData {
            processed_value: input.value * 2,
            processed_numbers: input.numbers.iter().map(|&x| x * 1.5).collect(),
            result: T::default(),
        }
    } else {
        let original_value = input.value;

        input.value += 10;
        input.numbers.push(input.value as f64);
        input.text.push_str("_processed");

        let mut recursive_input = InputData {
            value: original_value - 1,
            numbers: input.numbers.clone(),
            text: input.text.clone(),
        };

        let recursive_output: OutputData<T> = process_data_recursive(&mut recursive_input);

        OutputData {
            processed_value: input.value * 2,
            processed_numbers: input.numbers.iter().map(|&x| x * 1.5).collect(),
            result: recursive_output.result + T::from(original_value),
        }
    }
}

fn main() {
    let mut input = InputData {
        value: 3,
        numbers: vec![1.0, 2.0, 3.0],
        text: String::from("hello"),
    };

    let output: OutputData<i32> = process_data_recursive(&mut input);

    println!(
        "Input after recursion: value={}, numbers={:?}, text={}",
        input.value, input.numbers, input.text
    );
    println!(
        "Output: value={}, numbers={:?}, result={:?}",
        output.processed_value, output.processed_numbers, output.result
    );
}
