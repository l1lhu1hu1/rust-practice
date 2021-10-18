enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::*;

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // aはmove済で使おうとしても存在しないので、エラーになる
    let c = Cons(4, Box::new(a));
}
