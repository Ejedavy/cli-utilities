use std::error::Error;

pub trait Runnable {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

pub struct Config{
    pub command_configuration : Box<dyn Runnable>,
}

impl Runnable for Config{
    fn run(&self) -> Result<(), Box<dyn Error>> {
       self.command_configuration.run()
    }
}

impl  Config {
    pub fn new(args: &Vec<String>) -> Result<impl Runnable, &'static str>{
        if args.len() < 4{
            return Err("no enough arguments");
        }
        let command = args[1].clone();
        match command.as_str() {
            "grep" => {
                return Ok(
                    Config{
                        command_configuration: Box::new(crate::utilities::grep::Config{
                                                    filename : args[3].clone(),
                                                    query: args[2].clone()
                                                })
                        }
                    );
                },
            _ => {return Err("unimplemented utility");}
        };
    }
}

pub mod utilities;