use std::collections::HashMap;
use serde::{
    Deserialize,
    Serialize
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub accounts: Vec<String>,
    pub balances: HashMap<String, f64>
}

impl Account {
    pub fn new() -> Self {
        Self {
            accounts: vec![],
            balances: HashMap::new(),
        }
    }

    pub fn initialize(&mut self, address: &String) {
        if !self.balances.contains_key(address) {
            self.balances.insert(address.to_string(), 0.00);
            self.accounts.push(address.to_string());
        }
    }

    pub fn transfer(&mut self, from: &String, to: &String, amount: &f64) {
        self.initialize(from);
        self.initialize(to);
        self.increment(to, amount);
        self.decrement(to, amount);
    }

    pub fn increment(&mut self, to: &String, amount: &f64) {
        (*self.balances.get_mut(to).unwrap()) += amount;
    }

    pub fn decrement(&mut self, from: &String, amount: &f64) {
        (*self.balances.get_mut(from).unwrap()) -= amount;
    }

    pub fn get_balance(&mut self, address: &String) -> &f64 {
        self.initialize(address);
        self.balances.get(address).unwrap()
    }
}