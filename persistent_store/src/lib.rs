use std::path::{Path};
use std::io;
pub mod store;
pub mod persistence;

pub use crate::store::{Store, Value, STORAGE_PATH};
pub enum Command {
    Set,
    Get,
    Delete,
    List,
    Quit,
}


pub fn check_storage_existance()->bool{
    let path = Path::new(STORAGE_PATH);
    return path.exists()
}

pub fn initialize_program()->Result<Store,String>{
    println!("Welcome to the key-value store written in Rust\n");
    let mut empty_store = Store::new();
    let storage_already_exists:bool = check_storage_existance();
    if storage_already_exists {
        match empty_store.read_from_file(){
            Some(read_store)=>{
                empty_store = read_store;
            },
            None=>return Err("Couldn't read the storage file".to_string()),
        }
    }
    return Ok(empty_store)
    
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
            Some(key) => store.delete_value(&key),
            None => println!("Key cannot be empty!"),
        },
        Command::List => {
            store.list_values();
        }
        Command::Quit => {
            store.persist_to_file();
            println!("Bye bye!");
            std::process::exit(0)
        }
    }
}

pub fn insert_key() -> String {
    let mut new_empty_key: String = String::new();
    loop {
        println!("Insert the value of the key\n");
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
