pub fn read_number() {
    let my_number = 1u8;

    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => {
            println!("{} did not match", my_number);
            "something else"
        }
    };
    println!("result is {}", result);
}
