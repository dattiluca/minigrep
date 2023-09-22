use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config,&'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments");
            //Our error values will always be string literals that have the 'static lifetime.
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{ query, file_path})
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>>{

/*
    Box<dyn Error> means the function will return a type that implements the Error trait, 
    but we don’t have to specify what particular type the return value will be. 
    This gives us flexibility to return error values that may be of different types in different error cases.
    The dyn keyword is short for “dynamic.”
*/


    let contents = fs::read_to_string(config.file_path)?;

    /*
     ? will return the error value from the current function for the caller to handle.
    */

    println!("With text:\n{contents}");

    Ok(())

}