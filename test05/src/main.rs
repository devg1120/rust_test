extern crate typed_arena;

use std::cell::RefCell;
use typed_arena::Arena;

struct NodeData<'a> {
    references: RefCell<Vec<Node<'a>>>
}
type Node<'a> = &'a NodeData<'a>;

fn main() {
    let nodes = Arena::new(); // mut は不要
    let node0 = nodes.alloc(NodeData { references: RefCell::new(vec![]) });
    node0.references.borrow_mut().push(node0);
    let node1 = nodes.alloc(NodeData { references: RefCell::new(vec![]) });
    node0.references.borrow_mut().push(node1);
}
