use std::{env, process};

use cli_utilities::Runnable;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config  = cli_utilities::Config::new(&args);
    match config{
        Ok(val) => {
            if let Err(e) = val.run(){
                println!("{}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }

}
