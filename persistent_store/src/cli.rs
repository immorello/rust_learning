use crate::app::{Command};
use crate::app;
use crate::errors::AppError;
use crate::store::Value;
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
        app::choose_command(&option, &mut new_store);
    }
}

pub fn print_error(error: AppError) {
    match error {
        AppError::IoError(msg) => println!("File error: {}", msg),
        AppError::DecodeError(msg) => println!("Decode error: {}", msg),
        AppError::InvalidCommand => println!("Not a valid command"),
        AppError::InvalidInput(msg) => println!("Invalid input: {}", msg),
        AppError::KeyNotFound(key) => println!("Key not found: {}", key),
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
