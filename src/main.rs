use std::fs;
use std::env;
use colored::*;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let query = &arguments[1];
    let file = &arguments[2];
    println!("Query= \"{}\" \nFile=\"{}\"", query, file);
    let file_content = fs::read_to_string(file).expect("Something went wrong reading the file");
    let indices = file_content.match_indices(query);
    let mut aux_index = 0;
    for elem in indices {
        print!("{}",&file_content[aux_index..elem.0]);
        print!("{}",elem.1.bold().red());
        aux_index = elem.0+elem.1.len();
    }
    print!("{}",&file_content[aux_index..]);
}
