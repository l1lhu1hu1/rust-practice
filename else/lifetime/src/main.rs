fn main() {
    let string1 = String::from("hello");
    // let string2 = String::from("bye");

    // let result = longest(&string1, &string2);
    let result = longest(&string1);
    println!("longest string: {}", result);

    // println!("{}, {}", string1, string2);
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 1つだけ参照を渡す場合はライフタイムパラメーターが不要
// 引数が参照型で1つだけなら、変数のライフタイムはわかる
// 引数が参照型で2つ以上になると、それぞれの変数のライフタイムがわからなくなる
// 引数が1つの場合だったらコンパイラがライフタイムを推論できる

fn longest(x: &str) -> &str {
    let y = "sayonara";
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
