struct S;
struct GenericVal<T>(T);

impl GenericVal<f32> {}
impl GenericVal<S> {}
// ジェネリック型のまま扱うには<T>が先に来る必要がある
// でなければ上のコードのように型であるとTという名前の型と判断されてしまう
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl<T> GenVal<T>は GenValがジェネリックじゃないとダメ?
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{}, {}", x.value(), y.value());
}
