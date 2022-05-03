use std::fs;
// system predefined functions
use std::io::prelude::*;

pub fn write_fika() {
    let mut fika = String::new();
    fika.push_str("carrot cake\n");
    fika.push_str("latte\n");

    fs::write("fika.txt", fika);

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("fika.txt")
        .unwrap();

    file.write(b"\nChocolate");
}
