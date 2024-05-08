8. Given a binary tree, implement a function that returns the maximum depth of the tree.

-->


struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }

    fn max_depth(&self) -> i32 {
        match (self.left.as_ref(), self.right.as_ref()) {
            (Some(left), Some(right)) => 1 + std::cmp::max(left.max_depth(), right.max_depth()),
            (Some(left), None) => 1 + left.max_depth(),
            (None, Some(right)) => 1 + right.max_depth(),
            (None, None) => 1,
        }
    }
}

fn main() {
    // Constructing a sample binary tree
    let root = TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode::new(2))),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode::new(4))),
            right: None,
        })),
    };

    let depth = root.max_depth();
    println!("The maximum depth of the binary tree is {}.", depth);
}