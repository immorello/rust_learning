use std::collections::HashMap;

pub enum Command {
    Set,
    Get,
    Delete,
    List,
    Quit,
}

enum Value {
    Integer(i32),
    Float(f64),
    Text(String),
    Boolean(bool),
}

pub struct Store {
    data: HashMap<String, Value>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            data: HashMap::new(),
        }
    }

    fn set_value(&mut self, new_key: String, new_value: Value) {
        self.data.insert(new_key.clone(), new_value);
        println!("Inserito valore con chiave {}", new_key);
    }

    fn get_value(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    fn delete_value(&mut self, key: &str) -> Option<Value> {
        self.data.remove(key)
    }

    fn list_values(&self) {
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

pub fn execute_command(mut store: &Store, command: Command, key: Option<String>, value: Option<Value>) {
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
                Some(item) => println!("Deleted value with key {}", key),
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
