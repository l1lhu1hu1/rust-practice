fn main() {
    let z;
    let x = "foo".to_string();
    {
        let y = "bar".to_string();
        // yと引数の1つ目のライフタイムは同じで
        // printlnがある行まで生きられないのでエラーになる
        // 参照元の値よりも参照のライフタイムが長生きするのは許されない
        // z = some_method(&y, &x);

        z = some_method(&x, &y);

        // 以下の場合だと、戻り値のライフタイムはyのライフタイムと同じになる
        // z = hello_method(&x, &y);
    }
    println!("{}", z);
}

// arg2に関してwarningが出るが、コンパイルはできる
// arg1のライフタイムが第一引数のxと同じであることになる
// xと同じライフタイムなので、some_methodの結果はprintlnがある行まで生き続ける
fn some_method<'a>(arg1: &'a String, arg2: &String) -> &'a String {
    arg1
}

// 戻り値はライフタイムが短い方のライフタイムと同じになる
fn hello_method<'a>(arg1: &'a String, arg2: &'a String) -> &'a String {
    arg1
}
