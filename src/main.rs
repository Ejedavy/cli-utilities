use cli_utilities::{Config, Runnable};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    match config {
        Ok(utility) => {
            if let Err(e) = utility.run() {
                println!("{}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}
