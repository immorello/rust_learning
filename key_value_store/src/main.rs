use key_value_store::Command;
use key_value_store::Store;
use key_value_store::Value;
use key_value_store::execute_command;
use key_value_store::insert_key;
use key_value_store::insert_value;
use key_value_store::parse_option;
use std::io;

fn main() {
    println!("Welcome to the key-value store written in Rust\n");
    let mut new_store = Store::new();
    loop {
        println!("Choose the command:\n");
        println!("1:set\n2:get\n3:delete\n4:list\n5:quit\n");
        
        let option: Command = loop {
            let mut option_string: String = String::new();
            match io::stdin().read_line(&mut option_string) {
                Ok(_) => {}
                Err(_) => {
                    println!("Error while reading the character\n");
                    continue;
                }
            };
            let option_number: i32 = match option_string.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Not a valid number!");
                    continue;
                }
            };
            match parse_option(option_number) {
                Ok(option) => break option,
                Err(error) => {
                    println!("{}", error);
                    continue;
                }
            }
        };

        match option {
            Command::Set => {
                let new_key: String = insert_key();
                println!("Inserted key: {}", new_key);
                let new_value: Value = insert_value();
                
                execute_command(
                    &mut new_store,
                    Command::Set,
                    Some(String::from(new_key)),
                    Some(Value::from(new_value)),
                );
            }
            Command::Get => {
                let new_key: String = insert_key();
                execute_command(
                    &mut new_store,
                    Command::Get,
                    Some(String::from(new_key)),
                    None,
                );
            }
            Command::Delete => {
                let new_key: String = insert_key();
                execute_command(
                    &mut new_store,
                    Command::Delete,
                    Some(String::from(new_key)),
                    None,
                );
            }
            Command::List => {
                execute_command(&mut new_store, Command::List, None, None);
            }
            Command::Quit => {
                execute_command(&mut new_store, Command::Quit, None, None);
            }
        }
    }
}
