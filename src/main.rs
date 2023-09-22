use std::env;
use std::process;

use minigrep::Config;

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
            process::exit(1);
        });


    println!("Searching for {} in file {}",config.query,config.file_path);

    println!("In file {}",config.file_path);

    if let  Err(e) = minigrep::run(config) {
        println!("Application Errore {e}");
        process::exit(1);
    }


}

