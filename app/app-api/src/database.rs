use crate::models::error::Error;
use parse::{line, sentence, word};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{fs, path::Path};
use trie::Trie;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub tries: Vec<Trie>, // Arc<Mutex<Vec<Trie>>>
}

impl Database {
    pub fn new() -> Self {
        Self {
            tries: Vec::new(), // Arc<Mutex<Vec<Trie>>>::new();
        }
    }

    pub fn get_tries(&self) -> Vec<Trie> {
        self.tries.to_vec()
    }

    // Serialize the Database to a JSON string
    fn to_json_string(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|_err| Error::InternalError)
    }

    pub fn learn_from_resources(&mut self) {
        // Read the contents of the directory
        if let Ok(entries) = fs::read_dir("./resources") {
            // Iterate through the directory entries
            for entry in entries.flatten() {
                // Get the file name
                let filepath = entry.path();
                if filepath.extension().map_or(false, |ext| ext == "txt") {
                    // Process the file name as needed
                    self.create_trie_from_file(&filepath, "Line");
                }
            }
        } else {
            eprintln!("Error reading directory contents.");
        }
        self.save_to_database_json().unwrap_or_else(|e| {
            eprintln!("Error saving to JSON: {:?}", e);
        });
    }

    // Save the serialized Database to a file
    fn save_to_database_json(&self) -> Result<()> {
        let json_string = self.to_json_string().map_err(|_err| Error::InternalError);
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true) // Create the file if it doesn't exist
            .truncate(true) // Truncate the file if it already exists
            .open("./database.json")
            .map_err(|_err| Error::InternalError);
        file?.write_all(json_string?.as_bytes())?;
        Ok(())
    }

    // fn create_trie_from_vec(&mut self, string_list: Vec<String>) {
    //     self.tries.push(Trie::new(string_list));
    // }

    fn create_trie_from_file(&mut self, filepath: &Path, parse_mode: &str) {
        let filepath = filepath.to_string_lossy().to_string();
        //defaults to word if invalid parse mode is entered
        self.tries.push(Trie::new(match parse_mode {
            "Line" => line(&filepath).unwrap(),
            "Word" => word(&filepath).unwrap(),
            "Sentence" => sentence(&filepath).unwrap(),
            &_ => word(&filepath).unwrap(),
        }));
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
