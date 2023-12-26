use crate::Runnable;
use std::{
    error::Error,
    fs::{self, ReadDir},
    path::Path,
};

pub struct Config {
    pub path: String,
}

impl Runnable for Config {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let entries: ReadDir = Config::list_path(&self.path)?;
        for entry in entries {
            let some_entry = entry?;
            let entry_path = some_entry.path();
            if entry_path.is_dir() {
                println!("Directory: {}", entry_path.display());
            } else {
                println!("File: {}", entry_path.display());
            }
        }
        Ok(())
    }
}

impl Config {
    pub fn list_path<P: AsRef<Path>>(path: &P) -> Result<ReadDir, impl Error> {
        fs::read_dir(path)
    }
}
