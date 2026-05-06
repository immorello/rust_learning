use crate::Store;
use crate::STORAGE_PATH;
use std::fs;
use std::io::Write;

impl Store {
    pub fn to_json(&self) -> Result<String, String> {
        let json_res = serde_json::to_string(&self);
        match json_res {
            Ok(json) => Ok(json),
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn from_json(&self, store_string: &str) -> Result<Store, String> {
        let my_store = serde_json::from_str(store_string);
        match my_store {
            Ok(store) => Ok(store),
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn save_to_file(&self) -> Result<(), String> {
        let store_string = match self.to_json() {
            Ok(string) => string,
            Err(error) => return Err(error),
        };
        let file = std::fs::File::create(STORAGE_PATH);
        match file {
            Ok(mut new_file) => {
                let write_result = new_file.write_all(store_string.as_bytes());
                match write_result {
                    Ok(_) => Ok(println!("data persisted to file")),
                    Err(error) => Err(error.to_string()),
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn load_from_file(&self) -> Result<Store, String> {
        let file_to_read = fs::read_to_string(STORAGE_PATH);
        match file_to_read {
            Ok(string_file) => self.from_json(&string_file),
            Err(error) => Err(error.to_string()),
        }
    }
}
