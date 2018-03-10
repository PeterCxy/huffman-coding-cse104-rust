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

/*
 * Construct a dictionary based on a huffman tree
 */
fn traverse_huffman_tree(tree: &TreeNode) -> HashMap<Vec<bool>, u8> {
    let mut ret = HashMap::new();
    for (path, val) in TreeIter::new(tree) {
        ret.insert(path, val);
    }
    return ret;
}

fn _print_huffman_dict(dict: HashMap<Vec<bool>, u8>) {
    for (k, v) in &dict {
        println!("{}: {}", k.to_binary(), v);
    }
}

fn main() {
    let table = build_frequency_table(b"A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED");
    let tree = build_huffman_tree(table.clone());
    let dict = traverse_huffman_tree(&tree);
    _print_huffman_dict(dict);
}