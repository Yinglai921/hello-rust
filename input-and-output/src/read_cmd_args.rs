use std::env;

pub fn read_input() {
    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {} ", index, argument);
    }
}

pub fn read_nth_input(index: usize) -> std::string::String {
    if env::args().len() <= 1 {
        panic!("length must be larger than 2")
    } else {
        let arg = env::args().nth(index).unwrap();
        arg
    }
}
