use std::cmp::max;

/*
* We're building a binary tree that we can add search for the largest element
* (Not terribly useful by itself)
*/
struct Node {
    value: u32,

    // The children should be some variant of Node.
    // Think: Why shouldn't we use a reference? How do we model nodes without children?
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}
impl Node {
    /*
    * Get the largest node below you in the tree, recursively
    */
    pub fn largest_child(&self) -> u32 {
        let left_val = match &self.left_child {
            Some(node) => node.largest_child(),
            None => 0
        };
        let right_val = match &self.right_child {
            Some(node) => node.largest_child(),
            None => 0
        };

        max(self.value, max(left_val, right_val))
    }

    /*
    * Create a new Node, and populate it and its children with the vector in argument.
    */
    pub fn new(mut children: Vec<u32>) -> Option<Self> {
        // Get own value, return None if there isn't one
        let my_value = children.pop()?;
        // Split the array in half through the middle
        let other_children = children.split_off(children.len()/2);

        Some(Self {
            // Take the last value, removing it from the vector
            // Unwrap is safe because we guarantee we'll never pass
            value: my_value,
            left_child: Self::new(children).map(|n| Box::new(n)),
            right_child: Self::new(other_children).map(|n| Box::new(n)),
        })
    }
}

fn main() {
    let tree = Node::new(vec![]);
    let tree2 = Node::new(vec![1,3,4,3,5,0,1]).unwrap();
    let tree3 = Node::new(vec![1,3,4,3,5,0,6]).unwrap();
    let tree4 = Node::new(vec![6,3,4,3,5,0,1]).unwrap();

    assert!(tree.is_none());
    assert_eq!(tree2.largest_child(), 5);
    assert_eq!(tree3.largest_child(), 6);
    assert_eq!(tree4.largest_child(), 6);
}
