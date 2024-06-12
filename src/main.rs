use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // passing the ownership of the iterator to the build function
    let config = Config::build(env::args()).unwrap_or_else(|err| { 
        eprintln!("Problem parsing arguments: {err}"); 
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In the file: {}", config.file_path);

    if let Err(e) = minigrep::run(config) { // as run does not have a value to unwrap in Ok case.
        eprintln!("File process error: {e}");
        process::exit(1);
    }
    //dbg!(args);
}




