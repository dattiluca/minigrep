#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

use std::env;
use std::process;

use minigrep::Config;
use minigrep::search;
use minigrep::search_case_insensitive;

fn main() {
    
    let args: Vec<String> = env::args().collect();

/*
    args() restituisce un Iterator, un oggetto
    che possiamo manipolare tramite un "ciclo"

    collect trasforma l'iterator
    in una struttura tipo vettore, contenente 
    gli effettivi argomenti passati, in successione
*/


// salviamo gli argomenti passati
    
    let config = Config::build(&args)
        .unwrap_or_else(|err| {

        //Using unwrap_or_else allows us to define some custom, non-panic! error handling, instead of a simple panic!
            println!("Problem parsing arguments: {err}");
            process::exit(1); // l'exit code 1 è uno dei modi per comunicare termine programma con problemi
        });



    if let  Err(e) = minigrep::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }


}

