mod bitvec;
mod tree;
mod traversal;

use bitvec::*;
use std::collections::HashMap;
use tree::*;
use traversal::*;

/*
 * Build a frequency table from some data
 */
fn build_frequency_table(data: &[u8]) -> HashMap<u8, u64> {
    let mut freq_table: HashMap<u8, u64> = HashMap::new();
    for i in data {
        let n = freq_table.get(i).map(|x| x.clone()).unwrap_or(0);
        freq_table.insert(i.clone(), n + 1);
    }
    return freq_table;
}

/*
 * Construct a huffman tree from a given frequency table
 * which can be later traversed to produce a huffman codebook
 */
fn build_huffman_tree(mut freq_table: HashMap<u8, u64>) -> TreeNode {
    let mut vec: Vec<TreeNode> = freq_table.drain()
        .map(|(value, freq)| TreeNode::Leaf(Leaf {
            value,
            freq
        }))
        .collect();
    vec.sort();

    while vec.len() > 2 {
        let left = vec.remove(0);
        let right = vec.remove(0);

        let new_node = TreeNode::Node(Node {
            freq: left.freq() + right.freq(),
            left: Box::new(left),
            right: Box::new(right)
        });

        match vec.binary_search(&new_node) {
            Ok(index) => vec.insert(index, new_node),
            Err(index) => vec.insert(index, new_node)
        }
    }

    return TreeNode::Root(Root {
        left: Box::new(vec.remove(0)),
        right: Box::new(vec.remove(0))
    });
}

fn traverse_huffman_tree(tree: &TreeNode) {
    for (path, val) in TreeIter::new(tree) {
        println!("{}: {}", path.to_binary(), val);
    }
}

// SHOULD BE REMOVED
fn _traverse_huffman_tree(tree: &TreeNode, path: &mut Vec<bool>) {
    match *tree {
        TreeNode::Leaf(ref leaf) => println!("{}: {} freq {}", path.to_binary(), leaf.value, leaf.freq),
        TreeNode::Node(ref node) => {
            path.push(true);
            _traverse_huffman_tree(&node.left, path);
            path.pop();
            path.push(false);
            _traverse_huffman_tree(&node.right, path);
            path.pop();
        },
        TreeNode::Root(ref root) => {
            path.push(true);
            _traverse_huffman_tree(&root.left, path);
            path.pop();
            path.push(false);
            _traverse_huffman_tree(&root.right, path);
            path.pop();
        }
    }
}

fn main() {
    let table = build_frequency_table(b"A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED");
    let tree = build_huffman_tree(table.clone());
    traverse_huffman_tree(&tree);
}
