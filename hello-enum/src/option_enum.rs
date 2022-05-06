pub fn get_number() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5); // get item out of scope
    let number = number.unwrap_or(&0) + 1; // unwrap_or => Option type to a ref of value
    println!("number is {:?}", number);
}

pub fn get_number_2() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(1);
    let number = match number {
        Some(number) => number + 1,
        None => 0,
    };
    println!("number is {:?}", number);
}

pub fn if_let() {
    let number = Some(13);

    /*
      match number {
          Some(13) => println!("thirteen"),
          _ => ()
      }
    */
    if let Some(13) = number {
        println!("thirteen");
    }
}
