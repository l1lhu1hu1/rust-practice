// fn main() {
//     let mut s = String::from("Hello");
//     s = change(s);
//     println!("result: {}", s);
//     // println!("s: {}", s);
// }

// fn change(mut some_string: String) -> String {
//     some_string.push_str(", world");
//     some_string.to_string()
// }

fn main() {
    let mut s = String::from("Hello");
    let result = change(&mut s);
    println!("result: {}", result);
    println!("s: {}", s);
}

fn change(some_string: &mut String) -> String {
    some_string.push_str(", world");
    some_string.to_string()
}
