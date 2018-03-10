use bitvec::*;
use tree::*;

/*
 * A helper to traverse a binary tree
 * this can convert recursion into loops
 * with a self-maintained stack
 */
pub struct TreeTraverser<'a> {
    current_node: &'a TreeNode,
    path: Vec<bool>,
    left_visited: bool,
    finished: bool
}

impl<'a> TreeTraverser<'a> {
    pub fn new(current_node: &'a TreeNode, path: Vec<bool>) -> TreeTraverser<'a> {
        TreeTraverser {
            current_node,
            path,
            left_visited: false,
            finished: false
        }
    }

    /*
     * Visit the next node on this tree
     * return None if no more nodes can be visited
     * return the leaf value and the path to the leaf if we are currently on a leaf
     * otherwise, return a new TreeTraverser starting from the next traversable node
     */
    pub fn visit<'b>(&'b mut self) -> Option<Result<(Vec<bool>, u8), TreeTraverser<'a>>> {
        match *self.current_node {
            TreeNode::Leaf(ref leaf) => self.visit_leaf(leaf),
            TreeNode::Node(ref node) => self.visit_node(node),
            TreeNode::Root(ref root) => self.visit_node(root)
        }
    }

    fn visit_leaf<'b>(&'b mut self, leaf: &'a Leaf) -> Option<Result<(Vec<bool>, u8), TreeTraverser<'a>>> {
        if !self.finished {
            self.finished = true;
            Some(Ok((self.path.clone(), leaf.value)))
        } else {
            None
        }
    }

    fn visit_node<'b>(&'b mut self, node: &'a TraversableNode) -> Option<Result<(Vec<bool>, u8), TreeTraverser<'a>>> {
        if self.finished {
            return None;
        }

        if !self.left_visited {
            self.left_visited = true;
            Some(Err(TreeTraverser::new(node.left(), self.path.copy_append(true))))
        } else {
            self.finished = true;
            Some(Err(TreeTraverser::new(node.right(), self.path.copy_append(false))))
        }
    }
}

/*
 * An iterator to traverse a binary tree
 */
pub struct TreeIter<'a> {
    traversers: Vec<TreeTraverser<'a>>
}

impl<'a> TreeIter<'a> {
    pub fn new(node: &'a TreeNode) -> TreeIter<'a> {
        TreeIter {
            traversers: vec![TreeTraverser::new(node, Vec::new())]
        }
    }
}

impl<'a> Iterator for TreeIter<'a> {
    type Item = (Vec<bool>, u8);

    fn next(&mut self) -> Option<Self::Item> {
        while self.traversers.len() > 0 {
            let len = self.traversers.len();
            if let Some(result) = self.traversers[len - 1].visit() {
                match result {
                    Ok(item) => return Some(item),
                    Err(traverser) => self.traversers.push(traverser)
                }
            } else {
                self.traversers.remove(len - 1);
            }
        }

        return None;
    }
}
