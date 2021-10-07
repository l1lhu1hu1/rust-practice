#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civillian,
    Soldier,
}

fn main() {
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    // let status = Status::Poor;
    let status = Poor;
    // let work = Work::Civillian;
    let work = Civillian;

    match status {
        Rich => println!("The rich"),
        Poor => println!("The poor"),
    }

    match work {
        Civillian => println!("Civilians hello"),
        Soldier => println!("Soldiers hello"),
    }
}
