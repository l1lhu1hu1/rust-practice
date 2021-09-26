fn main() {
    // stackにstoreされるdataとheapにstoreされるdataではsemanticsが違う
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // errorになる. takes_ownershipにsは渡されている

    let x = 5;
    makes_copy(x);
    println!("{}", x); // errorにならない. makes_copyでxのコピーが渡されている
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
