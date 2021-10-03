fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result);
}

// xとyの参照のどちらが戻り値なのかわからないのでエラーになる
// 借用チェッカーでこのままだとライフタイムがわからない
// 借用チェッカーが解析を実行できるように、参照間の関係を定義する必要がある
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
