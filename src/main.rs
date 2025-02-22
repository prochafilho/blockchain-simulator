use sha2::{Digest, Sha256};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;
use chrono::{NaiveDateTime, DateTime, Utc, TimeZone};

const DIFFICULTY: usize = 2;

struct Block {
    index: u32,
    previous_hash: String,
    timestamp: u64,
    data: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            nonce: 0,
            hash: String::new(),
        }
    }

    fn calculate_hash(&mut self) -> String {
        let data: String = format!("{}{}{}{}{}",
            self.index,
            self.previous_hash,
            self.timestamp,
            self.data,
            self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        let hash_str: String = format!("{:x}", result);
        hash_str
    }

    fn mine_block(&mut self) {
        let mut iterations: i32 = 0;
        loop {
            self.hash = self.calculate_hash();
            iterations += 1;
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == &"0".repeat(DIFFICULTY) {
                println!("⛏️ Block mined: {}", self.index);
                break;
            }
            if iterations > 100 {
                println!("⏳ Mining in process...");
                thread::sleep(Duration::from_millis(300));
                println!("Calculated hash... {}", self.hash);
            }
            self.nonce += 1;
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let datetime: NaiveDateTime = NaiveDateTime::from_timestamp_opt(self.timestamp as i64, 0).expect("Invalid timestamp");
        write!(f, "Block {}: {} at {}", self.index, self.data, datetime)
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block: Block = Block::new(0, String::new(), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, mut new_block: Block) {
        let previous_hash: String = self.chain.last().unwrap().hash.clone();
        new_block.previous_hash = previous_hash;
        new_block.mine_block();
        self.chain.push(new_block);
    }

    fn get_total_blocks(&self) -> usize {
        self.chain.len()
    }
}

fn main() {
    println!("🚀 Welcome to LearnCoin Mining Simulator! 🚀");
    println!("🙍‍♂️ Enter your miner name: ");

    let mut miner_name: String = String::new();

    std::io::stdin().read_line(&mut miner_name).expect("Failed to read name");

    miner_name = miner_name.trim().to_string();

    let trader_names: Vec<String> = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
        String::from("David"),
        String::from("Eve"),
        String::from("Frank"),
        String::from("Grace"),
        String::from("Heidi"),
        String::from("Ivan"),
        String::from("Judy")
    ];

    let mut learncoin: Blockchain = Blockchain::new();

    println!("⛏️ Let's start mining and simulate transactions!\n");

    let mut sender: String = miner_name.clone();

    for i in 0..trader_names.len() {
        println!("🧱 Mining block {}...⛏️", i + 1);
        let recipient: String = if i < trader_names.len() - 1 {
            trader_names[i + 1].to_string()
        } else {
            trader_names[0].clone()
        };

        let transaction: String = format!("{} sent to {}", sender, recipient);
        let new_block: Block = Block::new((i + 1) as u32, String::new(), transaction.clone());

        learncoin.add_block(new_block);

        println!("🎉 Transaction successful: {}", transaction);

        sender = recipient;

        println!();
    }

    let total_blocks: usize = learncoin.get_total_blocks();

    println!("📚 Total blocks added to the blockchain: {}", total_blocks);

    let learncoin_per_block: usize = 137;
    let learncoin_traded: usize = total_blocks * learncoin_per_block;

    println!("💰 Total LearnCoins traded: {}", learncoin_traded);

    let end_timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    let end_datetime: DateTime<Utc> = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(end_timestamp as i64, 0).expect("Invalid timestamp"));

    println!("⏰ Simulation ended at: {}", end_datetime);
    println!("👋 Thank you for using LearnCoin Mining Simulator!");
}