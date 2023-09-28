use std::error::Error;
use std::{fs, result};
use std::env;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args:  impl Iterator<Item = String>,) -> Result<Config,&'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file_path string"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
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

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");   
    }
    
    Ok(())

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { 

    /*
    Il testo ritornato è una parte di contents. Il lifetime di contents deve essere lungo quanto basta
    per essere usato come valore di ritorno. 
     */
    /*

    OLD IMPLEMENTATION:

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            results.push(line);
        } 
    }

    return results;
    */

    contents
        .lines()
        .filter(|line| line.contains(query))    
        .collect()

}

pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {

    let query = query.to_lowercase(); // to_lowercase trasforma query in una String

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){ // contains vuole una &str. Per questo passiamo &query e non query
            results.push(line);
        } 
    }
    
    results
}