struct FikaBread {
    name: String,
    amount: u8,
    weight: f64,
}

struct AnotherStuct {
    name: String,
}

trait Description {
    fn describe(&self) -> String {
        String::from("this is a default description.")
    }
}

impl Description for FikaBread {
    fn describe(&self) -> String {
        String::from("a tasty fika bread!")
    }
}

// if not implement Description in a stuct, the default describe will be called
impl Description for AnotherStuct {}

fn main() {
    let donuts = FikaBread {
        name: String::from("donuts"),
        amount: 1,
        weight: 50.00,
    };

    let default = AnotherStuct {
        name: String::from("hi"),
    };

    println!("{}", donuts.describe());
    println!("{}", default.describe());
}
