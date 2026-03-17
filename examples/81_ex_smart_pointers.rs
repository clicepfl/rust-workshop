use std::cmp::max;

/*
* We're building a binary tree that we can search for the largest element
* (Not terribly useful by itself)
*/
struct Node {
    value: u32,

    // Is this the right type? Why not use a reference? How do we model nodes without children?
    left_child: Node,
    right_child: Node,
}
impl Node {
    /*
    * Get the largest node below you in the tree, recursively
    * Return 0 by default
    */
    pub fn largest_child(&self) -> u32 {
        // Revisions on destructuring
        let left_val = match &self.left_child {
            ??? => ???.largest_child(),
            ??? => 0
        };
        let right_val = match &self.right_child {
            ??? => ???.largest_child(),
            ??? => 0
        };

        max(self.value, max(left_val, right_val))
    }

    /*
    * Create a new Node, and populate it and its children with the vector in argument.
    */
    pub fn new(mut children: Vec<u32>) -> Option<Self> {
        // Get own value; pop removes and returns the last value of the vec
        let my_value = children.pop()?;
        // Split the array in half through the middle
        let other_children = children.split_off(children.len()/2);

        Some(Self {
            value: my_value,
            left_child: ???,
            right_child: ???,
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
