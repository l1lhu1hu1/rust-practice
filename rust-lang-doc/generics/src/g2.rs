// イメージ的にはTは引数
// idの型がT. 外部から型を切り替えられる
struct GenericVal<T> {
    id: T,
}

// GenericValのTがi32であるときのみ中の実装は定義される
impl GenericVal<i32> {
    // fooはTがi32であるときだけ定義されている
    fn foo(&self) {
        println!("integer: {}", self.id);
    }
}

// GenericValのTがcharであるときのみ中の実装は定義される
impl GenericVal<&str> {
    // fooはTがcharであるときだけ定義されている
    fn foo(&self) {
        println!("string: {}", self.id);
    }
}

// if you want to write an impl block that applies for all GenericVal<T> types,
// you must first declare a type parameter on the impl itself
// (otherwise, T would try to look up a type named T)

// GenericValのRがどんなものでも中の実装は定義される
// implの直後のRがなければ、GenericValの後のRはジェネリック型ではなく
// Rという名前の型を指定することになってしまう
impl<R> GenericVal<R> {
    // hogeはTがどんな型であっても定義されている
    fn hoge(&self) -> &str {
        "hoge called"
    }
}

// イメージ的には上のコードは以下のようにsomethingで受け取った引数を
// GenericValの引数として与えているような感じ
// fn something(R: type) {
//     GenericVal(R)
// }

fn main() {
    let t = GenericVal { id: 32 };
    let g = GenericVal { id: "hello world" };
    t.foo();
    g.foo();
    let t_res = t.hoge();
    let g_res = t.hoge();
    println!("t_res: {}", t_res);
    println!("g_res: {}", g_res);
}
