// 何も値を書かなければ0, 1, 2と順番に整数が入る
enum Number {
    Zero,
    One,
    Two,
}

// Twoには4が入る
// enum Number {
//     Zero,
//     One = 3,
//     Two,
// }

enum Color {
    Red = 0xff0000,
    Blue = 0x00ff00,
    Green = 0x0000ff,
}

fn main() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("two is {}", Number::Two as i32);
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
