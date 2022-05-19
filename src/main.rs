use std::fs;
use std::env;
use colored::*;

fn main() {
    let arguments = Config::new(&env::args().collect::<Vec<String>>());
    println!("Query= \"{}\" \nFile=\"{}\"", arguments.query, arguments.file);
    let file_content = fs::read_to_string(arguments.file).expect("Something went wrong reading the file");
    let indices = file_content.match_indices(&arguments.query);
    let mut aux_index = 0;
    for elem in indices {
        print!("{}",&file_content[aux_index..elem.0]);
        print!("{}",elem.1.bold().red());
        aux_index = elem.0+elem.1.len();
    }
    print!("{}",&file_content[aux_index..]);
}

struct Config {
    query: String,
    file: String
}


impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file = args[2].clone();

        Ok(Config { query, file })
    }
}