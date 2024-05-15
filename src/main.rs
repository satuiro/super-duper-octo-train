use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

struct Block {
    index: u32,
    timestamp: i64,
    transactions: Vec<Transaction>,
    prev_hash: String,
    hash: String,
    nonce: u32, // Added nonce field
}

struct Blockchain {
    blocks: Vec<Block>,
}

struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

impl Block {
    fn new_block(index: u32, transactions: Vec<Transaction>, prev_hash: &str, difficulty: u8) -> Block {
        let timestamp = Utc::now().timestamp();
        let transaction_data: Vec<String> = transactions
            .iter()
            .map(|tx| format!("{}-{}-{}", tx.sender, tx.receiver, tx.amount))
            .collect();
        let data = transaction_data.join(";");

        let mut nonce = 0;
        let mut hash = calculate_hash(index, timestamp, &data, prev_hash, nonce);
        while !hash.starts_with(&"0".repeat(difficulty as usize)) {
            nonce += 1;
            hash = calculate_hash(index, timestamp, &data, prev_hash, nonce);
        }

        Block {
            index,
            timestamp,
            transactions,
            prev_hash: prev_hash.to_string(),
            hash,
            nonce, // Store the nonce value used during mining
        }
    }
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp(),
            transactions: vec![], // An empty vector of transactions for the genesis block
            prev_hash: String::from("0"),
            hash: calculate_hash(0, Utc::now().timestamp(), "", "0", 0),
            nonce: 0, // Nonce value for the genesis block
        };

        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    fn add_block(&mut self, transactions: Vec<Transaction>, difficulty: u8) {
        let prev_hash = self.blocks.last().unwrap().hash.clone();
        let index = self.blocks.len() as u32;
        let new_block = Block::new_block(index, transactions, &prev_hash, difficulty);

        self.blocks.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let curr_block = &self.blocks[i];
            let prev_block = &self.blocks[i - 1];

            // Check if the current block's prev_hash matches the previous block's hash
            if curr_block.prev_hash != prev_block.hash {
                return false;
            }

            // Check if the current block's hash is correct
            let transaction_data: Vec<String> = curr_block
                .transactions
                .iter()
                .map(|tx| format!("{}-{}-{}", tx.sender, tx.receiver, tx.amount))
                .collect();
            let data = transaction_data.join(";");

            let calculated_hash = calculate_hash(
                curr_block.index,
                curr_block.timestamp,
                &data,
                &curr_block.prev_hash,
                curr_block.nonce, // Use the stored nonce value
            );
            if curr_block.hash != calculated_hash {
                return false;
            }
        }

        true
    }
}

fn calculate_hash(index: u32, timestamp: i64, data: &str, prev_hash: &str, nonce: u32) -> String {
    let mut hasher = Sha256::new();

    hasher.input_str(&index.to_string());
    hasher.input_str(&timestamp.to_string());
    hasher.input_str(data);
    hasher.input_str(prev_hash);
    hasher.input_str(&nonce.to_string());

    hasher.result_str()
}

fn main() {
    let mut blockchain = Blockchain::new();
    println!("Is blockchain valid? {}", blockchain.is_valid()); // true

    let tx1 = Transaction {
        sender: "Alice".to_string(),
        receiver: "Bob".to_string(),
        amount: 10.0,
    };
    let tx2 = Transaction {
        sender: "Bob".to_string(),
        receiver: "Charlie".to_string(),
        amount: 5.0,
    };

    blockchain.add_block(vec![tx1, tx2], 2); // Mining difficulty of 2 (leading zeros)

    println!("Is blockchain valid? {}", blockchain.is_valid()); // true

    // Tamper with the second block's data
    blockchain.blocks[1].transactions[0].amount = 100.0; // Modifying the transaction amount

    println!("Is blockchain valid? {}", blockchain.is_valid()); // false
}