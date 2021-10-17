use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            // Someの中にはRefCell<Rc<List>>が入っている
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

fn new_rc(l: List) -> Rc<List> {
    Rc::new(l)
}

fn new_ref_cel(r: Rc<List>) -> RefCell<Rc<List>> {
    RefCell::new(r)
}

fn new_ref_cel_and_rc(l: List) ->  RefCell<Rc<List>> {
    let r = new_rc(l);
    new_ref_cel(r)
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // countのデフォルトは1
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // b作成後のaの参照カウント = {}
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // bの最初の参照カウント = {}
    println!("b initial rc count = {}", Rc::strong_count(&b));
    // bの次の要素 = {:?}
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        // Nilからbの参照カウンタ方式のスマートポインタ(Rc<List>)に変更している
        *link.borrow_mut() = Rc::clone(&b);
    }

    // 2つともcountは2になる
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // aがbを指し、bがaを指すのでずっと参照カウントが0にならない
    // スタックオーバーフローする
    // println!("a next item = {:?}", a.tail());        // aの次の要素 = {:?}
}
