#[derive(Debug)]

// generic struct definition
struct Rectangle<T, U> {
    height: T,
    width: U,
}

// generic method definition
impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &U {
        &self.width
    }
}

impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * &self.width + 2 * &self.height
    }
}

// generic function definition
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let rec = Rectangle {
        height: 1u8,
        width: 3u8, // if 3u16, get_perimeter will return error
    };

    println!("rect is {:?}", rec);
    println!("width is {}", rec.get_width());
    println!("perimeter is {}", rec.get_perimeter());

    println!("biggest is {}", get_biggest(1, 2))
}
