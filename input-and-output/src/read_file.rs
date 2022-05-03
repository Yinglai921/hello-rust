use std::fs;
// use std::path;

pub fn read_fika() {
    let fika = fs::read_to_string("fika.txt").unwrap();
    println!("fika is {}", fika);

    for line in fika.lines() {
        println!("line is {}", line);
    }

    let bytes_content = fs::read("fika.txt").unwrap();
    println!("bytes fika is {:?}", bytes_content);
}
