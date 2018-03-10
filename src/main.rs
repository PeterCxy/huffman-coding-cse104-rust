extern crate base64;
extern crate bit_vec;

mod bitvec;
mod tree;
mod traversal;

use bit_vec::BitVec;
use bitvec::*;
use std::collections::HashMap;
use std::str;
use tree::*;
use traversal::*;

/*
 * Build a frequency table from some data
 */
fn build_frequency_table(data: &[u8]) -> HashMap<u8, u32> {
    let mut freq_table: HashMap<u8, u32> = HashMap::new();
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
fn build_huffman_tree(mut freq_table: HashMap<u8, u32>) -> TreeNode {
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
fn traverse_huffman_tree(tree: &TreeNode) -> HashMap<u8, BitVec<u32>> {
    let mut ret = HashMap::new();
    for (path, val) in TreeIter::new(tree) {
        ret.insert(val, path);
    }
    return ret;
}

fn _print_huffman_dict(dict: HashMap<u8, BitVec<u32>>) {
    for (k, v) in &dict {
        println!("{}: {}", v.to_binary(), k);
    }
}

/*
 * Encode some data by huffman coding
 * returns the frequency table being used, the encoded data and the padding value
 * padding value: how many zeros are added to the tail to align with the
 * memory cells, which should be truncated when decoding
 */
fn huffman_encode(data: &[u8]) -> (HashMap<u8, u32>, Vec<u8>, u8) {
    let freq_table = build_frequency_table(data);
    let tree = build_huffman_tree(freq_table.clone());
    let dict = traverse_huffman_tree(&tree);

    let mut ret: BitVec<u32> = BitVec::new();

    for item in data {
        ret.append_all(dict.get(item).unwrap().clone());
    }

    return (freq_table, ret.to_bytes(), 8 - (ret.len() % 8) as u8);
}

/*
 * Decode some given data encoded with huffman coding
 * with the frequency table, the data and the padding value
 */
fn huffman_decode(freq_table: HashMap<u8, u32>, data: &[u8], padding: u8) -> Vec<u8> {
    let tree = build_huffman_tree(freq_table);
    let mut huffman_vec = BitVec::from_bytes(data);
    let unpad_len = huffman_vec.len() - padding as usize + 1;
    huffman_vec.truncate(unpad_len); // Remove the padding

    // Traverse the tree while we read the data bit-by-bit
    let mut current_node = &tree;
    let mut ret: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < huffman_vec.len() {
        let cur_value = huffman_vec.get(i).unwrap();
        i += 1;
        match *current_node {
            TreeNode::Leaf(ref leaf) => {
                // If the current node is a leaf, it means that
                // we have already found a complete value
                // Add it to the output queue and continue
                ret.push(leaf.value);
                current_node = &tree;
                i -= 1;
            },

            // For root and node, we go left for 0 and go right for 1.
            TreeNode::Root(ref root) => current_node = _huffman_decode_step(cur_value, root),
            TreeNode::Node(ref node) => current_node = _huffman_decode_step(cur_value, node)
        }
    }

    return ret;
}

fn _huffman_decode_step(cur_value: bool, node: &TraversableNode) -> &TreeNode {
    if cur_value {
        node.right()
    } else {
        node.left()
    }
}

fn main() {
    let data = b"A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED";
    println!("original len = {}", data.len());
    let (freq_table, encoded_data, padding) = huffman_encode(data);
    println!("encoded: padding = {}, len = {}, data = {}", padding, encoded_data.len(), base64::encode(&encoded_data));
    let decoded_data = huffman_decode(freq_table, &encoded_data, padding);
    println!("decoded: len = {}, data = {}", decoded_data.len(), str::from_utf8(&decoded_data).unwrap());
}