use std::env;
// questa libreria ci permette di passare
// argomenti al programma


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
    
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {} in file {}",query,file_path);
}
