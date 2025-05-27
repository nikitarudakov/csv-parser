use std::io::{Read};
use std::hash::{Hash, Hasher, DefaultHasher};
use std::time::{SystemTime};

struct Block {
    hash: String,
    prev_hash: String,
    timestamp: i64,
    data: dyn Read,
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {}
}

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        Self {
            blocks: Vec::new(),
        }
    }

    fn push_block(&mut self, data: &mut dyn Read) {
        let last_block = &self.blocks[self.blocks.len() - 1];
        let mut hasher = DefaultHasher::new();
        last_block.hash.hash(&hasher);

        let block = Block {
            hash: "".to_string(),
            prev_hash: hasher.finish().to_string(),
            timestamp: SystemTime::now(),
            data,
        };

        self.blocks.push(block);
    }
}
