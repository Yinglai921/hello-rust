use std::mem;

#[derive(Debug, Clone)]

struct FikaBread {
    name: String,
    amount: u8,
    weight: f64,
}

impl FikaBread {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_amount(&mut self, amount: u8) {
        self.amount += amount;
    }

    // associated function
    fn new(name: &str) -> FikaBread {
        FikaBread {
            name: String::from(name),
            amount: 1,
            weight: 50.0,
        }
    }
}

fn main() {
    let mut donuts = FikaBread::new("Donuts");

    let banana_donuts = FikaBread {
        weight: 45.9,
        ..donuts.clone() // name is a string and it will be reassigned by default
    };

    println!("name is {}", donuts.name);

    donuts.amount = 3;
    println!("donuts is {:?}", donuts);
    println!("banana_donuts is {:?}", banana_donuts);

    let donuts_name = donuts.get_name();
    println!("donuts name is {}", donuts_name);

    donuts.add_amount(5);
    println!("donuts amount is {}", donuts.amount);

    // donuts is no longer valid
    // donuts is on the stack, boxed_donuts is on the heap
    let boxed_donuts: Box<FikaBread> = Box::new(donuts);
    println!(
        "boxed_donuts size on stack: {} bytes",
        mem::size_of_val(&boxed_donuts)
    );

    println!(
        "boxed_donuts size on heap: {} bytes",
        mem::size_of_val(&*boxed_donuts) // * dereference
    );

    let unboxed_donuts: FikaBread = *boxed_donuts;
    println!(
        "unboxed_donuts size on stack: {} bytes",
        mem::size_of_val(&unboxed_donuts)
    );
}
