use std::fs::File;
fn main() {
    // formatter, linterによって出力されるエラーとは異なる
    // 以下のコードが動くかどうかは実行時までわからない
    // let v = vec![1, 2, 3];
    // v[500];

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
