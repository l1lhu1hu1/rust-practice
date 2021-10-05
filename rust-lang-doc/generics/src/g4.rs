struct A;
// ()でstructを表す場合は、型のみを書く
// {}でstructを表す場合は、フィールドと型を書く

// Aという型のみ受け付けられる
struct Single(A);

// どんな型でも受け付けられる
struct SingleGen<T>(T);

struct S(A);
struct SGen<T>(T);

// ワンラインで宣言したstructのインスタンスを、作ったときには、
// フィールドにアクセスする時に、tp.0, tp.1のようにしないといけない
struct Tuple(i32, String);
struct Point(i32);

struct Hello<T> {
    ja: T,
    en: T,
}

fn reg_fn(_s: S) {}

// ジェネリックではない
// 明示的な型をパラーメーターとしてi32を与えられているから
// gen_spec_tの直後にもし<A>があれば、ジェネリックな関数になる
fn gen_spec_t(_s: SGen<A>) {}

// ジェネリックではない
// 明示的な型をパラーメーターとしてi32を与えられているから
fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

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

    println!("########################################");

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    // どちらでもオッケー
    generic::<char>(SGen('a'));
    generic(SGen('c'));
}
