#![allow(unused)]

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
struct TrieNode {
    nodes: Vec<Option<Box<TrieNode>>>,
    is_final: bool,
    data: Option<String>,
}

impl TrieNode {
    pub fn new() -> Self {
        let is_final = false;
        let nodes = vec![None; 74];
        let data = None;

        Self { nodes, is_final, data }
    }

    pub fn assert_is_final(&self) -> bool {
        self.is_final
    }

    pub fn set_node_at_index(&mut self, index: usize, node: Option<Box<TrieNode>>) {
        self.nodes[index - 48] = node;
    }

    pub fn get_node_at_index(&self, index: usize) -> Option<Box<TrieNode>> {
        self.nodes.get(index - 48).unwrap().clone()
    }

    pub fn assert_contains(&self, index: usize) -> bool {
        self.nodes[index - 48].is_some()
    }

    pub fn set_final(&mut self) {
        self.is_final = true;
    }
    
    pub fn get_data(&self) -> Option<String> {
        self.data.clone()
    }

    pub fn set_data(&mut self, data: Option<String>) {
        self.data = data;
    }
}

#[pyclass]
#[derive(Clone)]
struct PhyTrie {
    root: TrieNode
}

impl PhyTrie {
    pub fn new() -> Self {
        let root = TrieNode::new();

        Self { root }
    }

    pub fn insert(&mut self, key: String, data: String) {
        let mut node = self.root.clone();

        for (i, ch) in key.char_indices() {
            let index = ch.to_digit(10).unwrap();
        
            if !node.assert_contains(index as usize) {
                node.set_node_at_index(
                    index as usize, 
                    Some(Box::new(TrieNode::new()))
                )
            }

            node = *(node.get_node_at_index(index as usize).unwrap());
        }

        node.set_final();
        node.set_data(Some(data));

        self.root = node;
    }

    pub fn get(&self, key: String) -> Option<String> {
        let mut node = self.root.clone();

        for (i, ch) in key.char_indices() {
            let index = ch.to_digit(10).unwrap();
            
            if node.assert_contains(index as usize) {
                node = *(node.get_node_at_index(index as usize).unwrap());
            } else {
                return None;
            }
        }

        node.get_data()
    }

}

/// A Python module implemented in Rust.
#[pymodule]
fn phymmr_trie(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PhyTrie>()?;
    Ok(())
}