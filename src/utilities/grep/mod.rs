use crate::Runnable;
use std::{fs, error::Error};


pub struct Config{
    pub filename : String,
    pub query: String,
}

impl Runnable for Config {
    fn run(&self) -> Result<(), Box<dyn Error>>{
        let content : String = fs::read_to_string(self.filename.as_str())?;
        let results = self.search(&content);
        for matching in results{
            println!("{}", matching);
        }
        Ok(())
    }
}

impl Config{
    fn search(&self, content : &String) -> Vec<String>{
        println!("Performing search");
        let mut result:Vec<String> = Vec::new();
        for line in content.lines(){
            if line.contains(self.query.as_str()){
                result.push(String::from(line));
            }
        }
        result
    }
}