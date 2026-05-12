use crate::errors::AppError;
pub use crate::store::{STORAGE_PATH, Store, Value};
use std::path::Path;
use crate::cli::{insert_key,insert_value};

pub enum Command {
    Set,
    Get,
    Delete,
    List,
    Quit,
}


pub fn check_storage_existance() -> bool {
    let path = Path::new(STORAGE_PATH);
    return path.exists();
}

pub fn start_store() -> Result<Store, AppError> {
    let mut empty_store = Store::new();
    let storage_already_exists: bool = check_storage_existance();
    if storage_already_exists {
        match empty_store.load_from_file() {
            Ok(read_store) => {
                empty_store = read_store;
            }
            Err(error) => return Err(error),
        }
    }
    return Ok(empty_store);
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
            match store.save_to_file() {
                Ok(_) => {}
                Err(error) => println!("{}", error.to_string()),
            };
            println!("Bye bye!");
            std::process::exit(0)
        }
    }
}

pub fn choose_command(option: &Command, new_store: &mut Store){
    match option {
            Command::Set => {
                let new_key: String = insert_key();
                println!("Inserted key: {}", new_key);
                let new_value: Value = insert_value();

                execute_command(
                    new_store,
                    Command::Set,
                    Some(String::from(new_key)),
                    Some(Value::from(new_value)),
                );
            }
            Command::Get => {
                let new_key: String = insert_key();
                execute_command(
                    new_store,
                    Command::Get,
                    Some(String::from(new_key)),
                    None,
                );
            }
            Command::Delete => {
                let new_key: String = insert_key();
                execute_command(
                    new_store,
                    Command::Delete,
                    Some(String::from(new_key)),
                    None,
                );
            }
            Command::List => {
                execute_command(new_store, Command::List, None, None);
            }
            Command::Quit => {
                execute_command(new_store, Command::Quit, None, None);
            }
        }
}
