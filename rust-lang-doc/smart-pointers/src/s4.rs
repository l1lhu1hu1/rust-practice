enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::*;
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // Rc::cloneが呼ばれるたびに参照カウントが増える
    // 参照カウントが0にならない限りaは片付けられない
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    // a.cloneはRc::clone(&a)と同じ
    let c = Cons(4, a.clone());
    println!("count after creating c = {}", Rc::strong_count(&a));
}
