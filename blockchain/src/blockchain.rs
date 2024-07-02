crate time; 
crate serde; 
crate serde_json;
crate sha2; 

use sha2::{Sha256, Digest}; 
use std::fmt::Write; 

#[derive(Debug, Clone, Serialize,)] 
struct Transaction {
    sender: String, 
    receiver: String, 
    amount: f32, 
}

#[derive(Debug, Clone, Serialize,)]
pub struct Blockheader {
    timestamp: u64, 
    nonce: u64, 
    prev_block_hash: String, 
    merkle_root: String, 
    difficulty: u32,
}