use crate::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("Bye!");
    }
}

fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}

fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length {}", list.len());
    println!("{}", list.stringify());
    let l2 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let t2 = type_of(l2);
    println!("type of l2 is {}", t2);
    // let r2 = l2.head();
    // println!("{}", l2.head());

    println!("################################");
    // dropはBox関係なしに呼ばれる
    // ヒープ領域にメモリが確保される
    let p = Box::new(Point { x: 20, y: 30 });
    // スタック領域にメモリが確保される
    let p2 = Point { x: 2000, y: 3000 };
    println!("x: {}, y: {}", p.x, p.y);
    println!("p2.x: {}, p2.y: {}", p2.x, p2.y);
}
