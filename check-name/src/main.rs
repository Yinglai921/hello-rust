use std::env;
use std::fs;

fn find_name(names: &str, search: &str) -> bool {
    let mut res: bool = false;
    for line in names.lines() {
        if line.trim() == search {
            println!("{},{}", line, search);
            res = true;
        }
    }
    res
}
// cargo run name_list.txt Alice

fn main() {
    if env::args().len() <= 2 {
        eprintln!("cargo run <file_path> <search_name>");
        std::process::exit(1);
    }
    // get args from cmd
    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    // get contents from a file path
    let names = fs::read_to_string(file_path).unwrap();

    if find_name(&names, &search_name) {
        println!("found name: {}", &search_name);
    } else {
        println!("name not found: {}", &search_name);
    }
}
