// mainのsの時点ではsはimmutableだったけど、
// strとしてownershipがadd_sayoにうつってからはmutable
fn add_sayo(mut str: String) {
    str = str + "sayo";
    println!("{}", str);
}

fn check_muta() {
    let mut m = String::from("mutata");
    let s1 = &m;
    println!("{}", s1);
    let s2 = &mut m;
    // immutable refの変数のスコープ(以降の行で出てこない)が終わってからしか
    // mutable refは存在できない
    // 以下のコードがあるとs2への代入部分でエラーになる
    // println!("{}", s1);
    s2.push_str("jaja");
    println!("{}", s2);
}

// 破壊的変更
fn fn_aaa(s: &mut String) {
    s.push_str("aaa");
}

fn fn_bbb(s: &String) {
    println!("{}, {}", s, "bbb");
}

fn check_fn_ref() {
    let mut m = String::from("check");
    let t = &m;
    fn_bbb(t);
    fn_aaa(&mut m);
    println!("{}", m);
    // immutable refの変数のスコープ(以降の行で出てこない)が終わってからしか
    // mutable refは存在できない
    // 以下のコードがあるとfn_aaaを呼び出すところでエラーになる
    // println!("{}", t);
}

fn main() {
    let s = String::from("Hello");
    add_sayo(s);
    check_muta();
    check_fn_ref();
}
