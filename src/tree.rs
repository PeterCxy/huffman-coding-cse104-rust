use std::cmp::{PartialOrd, Ord, Ordering};

/*
 * Simple naive implementation of a binary tree
 * Since Huffman coding does not involve visiting the parent node
 * we don't have any reference to parent node in this implementation
 */
#[derive(PartialEq, Eq)]
pub struct Root {
    pub left: Box<TreeNode>,
    pub right: Box<TreeNode>
}

#[derive(PartialEq, Eq)]
pub struct Leaf {
    pub value: u8,
    pub freq: u64
}

#[derive(PartialEq, Eq)]
pub struct Node {
    pub freq: u64,
    pub left: Box<TreeNode>,
    pub right: Box<TreeNode>
}

#[derive(PartialEq, Eq)]
pub enum TreeNode {
    Root(Root),
    Leaf(Leaf),
    Node(Node)
}

impl TreeNode {
    // For sorting
    pub fn freq(&self) -> u64 {
        match *self {
            TreeNode::Root(ref _root) => <u64>::max_value(),
            TreeNode::Leaf(ref leaf) => leaf.freq,
            TreeNode::Node(ref node) => node.freq
        }
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &TreeNode) -> Option<Ordering> {
        self.freq().partial_cmp(&other.freq())
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &TreeNode) -> Ordering {
        self.freq().cmp(&other.freq())
    }
}

/*
 * Convenience trait to visit left and right childs
 * for Node and Root
 */
pub trait TraversableNode {
    fn left(&self) -> &TreeNode;
    fn right(&self) -> &TreeNode;
}

impl TraversableNode for Node {
    fn left(&self) -> &TreeNode {
        &self.left
    }

    fn right(&self) -> &TreeNode {
        &self.right
    }
}

impl TraversableNode for Root {
    fn left(&self) -> &TreeNode {
        &self.left
    }

    fn right(&self) -> &TreeNode {
        &self.right
    }
}
