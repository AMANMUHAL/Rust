// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Example binary tree:
    //      3
    //     / \
    //    9  20
    //      /  \
    //     15   7
    let mut root = TreeNode::new(3);
    let mut node9 = TreeNode::new(9);
    let mut node20 = TreeNode::new(20);
    let mut node15 = TreeNode::new(15);
    let mut node7 = TreeNode::new(7);

    node20.left = Some(Box::new(node15));
    node20.right = Some(Box::new(node7));
    root.left = Some(Box::new(node9));
    root.right = Some(Box::new(node20));

    println!("Maximum depth of the tree: {}", max_depth(Some(Box::new(root))));
}
