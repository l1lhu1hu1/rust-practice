// 標準ライブラリで提供されているBox<T>型に似たスマートポインタを作る(s1-s2)
// 以下に示す2つのことを確認するために作る
// - スマートポインタがそのままだと参照と同じように振る舞わないことを確認するため
// - どうすれば参照外し演算子が使えるようになるのかを確認するため

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5,x);
    // このままだとdereferenceは使えない
    assert_eq!(5,*y);
}
