use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value >50 {
            panic!("Guess value must be between 1 and 50, got {}", value)
        }
        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
fn main() {
    let mut val = String::new();
    io::stdin().read_line(&mut val).expect("Failed to read line");
    let vv: u32 = match val.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("failed"),
    };
    let g = Guess::new(vv);
    let v = g.value();
    println!("v: {}", v);
}
