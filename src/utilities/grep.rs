use crate::Runnable;
use std::{error::Error, fs};

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Runnable for Config {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let content: String = fs::read_to_string(self.filename.as_str())?;
        let results = self.search(&content);
        for matching in results {
            println!("{}", matching);
        }
        Ok(())
    }
}

impl Config {
    fn search(&self, content: &String) -> Vec<String> {
        println!("Performing search");
        let mut result: Vec<String> = Vec::new();
        for line in content.lines() {
            if line
                .to_lowercase()
                .contains(self.query.to_lowercase().as_str())
            {
                result.push(String::from(line));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    #[test]
    fn test_search() {
        let filename = String::new();
        let query = String::from("aim");
        let config = Config {
            filename: filename,
            query: query,
        };
        let content = String::from("This is a test content.\nThe aim of this content is to make sure that we are happy.\nCome and enjoy the content");
        let results = config.search(&content);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_case_insensitive_search() {
        let filename = String::new();
        let query = String::from("the");
        let config = Config {
            filename: filename,
            query: query,
        };
        let content = String::from("This is a test content.\nThe aim of this content is to make sure that we are happy.\nCome and enjoy the content");
        let results = config.search(&content);
        assert_eq!(results.len(), 2);
    }
}
