fn main() {
    let mut important_data = "Hello World".to_string();
    // 所有権を渡した場合
    important_data = calc_data(important_data);
    println!("{}", important_data);
    // 所有権を渡さないで代わりに参照を渡す場合
    calc_data2(&important_data);
    println!("{}", important_data);
}

// 所有権を渡した場合
fn calc_data(data: String) -> String {
    println!("{}", data);
    // 最後に所有権を返す処理が必要
    data
}

// 所有権を渡さないで代わりに参照を渡す場合
fn calc_data2(data: &String) {
    println!("Reference {}", data);
}
