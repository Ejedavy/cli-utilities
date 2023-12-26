use crate::utilities::grep;
use crate::utilities::ls;
use std::error::Error;

pub mod utilities;

pub trait Runnable {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

pub struct Config {
    pub command_configuration: Box<dyn Runnable>,
}

impl Runnable for Config {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        self.command_configuration.run()
    }
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<impl Runnable, &'static str> {
        if args.len() < 2 {
            return Err("no enough arguments");
        }
        let command = args[1].clone();
        match command.as_str() {
            "grep" => {
                if args.len() < 4 {
                    Err("no enough arguments")
                } else {
                    Ok(Config {
                        command_configuration: Box::new(grep::Config {
                            filename: args[3].clone(),
                            query: args[2].clone(),
                        }),
                    })
                }
            }
            "ls" => {
                if args.len() < 3 {
                    return Err("no enough arguments");
                }
                Ok(Config {
                    command_configuration: Box::new(ls::Config {
                        path: args[2].clone(),
                    }),
                })
            }
            _ => Err("unimplemented utility"),
        }
    }
}
