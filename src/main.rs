use std::env;
use std::process;

use minigrep::Config;
// std::env::args returns an iterator of the command line arguments 
// iterators: iterators produce a series of values, and we can call the collect method on an iterator to turn it 
// into a collection, such as a vector, that contains all the elements the iterator produces.
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| { // if the result is Ok it returns the inner value of Ok, that it's wrapping
        eprintln!("Problem parsing arguments: {err}"); // if is err it calls the code in the closure, this anonymous function difined here
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




