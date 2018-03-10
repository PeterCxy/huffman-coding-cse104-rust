extern crate base64;
extern crate bit_vec;
extern crate byteorder;

mod bitvec;
mod tree;
mod traversal;

use bit_vec::BitVec;
use bitvec::*;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use std::collections::HashMap;
use std::io::Cursor;
use std::str;
use tree::*;
use traversal::*;

fn u32_to_bytes(value: u32) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(4);
    ret.write_u32::<LittleEndian>(value).unwrap();
    return ret;
}

fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let mut v = Vec::with_capacity(4);
    v.resize(4, 0);
    v.clone_from_slice(bytes);
    let mut cursor = Cursor::new(v);
    cursor.read_u32::<LittleEndian>().unwrap()
}

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

fn frequency_table_to_bytes(table: &HashMap<u8, u32>) -> Vec<u8> {
    let mut ret = Vec::with_capacity(table.len() * 5);

    for (&k, &v) in table {
        ret.push(k);
        ret.append(&mut u32_to_bytes(v));
    }

    return ret;
}

fn bytes_to_frequency_table(bytes: &[u8]) -> HashMap<u8, u32> {
    let mut ret = HashMap::new();
    let mut i = 0;
    while i < bytes.len() {
        let k = bytes[i];
        let v = bytes_to_u32(&bytes[(i + 1)..(i + 5)]);
        ret.insert(k, v);
        i += 5;
    }

    return ret;
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

/*
 * Encode some data with huffman coding
 * and serialize the result as bytes
 * Layout:
 * | Freq Table Length (1) | Padding Length (1) | Freq Table (variable) | Encoded Data (variable) |
 */
fn huffman_encode_to_bytes(data: &[u8]) -> Vec<u8> {
    let (freq_table, mut encoded_data, padding) = huffman_encode(data);
    let mut ret = Vec::new();

    if freq_table.len() > <u8>::max_value() as usize {
        panic!("Table impossibly large.");
    }

    ret.push(freq_table.len() as u8);
    ret.push(padding);
    ret.append(&mut frequency_table_to_bytes(&freq_table));
    ret.append(&mut encoded_data);
    return ret;
}

/*
 * Deserialize the result of some previous huffman coding
 * and decode it to the original data
 */
fn huffman_decode_from_bytes(data: &[u8]) -> Vec<u8> {
    let freq_table_len = (data[0] as usize) * 5;
    let padding = data[1];
    let freq_table = bytes_to_frequency_table(&data[2..(2 + freq_table_len)]);
    huffman_decode(freq_table, &data[(2 + freq_table_len)..], padding)
}

fn main() {
    let data = b"A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED";
    println!("original len = {}", data.len());
    /*let (freq_table, encoded_data, padding) = huffman_encode(data);
    println!("encoded: padding = {}, len = {}, data = {}", padding, encoded_data.len(), base64::encode(&encoded_data));
    let decoded_data = huffman_decode(freq_table, &encoded_data, padding);
    println!("decoded: len = {}, data = {}", decoded_data.len(), str::from_utf8(&decoded_data).unwrap());*/
    let encoded_data = huffman_encode_to_bytes(data);
    println!("encoded: len = {}, data = {}", encoded_data.len(), base64::encode(&encoded_data));
    let decoded_data = huffman_decode_from_bytes(&encoded_data);
    println!("decoded: len = {}, data = {}", decoded_data.len(), str::from_utf8(&decoded_data).unwrap());
}