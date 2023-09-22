use std::env;
// questa libreria ci permette di passare
// argomenti al programma

use std::fs;
// libreria per lettura/scrittura file

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
    
    let config = Config::new(&args);

    println!("Searching for {} in file {}",config.query,config.file_path);

    println!("In file {}",config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config{ query, file_path}
    }
}