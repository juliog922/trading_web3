use log::info;
use serde_json::json;
use chrono::prelude::*;
use serde::{
    Deserialize,
    Serialize
};

use crate::transaction::Transaction;
use crate::util::Util;
use crate::wallet::Wallet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: usize,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub txn: Vec<Transaction>,
    pub validator: String,
    pub signature: String,
    pub difficulty: u32,
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.previous_hash == other.previous_hash
    }
}

impl Block {

    pub fn new(
        id: usize,
        previous_hash: String,
        timestamp: i64,
        txn: Vec<Transaction>,
        difficulty: u32,
        mut validator_wallet: Wallet
    ) -> Self {
        info!("Creatin block...");

        let hash = Block::calculate_hash(
            &id, 
            &timestamp, 
            &previous_hash, 
            &txn,
            &validator_wallet.get_public_key(),
            &difficulty
        );

        let signature = validator_wallet.sign(&hash);

        Self {
            id,
            hash,
            previous_hash,
            timestamp,
            txn,
            validator: validator_wallet.get_public_key(),
            signature: signature,
            difficulty: difficulty,
        }
    }
    
    pub fn calculate_hash(
        id: &usize,
        timestamp: &i64,
        previous_hash: &str,
        txn: &Vec<Transaction>,
        validator: &String,
        difficulty: &u32,
    ) -> String {

       info!("Calculating hash ...");

       let hash = json!({
        "id": id,
        "previous_hash": previous_hash,
        "transactions": txn,
        "timestamp": timestamp,
        "validator": validator,
        "difficulty": difficulty
       });

       Util::hash(&hash.to_string())
    }

    pub fn genesis() -> Self {
        info!("Creating Genesis block...");

        let previous_hash = String::from("genesis");
        let validator = previous_hash.clone();
        let signature = previous_hash.clone();
        let id = 0;
        let timestamp = Utc::now().timestamp();
        Self {
            id,
            hash: Block::calculate_hash(
                &0, 
                &timestamp, 
                &previous_hash, 
                &vec![],
                &validator,
                &5),
            previous_hash,
            timestamp,
            txn: vec![],
            validator,
            signature,
            difficulty: 5
        }
    }

    pub fn verify_block_signature(block: &Self) -> bool {
        info!("Verifyinf block ...");
        let hash = Block::calculate_hash(
            &block.id, 
            &block.timestamp, 
            &block.previous_hash, 
            &block.txn,
            &block.validator,
            &block.difficulty
        );
        Util::verify_signature(&block.validator, &hash, &block.signature).is_ok()
    }
}