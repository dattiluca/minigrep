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
    
    let config = parse_config(&args);

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

fn parse_config(args: &[String]) -> Config {


/*
 The args variable in main is the owner of the argument values and is only
  letting the parse_config function borrow them, which means we’d violate Rust’s 
  borrowing rules if Config tried to take ownership of the values in args.
*/
    let query= args[1].clone();
    let file_path = args[2].clone();
/*
here are a number of ways we could manage the String data; the easiest,
though somewhat inefficient, route is to call the clone method on the values. 
This will make a full copy of the data for the Config instance to own,
which takes more time and memory than storing a reference to the string data
*/
    Config { query, file_path}
}