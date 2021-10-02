struct GenericVal<T> {
    id: T,
}

// GenericValの型がi32であるときのみ中の実装は定義される
impl GenericVal<i32> {
    // fooはTがi32であるときだけ定義されている
    fn foo(&self) {
        println!("integer: {}", self.id);
    }
}

// GenericValの型がcharであるときのみ中の実装は定義される
impl GenericVal<&str> {
    // fooはTがcharであるときだけ定義されている
    fn foo(&self) {
        println!("string: {}", self.id);
    }
}

// if you want to write an impl block that applies for all GenericVal<T> types,
// you must first declare a type parameter on the impl itself
// (otherwise, T would try to look up a type named T)

// GenericValの型がどんなものでも中の実装は定義される
impl<T> GenericVal<T> {
    // hogeはTがどんな型であっても定義されている
    fn hoge(&self) -> &str {
        "hoge called"
    }
}

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
