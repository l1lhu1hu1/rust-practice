struct A;
// Aという型のみ受け付けられる
struct Single(A);

// どんな型でも受け付けられる
struct SingleGen<T>(T);

// ワンラインで宣言したstructのインスタンスを、作ったときには、
// フィールドにアクセスする時に、tp.0, tp.1のようにしないといけない
struct Tuple(u32, String);
struct Point(i32);

struct Hello<T> {
    ja: T,
    en: T,
}

fn main() {
    let _s = Single(A);
    let _char: SingleGen<char> = SingleGen('a');
    let _t = SingleGen('a');
    let _i32 = SingleGen(6);
    println!("_i32 is {}", _i32.0);

    let tp = Tuple(32, "hello".to_string());
    println!("first: {}", tp.0);
    println!("second: {}", tp.1);

    let point = Point(333);
    println!("{}", point.0);

    let h = Hello {
        ja: "konnnichiwa",
        en: "hello",
    };

    println!("ja: {}, en: {}", h.ja, h.en);
}
