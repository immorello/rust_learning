use crate::{errors::AppError};
pub use crate::store::{STORAGE_PATH, Store, Value};
use std::path::Path;

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

pub fn set_value(store:&mut Store, key:String, value:Value)->Result<String,AppError>{
    store.set_value(key, value)
}

pub fn get_value<'a>(store:&'a Store, key:&String)->Result<&'a Value,AppError>{
    match store.get_value(key){
        Some(value)=>Ok(value),
        None=>return Err(AppError::KeyNotFound(format!("Couldn't find any value with key {}", key))),
    }
}

pub fn delete_value(store:&mut Store, key:&String)->Result<String, AppError>{
    store.delete_value(key)
}

pub fn list_values(store:&Store)->Result<String, AppError>{
    store.list_values()
}

pub fn quit(store:&Store)->Result<String,AppError>{
    match store.save_to_file(){
        Ok(result)=>Ok(result),
        Err(error)=>Err(AppError::InternalError(error))
    }
}
