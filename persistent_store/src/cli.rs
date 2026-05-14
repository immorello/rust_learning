use crate::app::{Command};
use crate::app;
use crate::errors::AppError;
use crate::store::{Store,Value};

use std::io;

pub fn run() {
    println!("Welcome to the key-value store written in Rust\n");
    let mut new_store = match app::start_store() {
        Ok(store) => store,
        Err(err) => {
            print_error(err);
            return;
        }
    };
    loop {
        print_menu();

        let option = loop {
            match ask_command(){
                Ok(command)=>break command,
                Err(error)=>print_error(error),
            };
        };
        choose_command(&option, &mut new_store);
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
                    match app::set_value(store, key, value){
                        Ok(ok_message)=> println!("{}",ok_message),
                        Err(error)=>print_error(error),
                    }
                }
                None => print_error(AppError::InvalidInput("Value cannot be empty!".to_string())),
            },
            None => print_error(AppError::InvalidInput("Key cannot be empty!".to_string())),
        },
        Command::Get => match key {
            Some(key) => match app::get_value(store,&key) {
                Ok(value) => match value {
                    Value::Integer(num) => println!("Value for item with key {}: {}\n", key, num),
                    Value::Float(num) => println!("Value for item with key {}: {}\n", key, num),
                    Value::Text(txt) => println!("Value for item with key {}: {}\n", key, txt),
                    Value::Boolean(bool) => println!("Value for item with key {}: {}\n", key, bool),
                },
                Err(error) => print_error(error),
            },
            None => print_error(AppError::InvalidInput("Key cannot be empty!".to_string())),
        },
        Command::Delete => match key {
            Some(key) => {
                match app::delete_value(store, &key){
                    Ok(result)=>println!("{}",result),
                    Err(error)=>print_error(error),
                }
            },
            None => print_error(AppError::InvalidInput("Key cannot be empty!".to_string())),
        },
        Command::List => {
            match app::list_values(store){
                Ok(result)=>println!("{}",result),
                Err(error)=>print_error(error),
            }
        }
        Command::Quit => {
            match app::quit(store){
                Ok(string)=>{
                    println!("{}",string);
                    println!("Bye bye!");
                    std::process::exit(0)
                },
                Err(error)=>{
                    print_error(error);
                    print_error(AppError::InternalError("Could not save to file. Quitting aborted".to_string()));
                }
            }
            
        }
    }
}

pub fn print_error(error: AppError) {
    match error {
        AppError::IoError(msg) => println!("File error: {}", msg),
        AppError::DecodeError(msg) => println!("Decode error: {}", msg),
        AppError::InvalidCommand => println!("Not a valid command"),
        AppError::InvalidInput(msg) => println!("Invalid input: {}", msg),
        AppError::KeyNotFound(key) => println!("Key not found: {}", key),
        AppError::InternalError(error)=>println!("Error with basic commands: {}",error),
    }
}

pub fn print_menu() {
    println!("Choose the command:\n");
    println!("1:set\n2:get\n3:delete\n4:list\n5:quit\n");
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

pub fn ask_command() -> Result<Command, AppError> {
    let mut option_string: String = String::new();
    match io::stdin().read_line(&mut option_string) {
        Ok(_) => &option_string,
        Err(_) => {
            return Err(AppError::InvalidInput("Error while reading the character\n".to_string()));
        }
    };

    let option_number: i32 = match option_string.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return Err(AppError::InvalidInput("Not a valid number!".to_string()));
        }
    };
    match parse_option(option_number) {
        Ok(option) => Ok(option),
        Err(error) => {
            return Err(AppError::InvalidInput(error));
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
