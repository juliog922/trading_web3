use ed25519_compact::KeyPair;
use serde::{Deserialize, Serialize};
use hex::{
    encode,
    decode
};

use crate::blockchain::Blockchain;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub key_pair: String,
}

impl Wallet {
    pub fn new() -> Self {
        let keypair = KeyPair::generate();
        let pub_key = encode(keypair.pk.as_slice());
        println!("Your Public Key {}", pub_key);
        let keypair = encode(keypair.as_slice());
        println!("Your Key Pair {}", keypair);
        Self { key_pair: keypair }
    }

    pub fn generate_wallet() {
        let keypair = KeyPair::generate();
        let pub_key = encode(keypair.pk.as_slice());
        println!("Your Public Key {}", pub_key);
        let keypair = encode(keypair.as_slice());
        println!("Your Key Pair {}", keypair);
    }

    fn get_keypair(keypair_str: &String) -> KeyPair {
        KeyPair::from_slice(&decode(keypair_str)
            .expect("Hex to Byte conversion"))
                .expect("Byte to Keypair conversion")
    }

    pub fn get_wallet(keypair: String) -> Self {
        Self { key_pair: keypair }
    }

    pub fn sign(&mut self, data_hash: &String) -> String {
        encode(Wallet::get_keypair(&self.key_pair).sk.sign(data_hash.as_bytes(), None))
    }

    pub fn get_public_key(&mut self) -> String {
        encode(Wallet::get_keypair(&self.key_pair).pk.as_slice())
    }

    pub fn get_balance<'a>(&mut self, blockchain: &'a mut Blockchain) -> &'a f64 {
        blockchain.get_balance(&self.get_public_key())
    }
}