enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    // coinの全てのパターンを網羅できていないとコンパイルエラーになる
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let v = value_in_cents(Coin::Penny);
    println!("{}", v);

    let some_u8_val: u8 = 0;
    match some_u8_val {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        // 一部の値にマッチしたときだけ処理を行ってほしい時
        // 上に列挙した値以外
        _ => println!(),
    }

    // if letを用いた場合には、letの直後の書かれている値と
    // 右辺が同値だった場合のみ処理を行う
    // 以下の場合だと3かそれ以外
    // some_u8_valみたいに複数のパターンにマッチする場合はif letは使わない
    let some_i32_val = 3;
    if let 3 = some_i32_val {
        println!("3");
    } else {
        println!("else");
    }
}
