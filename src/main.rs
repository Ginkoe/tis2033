mod node;
mod instructions;

use node::Node;

use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("roms/test.t33").unwrap();
    let mut main_node = Node::new();
    main_node.load(contents);
    main_node.run();
}