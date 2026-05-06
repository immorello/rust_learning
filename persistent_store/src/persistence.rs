use crate::Store;
use std::io::{Write};
use crate::STORAGE_PATH;
use std::fs;

impl Store {
    pub fn serialize(&self) -> String {
        let json_res = serde_json::to_string(&self);
        match json_res {
            Ok(json) => json,
            Err(_) => "Could not serialize json".to_string(),
        }
    }

    pub fn deserialize(&self, store_string: &str) -> Option<Store> {
        let my_store = serde_json::from_str(store_string);
        match my_store {
            Ok(store) => store,
            Err(_) => None,
        }
    }

    pub fn persist_to_file(&self) {
        let store_string = self.serialize();
        let file = std::fs::File::create(STORAGE_PATH);
        match file {
            Ok(mut new_file) => {
                let write_result = new_file.write_all(store_string.as_bytes());
                match write_result {
                    Ok(_) => println!("data persisted to file"),
                    Err(_) => println!("Error while printing to file"),
                }
            }
            Err(_) => println!("Could not create the file"),
        }
    }

    pub fn read_from_file(&self) -> Option<Store> {
        let file_to_read = fs::read_to_string(STORAGE_PATH);
        match file_to_read {
            Ok(string_file) => {
                let store = self.deserialize(&string_file);
                store
            }
            Err(_) => None,
        }
    }
}
