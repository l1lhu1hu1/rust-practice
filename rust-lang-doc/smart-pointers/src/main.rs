fn main() {
    let x = 5;
    let y = &x;

    let z = Box::new(x);

    assert_eq!(5, x);
    // 参照が指している値にアクセスするために参照外し演算子を使用している
    // Derefトレイトを実装することで参照外し演算子の振る舞いはカスタマイズできる
    assert_eq!(5, *y);
    // 参照と値を比べているため、これだとエラーになる
    // assert_eq!(5, y);
    // yと同様に参照が指している値にアクセスしている
    assert_eq!(5, *z);
}
