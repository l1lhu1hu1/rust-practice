// Weak<T>が参照する値はドロップされている可能性がある
// Weak<T>が指す値に何かをするには、値がまだ存在するかを確認する必要がある
// Weak<T>のupgradeメソッドによって確認はできる

// それぞれのノードが親ノードと子ノードを知っているような木を作る
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Rc::downgradeメソッドを呼び出すと、Rc<T>のスマートポインタが得られる
    // Rc<T>内部の値への弱い参照を作る
    // downgradeが呼ばれると、Rc<T>のweak_countが1増える
    // leafの親としてbranchへの弱い参照を代入している
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // upgradeメソッドでleafの親への参照を得ようとしている
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
