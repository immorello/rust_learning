use std::{collections::HashMap, fs::File};
use std::io::{self, Read, Write};
use serde::{Serialize, Deserialize};
use std::fs;
const STORAGE_PATH: &str = "./Storage/storage.json";
pub enum Command {
    Set,
    Get,
    Delete,
    List,
    Quit,
}
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

    fn serialize(&self)-> String{
        let json_res = serde_json::to_string(&self);
        match json_res {
            Ok(json)=>json,
            Err(_)=>"Could not serialize json".to_string(),
        }
    }

    fn deserialize(&self, store_string:&str)->Option<Store>{
        let my_store = serde_json::from_str(store_string);
        match my_store {
            Ok(store)=>store,
            Err(_)=>None,
        }
    }

    fn persist_to_file(&self){
        let store_string = self.serialize();
        let file = std::fs::File::create(STORAGE_PATH);
        match file {
            Ok(mut new_file)=>{
                let write_result = new_file.write_all(store_string.as_bytes());
                match write_result {
                    Ok(_)=>println!("data persisted to file"),
                    Err(_)=>println!("Error while printing to file"),
                }
            },
            Err(_)=>println!("Could not create the file"),
        }
    }

    fn read_from_file(&self)->Option<Store>{
        let file_to_read = fs::read_to_string(STORAGE_PATH);
        match file_to_read {
            Ok(string_file)=>{
                let store = self.deserialize(&string_file);
                store
            },
            Err(_)=>None,
        }
    }

    fn set_value(&mut self, new_key: String, new_value: Value) {
        self.data.insert(new_key.clone(), new_value);
        println!("Inserted value with key {}", new_key);

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

    fn get_value(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    fn delete_value(&mut self, key: &str) -> Option<Value> {
        self.data.remove(key)
    }

    fn list_values(&self) {
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

pub fn parse_value(input: &str) -> Value {
    if let Ok(num) = input.parse::<i32>() {
        Value::Integer(num)
    } else if let Ok(num) = input.parse::<f64>() {
        Value::Float(num)
    } else if let Ok(bool) = input.parse::<bool>() {
        Value::Boolean(bool)
    } else {
        Value::Text(input.to_string())
    }
}

pub fn parse_option(option_number: i32) -> Result<Command, String> {
    match option_number {
        1 => Ok(Command::Set),
        2 => Ok(Command::Get),
        3 => Ok(Command::Delete),
        4 => Ok(Command::List),
        5 => Ok(Command::Quit),
        _ => Err("Not a valid option!".to_string()),
    }
}

pub fn execute_command(
    store: &mut Store,
    command: Command,
    key: Option<String>,
    value: Option<Value>,
) {
    match command {
        Command::Set => match key {
            Some(key) => match value {
                Some(value) => {
                    store.set_value(key, value);
                }
                None => println!("Value cannot be empty!"),
            },
            None => println!("Key cannot be empty!"),
        },
        Command::Get => match key {
            Some(key) => match store.get_value(&key) {
                Some(value) => match value {
                    Value::Integer(num) => println!("Value for item with key {}: {}\n", key, num),
                    Value::Float(num) => println!("Value for item with key {}: {}\n", key, num),
                    Value::Text(txt) => println!("Value for item with key {}: {}\n", key, txt),
                    Value::Boolean(bool) => println!("Value for item with key {}: {}\n", key, bool),
                },
                None => println!("Couldn't find any value with key {}", key),
            },
            None => println!("Key cannot be empty!"),
        },
        Command::Delete => match key {
            Some(key) => match store.delete_value(&key) {
                Some(_) => println!("Deleted value with key {}", key),
                None => println!("Could not delete value with key {}", key),
            },
            None => println!("Key cannot be empty!"),
        },
        Command::List => {
            store.list_values();
        }
        Command::Quit => {
            println!("Bye bye!");
            std::process::exit(0)
        }
    }
}

pub fn insert_key() -> String {
    let mut new_empty_key: String = String::new();
    loop {
        println!("Insert the value of the key you want to save\n");
        match io::stdin().read_line(&mut new_empty_key) {
            Ok(_) => break new_empty_key,
            Err(_) => {
                println!("Error while reading key!");
                continue;
            }
        }
    }
}

pub fn insert_value() -> Value {
    let mut new_value: String = String::new();
    loop {
        println!("Insert the value to associate to the key\n");
        match io::stdin().read_line(&mut new_value) {
            Ok(_) => break parse_value(&new_value),
            Err(_) => {
                println!("Error while reading value");
                continue;
            }
        }
    }
}
