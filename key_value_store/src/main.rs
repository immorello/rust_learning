use core::error;
use std::io;
use key_value_store::Command;
use key_value_store::Store;
use key_value_store::execute_command;
use key_value_store::parse_option;

fn main() {
    println!("Welcome to the key-value store written in Rust\n");
    println!("Choose the command:\n");
    println!("1:set\n2:get\n3:delete\n4:list\n5:quit\n");
    let new_store = Store::new();
    let option: Command = loop {
        let mut option_string:String = String::new();
        match io::stdin().read_line(&mut option_string){
            Ok(_)=>{},
            Err(_)=>{
                println!("Error while reading the character\n");
                continue;
            },
        };
        let option_number:i32 = match option_string.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{
                println!("Not a valid number!");
                continue;
            }
        };
        match parse_option(option_number){
            Ok(option)=>break option,
            Err(error)=>{
                println!("{}",error);
                continue;
            }
        }
        
    }

    match option {
        Command::Set => {
            let mut new_empty_key: String = String::new();
            let new_key = loop{
                println!("Insert the value of the key you want to save\n");
                match io::stdin().read_line(&mut new_empty_key){
                Ok(key)=>{break new_empty_key},
                Err(_)=>{
                    println!("Error while reading key!");
                    continue;
                },
                }
                
            };
            execute_command(mut &new_store, Command::Set, key, value)},
        }
    }

}
