use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const STORAGE_PATH: &str = "./Storage/storage.json";

#[derive(Serialize, Deserialize)]
pub enum Value {
    Integer(i32),
    Float(f64),
    Text(String),
    Boolean(bool),
}
#[derive(Serialize, Deserialize)]
pub struct Store {
    data: HashMap<String, Value>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            data: HashMap::new(),
        }
    }

    pub fn set_value(&mut self, new_key: String, new_value: Value) {
        self.data.insert(new_key.clone(), new_value);
        println!("Inserted value with key {}", new_key);
        match self.save_to_file() {
            Ok(_) => {}
            Err(error) => println!("{}", error.to_string()),
        };
        match self.data.get(&new_key) {
            Some(new_value) => match new_value {
                Value::Integer(num) => println!("Value for item with key {}: {}\n", new_key, num),
                Value::Float(num) => println!("Value for item with key {}: {}\n", new_key, num),
                Value::Text(txt) => println!("Value for item with key {}: {}\n", new_key, txt),
                Value::Boolean(bool) => println!("Value for item with key {}: {}\n", new_key, bool),
            },
            None => println!("Recently inserted value not found!"),
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    pub fn delete_value(&mut self, key: &str) {
        match self.data.remove(key) {
            Some(_) => {
                match self.save_to_file() {
                    Ok(_) => {}
                    Err(error) => println!("{}", error.to_string()),
                };
                println!("Deleted value with key {}", key);
            }
            None => {
                println!("Could not delete value with key {}", key);
            }
        }
    }

    pub fn list_values(&self) {
        if self.data.is_empty() {
            println!("Store is empty");
            return;
        }
        println!("Here's the complete list of items in the store:\n");
        for (key, value) in &self.data {
            match value {
                Value::Integer(num) => println!("Value for item with key {}: {}\n", key, num),
                Value::Float(num) => println!("Value for item with key {}: {}\n", key, num),
                Value::Text(txt) => println!("Value for item with key {}: {}\n", key, txt),
                Value::Boolean(bool) => println!("Value for item with key {}: {}\n", key, bool),
            }
        }
    }
}
