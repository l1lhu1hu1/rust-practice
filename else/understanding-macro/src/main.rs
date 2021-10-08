// =>の左側をマッチャーと呼ぶ
// 左側の式にマッチしたら右側に展開される(置き換えられるイメージ)
// 左右とも(), {}, []のいずれかで囲む
macro_rules! foo {
    () => {};
}

// name: discriminator(何にマッチさせるか)の形式で表現する
// discriminatorには様々な種類がある
// 式をマッチさせたい場合はexprを使用する
// exprは式を表す. 他にはty(型), stmt(文)などがある
macro_rules! hoge {
    ($x: expr) => {
        println!("{}", $x)
    };
}

// itemは型みたいなもので、trait, fn, struct, enum等様々なものを指定できる
macro_rules! match_item {
    ($i: item) => {
        println!("item: {}", stringify!($i))
    };
}

// これは(value=>式): exprになっているものと考える
// hige!(32)はエラーになり、hige!(value=>32)は通る
// hige!(hello => 32)もエラーになる
// 必ずvalue=>式の形を渡さないといけない
macro_rules! hige {
    (value => $e:expr) => {
        println!("value = {}", $e)
    };
}

// 複数マッチの条件を指定できる
// x=>式が来た場合には最初のコードへと展開される
// y=>式が来た場合には最後のコードへと展開される
macro_rules! foge {
    (x => $e:expr) => {
        println!("y = {}", $e)
    };
    (y => $e:expr) => {
        println!("x = {}", $e)
    };
}

fn main() {
    // 何も起こらない
    foo!();
    hoge!(321);
    hoge!(321 == 321);
    match_item!(
        fn dosomething() {
            println!("hello");
        }
    );

    match_item!(
        mod hoge {}
    );

    match_item!(
        struct Hoge {
            value: i32,
            name: String,
        }
    );

    hige!(value => 32);
    foge!(x => 32);
    foge!(y => 55);
}
