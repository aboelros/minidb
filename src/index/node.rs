use crate::types::Value;
use crate::storage::page::RecordId;
use crate::storage::page::PageId;
use super::MAX_KEYS_PER_NODE;

#[derive(Debug, Clone)]
pub enum NodeType {
    Internal(InternalNode),
    Leaf(LeafNode),
}

#[derive(Debug, Clone)]
pub struct InternalNode {
    pub keys: Vec<Value>,
    pub children: Vec<PageId>, // length is keys.len() + 1
}

#[derive(Debug, Clone)]
pub struct LeafNode {
    pub keys: Vec<Value>,
    pub records: Vec<RecordId>, // length is keys.len()
    pub next_leaf: Option<PageId>,
}

impl InternalNode {
    pub fn new() -> Self {
        Self {
            keys: Vec::with_capacity(MAX_KEYS_PER_NODE),
            children: Vec::with_capacity(MAX_KEYS_PER_NODE + 1),
        }
    }
}

impl LeafNode {
    pub fn new() -> Self {
        Self {
            keys: Vec::with_capacity(MAX_KEYS_PER_NODE),
            records: Vec::with_capacity(MAX_KEYS_PER_NODE),
            next_leaf: None,
        }
    }
}
