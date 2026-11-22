//! Nested prev-boxes: `drill_mut!` / `drill_read!` hide the while-let walk.

use alex_xp::{drill_mut, drill_read};

#[derive(Clone)]
enum NodeData {
    Value(i32),
    Prev(Box<Node>),
}

#[derive(Clone)]
struct Node {
    data: NodeData,
}

impl Node {
    fn new_value(value: i32) -> Self {
        Node {
            data: NodeData::Value(value),
        }
    }

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

    let mut node = match prev {
        Some(p) => Node::new_prev(p),
        None => Node::new_value(curr_depth),
    };

    let add_val = curr_depth;
    let current = drill_mut!(&mut node.data, NodeData::Prev, data);
    if let NodeData::Value(val) = current {
        *current = NodeData::Value(*val + add_val);
    }

    recurse(max_depth, curr_depth + 1, Some(node))
}

fn modify_lowest_value(deepest: &mut Option<Node>, new_value: i32) {
    if let Some(node) = deepest {
        let current = drill_mut!(&mut node.data, NodeData::Prev, data);
        if let NodeData::Value(_) = current {
            *current = NodeData::Value(new_value);
        }
    }
}

fn get_lowest_value(deepest: &Option<Node>) -> Option<i32> {
    deepest.as_ref().and_then(|node| {
        match drill_read!(&node.data, NodeData::Prev, data) {
            NodeData::Value(val) => Some(*val),
            _ => None,
        }
    })
}

pub fn run() {
    let max_depth = 5;
    let mut deepest = recurse(max_depth, 1, None);

    assert_eq!(get_lowest_value(&deepest), Some(16));
    if let Some(value) = get_lowest_value(&deepest) {
        println!("Initial lowest child's value: {value}");
    }

    modify_lowest_value(&mut deepest, 100);

    assert_eq!(get_lowest_value(&deepest), Some(100));
    if let Some(value) = get_lowest_value(&deepest) {
        println!("Updated lowest child's value: {value}");
    }
}

#[allow(dead_code)]
fn main() {
    run();
}
