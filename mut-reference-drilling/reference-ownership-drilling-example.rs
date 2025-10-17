// Enum to represent either a value or a previous node
#[derive(Clone)]
enum NodeData {
    Value(i32),
    Prev(Box<Node>),
}

// Node structure
#[derive(Clone)]
struct Node {
    data: NodeData,
}

impl Node {
    // Create a new node with a value
    fn new_value(value: i32) -> Self {
        Node {
            data: NodeData::Value(value),
        }
    }

    // Create a new node with a previous node
    fn new_prev(prev: Node) -> Self {
        Node {
            data: NodeData::Prev(Box::new(prev)),
        }
    }
}

fn recurse(max_depth: i32, curr_depth: i32, prev: Option<Node>) -> Option<Node> {
    if curr_depth > max_depth {
        return prev;
    }

    // Create a new mutable node
    let mut node = match prev {
        Some(p) => Node::new_prev(p),
        None => Node::new_value(curr_depth),
    };

    // Modify the value by drilling down to the deepest node
    let mut current = &mut node.data;
    let add_val = curr_depth;
    while let NodeData::Prev(prev) = current {
        current = &mut prev.data;
    }
    if let NodeData::Value(val) = current {
        *current = NodeData::Value(*val + add_val); // Increment the value
    }

    // Recurse to the next depth
    recurse(max_depth, curr_depth + 1, Some(node))
}

// Function to modify the lowest child's value
fn modify_lowest_value(deepest: &mut Option<Node>, new_value: i32) {
    if let Some(node) = deepest {
        let mut current = &mut node.data;
        while let NodeData::Prev(prev) = current {
            current = &mut prev.data;
        }
        if let NodeData::Value(_) = current {
            *current = NodeData::Value(new_value); // Update the lowest child's value
        }
    }
}

// Function to return the lowest child's value
fn get_lowest_value(deepest: &Option<Node>) -> Option<i32> {
    let mut current = deepest.as_ref().map(|node| &node.data);
    while let Some(NodeData::Prev(prev)) = current {
        current = Some(&prev.data);
    }
    match current {
        Some(NodeData::Value(val)) => Some(*val),
        _ => None,
    }
}

fn main() {
    let max_depth = 5;
    let mut deepest = recurse(max_depth, 1, None);

    // Print initial values
    let mut current = deepest.clone(); // Use clone since Node is now Clone
    let mut depth = max_depth;
    println!("Initial values:");
    while let Some(node) = current {
        match node.data {
            NodeData::Value(val) => {
                println!("Depth {}: Value {}", depth, val);
                break;
            }
            NodeData::Prev(prev) => {
                println!("Depth {}: Links to previous", depth);
                current = Some(*prev);
            }
        }
        depth -= 1;
    }

    // Get and print the initial lowest child's value
    if let Some(value) = get_lowest_value(&deepest) {
        println!("\nInitial lowest child's value: {}", value);
    }

    // Modify the lowest child's value to 100
    modify_lowest_value(&mut deepest, 100);

    // Print updated values
    let mut current = deepest.clone(); // Use clone since Node is now Clone
    let mut depth = max_depth;
    println!("\nUpdated values:");
    while let Some(node) = current {
        match node.data {
            NodeData::Value(val) => {
                println!("Depth {}: Value {}", depth, val);
                break;
            }
            NodeData::Prev(prev) => {
                println!("Depth {}: Links to previous", depth);
                current = Some(*prev);
            }
        }
        depth -= 1;
    }

    // Get and print the updated lowest child's value
    if let Some(value) = get_lowest_value(&deepest) {
        println!("\nUpdated lowest child's value: {}", value);
    }
}
