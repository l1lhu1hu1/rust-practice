#[async_std::main]
async fn main() {
    hello().await;
}

// mutを付ける位置がよくわからない
// stringの場合だと string_value: mut String みたいに型の前につく
async fn bye(mut val: i32) -> i32 {
    println!("bye: {}", val);
    val += 1;
    val
}

async fn hello() -> i32 {
    let mut val = 31;
    val = bye(val).await;
    println!("Hello: {}", val);
    val
}
